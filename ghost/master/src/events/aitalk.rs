use crate::events::talk::anchor::anchor_talks;
use crate::events::talk::randomtalk::{all_random_talks, random_talks};
use crate::events::talk::{register_talk_collection, TalkType};
use crate::system::error::ShioriError;
use crate::system::response::*;
use crate::system::variables::*;
use shiorust::message::{Request, Response};

use super::talk::randomtalk::derivative_talks;
use super::talk::Talk;
use super::webclap::derivative_talk_request_open;

pub(crate) fn on_ai_talk(_req: &Request) -> Result<Response, ShioriError> {
  let if_consume_talk_bias = *get_read(&IDLE_SECONDS) < IDLE_THRESHOLD;
  *get_write(&LAST_RANDOM_TALK_TIME) = *get_read(&GHOST_UP_TIME);

  let talk_lists = TalkType::all().into_iter().map(random_talks);
  if talk_lists.clone().any(|t| t.is_none()) {
    return Err(ShioriError::TalkNotFound);
  };
  let talks = talk_lists.flatten().flatten().collect::<Vec<_>>();
  let len_after_flatten = talks.len();
  let index = if let Some(v) = choose_one(&talks, if_consume_talk_bias) {
    v
  } else {
    let mut res = new_response_nocontent();
    add_error_description(
      &mut res,
      format!("No talk found: , {}", len_after_flatten).as_str(),
    );
    return Ok(res);
  };
  let choosed_talk = talks[index].clone();
  if if_consume_talk_bias {
    // ユーザが見ているときのみトークを消費&トークカウントを加算
    if let Some(talk_type) = choosed_talk.talk_type {
      register_talk_collection(&choosed_talk.id, talk_type)?;
    }
    *get_write(&CUMULATIVE_TALK_COUNT) += 1;
  }

  new_response_with_value_with_translate(
    format!("\\0{}", render_talk(&choosed_talk),),
    TranslateOption::simple_translate(),
  )
}

pub fn render_talk(talk: &Talk) -> String {
  talk.consume()
}

pub(crate) fn on_anchor_select_ex(req: &Request) -> Result<Response, ShioriError> {
  let refs = get_references(req);
  let anchor_type = refs[1]; // AnchorTalk || DerivativeTalk // DerivativeTalkRequest
  let id = refs[2];
  let user_dialog = refs.get(3).unwrap_or(&"").to_string();

  if *get_read(&LAST_ANCHOR_ID) == Some(id.to_string()) {
    return Ok(new_response_nocontent());
  }

  match anchor_type {
    "AnchorTalk" => anchor_talk_dialog(id, &user_dialog),
    "DerivativeTalk" => derivative_talk_dialog(id),
    "DerivativeTalkRequest" => derivative_talk_request_open(id),
    _ => Err(ShioriError::BadRequest),
  }
}

fn derivative_talk_dialog(id: &str) -> Result<Response, ShioriError> {
  match derivative_talks().iter().find(|t| t.id == id) {
    Some(talk) => {
      let mut m = String::from("\\C");
      m += &format!("\\1\\c\\_q{}\\n\\_q", talk.summary);
      m += "\\0\\n\\f[align,center]\\_q─\\w1──\\w1───\\w1─────\\w1────\\w1──\\w1──\\w1─\\w1─\\n";
      m += "\\_w[750]\\_q\\_l[@0,]";
      m += &talk.consume();
      let parent_talk = TalkType::all()
        .iter()
        .map(|t| {
          if let Some(talks) = all_random_talks(*t) {
            talks.iter().find(|t| t.id == talk.parent_id).cloned()
          } else {
            None
          }
        })
        .find(|t| t.is_some())
        .and_then(|t| t);
      if let Some(parent) = parent_talk {
        if let Some(talk_type) = parent.talk_type {
          register_talk_collection(id, talk_type)?;
        }
      }
      new_response_with_value_with_translate(m, TranslateOption::simple_translate())
    }
    None => Ok(new_response_nocontent()),
  }
}

fn anchor_talk_dialog(id: &str, user_dialog: &str) -> Result<Response, ShioriError> {
  let mut m = String::from("\\C");
  m += "\\0\\n\\f[align,center]\\_q─\\w1──\\w1───\\w1─────\\w1────\\w1──\\w1──\\w1─\\w1─\\n\\_w[750]\\_q\\_l[@0,]";
  if !user_dialog.is_empty() {
    m += &format!("\\1『{}』\\_w[500]", user_dialog);
  }
  match anchor_talks(id) {
    Some(t) => {
      *get_write(&LAST_ANCHOR_ID) = Some(id.to_string());
      new_response_with_value_with_translate(m + &t, TranslateOption::simple_translate())
    }
    None => Ok(new_response_nocontent()),
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::events::input::{on_user_input, on_user_input_cancel};
  use crate::events::periodic::on_second_change;
  use crate::events::{on_boot, on_close};
  use crate::system::variables::USER_NAME;
  use shiorust::message::parts::*;
  use shiorust::message::Request;

  fn dummy_request(id: &str) -> Request {
    let mut headers = Headers::new();
    headers.insert_by_header_name(HeaderName::from("ID"), id.to_string());
    Request {
      method: Method::GET,
      version: Version::V20,
      headers,
    }
  }

  fn dummy_request_with_refs(id: &str, refs: &[&str]) -> Request {
    let mut headers = Headers::new();
    headers.insert_by_header_name(HeaderName::from("ID"), id.to_string());
    for (i, r) in refs.iter().enumerate() {
      headers.insert_by_header_name(HeaderName::from(&format!("Reference{}", i)), r.to_string());
    }
    Request {
      method: Method::GET,
      version: Version::V20,
      headers,
    }
  }

  fn value_of(res: &Response) -> Option<&String> {
    res.headers.get_by_header_name(&HeaderName::from("Value"))
  }

  /// 起動と終了まわりの全テスト。
  /// FLAGSとLOAD_STATUSを共有するため1つのテスト関数にまとめて直列実行する。
  #[test]
  fn test_boot() -> Result<(), Box<dyn std::error::Error>> {
    *get_write(&USER_NAME) = "test".to_string(); // 実際はセーブから読まれるか、初回起動の名前入力で設定される
    let req = dummy_request("OnBoot");

    // 1. 初回起動: フラグが立ち、初回トークが返る
    assert!(!get_read(&FLAGS).check(&EventFlag::FirstBoot));
    let res = on_boot(&req)?;
    assert!(get_read(&FLAGS).check(&EventFlag::FirstBoot));
    assert!(*get_read(&WAITING_FIRST_USER_NAME));
    assert!(!value_of(&res).ok_or("Failed to get value")?.is_empty());

    // 2. 2回目以降: 通常の起動トークに切り替わる
    let res = on_boot(&req)?;
    assert!(!value_of(&res).ok_or("Failed to get value")?.is_empty());

    // 3. ロードの不調の知らせは、起動トークと同じレスポンスに載る。
    //    別のレスポンスに積むと本体へ渡らず、ユーザに気づかれない
    *get_write(&LOAD_STATUS) = LoadStatus::PartialSuccess(vec!["user_name".to_string()]);
    let res = on_boot(&req)?;
    assert!(!value_of(&res).ok_or("Failed to get value")?.is_empty());
    assert_eq!(
      res
        .headers
        .get_by_header_name(&HeaderName::from("ErrorLevel")),
      Some(&"error".to_string())
    );
    assert!(res
      .headers
      .get_by_header_name(&HeaderName::from("ErrorDescription"))
      .ok_or("Failed to get ErrorDescription")?
      .contains("user_name"));

    *get_write(&LOAD_STATUS) = LoadStatus::RestoredFromBackup;
    let res = on_boot(&req)?;
    assert!(!value_of(&res).ok_or("Failed to get value")?.is_empty());
    assert_eq!(
      res
        .headers
        .get_by_header_name(&HeaderName::from("ErrorLevel")),
      Some(&"notice".to_string())
    );

    // 4. 問題なくロードできたときは何も添えない
    *get_write(&LOAD_STATUS) = LoadStatus::Success;
    let res = on_boot(&req)?;
    assert!(res
      .headers
      .get_by_header_name(&HeaderName::from("ErrorLevel"))
      .is_none());

    // 5. 名前の入力を待つあいだはランダムトークを出さない
    assert!(*get_read(&WAITING_FIRST_USER_NAME));
    let res = on_second_change(&dummy_request_with_refs(
      "OnSecondChange",
      &["0", "0", "0", "0", "0"],
    ))?;
    assert!(value_of(&res).is_none());

    // 6. 名前を受け取ると初回だけの返しになり、待ちが解ける
    let res = on_user_input(&dummy_request_with_refs(
      "OnUserInput",
      &["user_name", "テスト"],
    ))?;
    assert!(value_of(&res)
      .ok_or("Failed to get value")?
      .contains("しかと控えましタ"));
    assert!(!*get_read(&WAITING_FIRST_USER_NAME));

    // 7. 2回目以降は呼び名の変更として返す
    let res = on_user_input(&dummy_request_with_refs(
      "OnUserInput",
      &["user_name", "テスト2"],
    ))?;
    assert!(value_of(&res)
      .ok_or("Failed to get value")?
      .contains("しかと覚えましタ"));

    // 8. 入力を断られたときも待ちを解く。解かないと以後ずっと黙ったままになる。
    //    呼び名は「お客」にして、{user_name}様 が「お客様」になるようにする
    *get_write(&WAITING_FIRST_USER_NAME) = true;
    let res = on_user_input_cancel(&dummy_request_with_refs(
      "OnUserInputCancel",
      &["user_name", "close"],
    ))?;
    assert!(value_of(&res)
      .ok_or("Failed to get value")?
      .contains("無理には伺いませン"));
    assert!(!*get_read(&WAITING_FIRST_USER_NAME));
    assert_eq!(*get_read(&USER_NAME), "お客");

    // 9. 空のまま確定されたときも断られたのと同じ。呼び名を空で上書きしない
    *get_write(&WAITING_FIRST_USER_NAME) = true;
    *get_write(&USER_NAME) = "test".to_string();
    let res = on_user_input(&dummy_request_with_refs("OnUserInput", &["user_name", " "]))?;
    assert!(value_of(&res)
      .ok_or("Failed to get value")?
      .contains("無理には伺いませン"));
    assert_eq!(*get_read(&USER_NAME), "お客");

    // 10. 終了トークは \- で閉じる。これが無いと本体の終了が中断される
    let res = on_close(&dummy_request("OnClose"))?;
    let value = value_of(&res).ok_or("Failed to get value")?;
    assert!(!value.is_empty());
    assert!(
      value.ends_with("\\-"),
      "終了スクリプトが \\- で終わっていない: {}",
      value
    );

    *get_write(&LOAD_STATUS) = LoadStatus::NotLoaded;
    Ok(())
  }
}
