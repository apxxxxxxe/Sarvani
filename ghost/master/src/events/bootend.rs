use crate::events::first_boot::FIRST_BOOT_TALK;
use crate::system::error::ShioriError;
use crate::system::response::*;
use crate::system::variables::*;
use shiorust::message::{parts::HeaderName, Response, *};

pub(crate) fn on_boot(_req: &Request) -> Result<Response, ShioriError> {
  *get_write(&TOTAL_BOOT_COUNT) += 1;

  // ロード失敗かつバックアップもないなら何もしない
  if *get_read(&LOAD_STATUS) == LoadStatus::FailedNoBackup {
    let mut res = new_response_nocontent();
    add_error_description(
      &mut res,
      "ロードに失敗しました。ゴーストを終了し、ゴーストフォルダ内のsarvani.logの内容とともにバグ報告をお願い致します。",
    );
    return Ok(res);
  }

  // 初回起動
  let mut res = if !get_read(&FLAGS).check(&EventFlag::FirstBoot) {
    get_write(&FLAGS).done(EventFlag::FirstBoot);
    // 初回起動トークは末尾でユーザ名を尋ねる。返答が来るまで他の発話を止める
    *get_write(&WAITING_FIRST_USER_NAME) = true;
    new_response_with_value_with_translate(
      FIRST_BOOT_TALK.to_string(),
      TranslateOption::simple_translate(),
    )?
  } else {
    let greetings = boot_greetings();
    let index = choose_one(&greetings, true).ok_or(ShioriError::ArrayAccessError)?;
    new_response_with_value_with_translate(
      greetings[index].clone(),
      TranslateOption::simple_translate(),
    )?
  };

  // ロードの不調は、これから返す起動トークに添えて知らせる。
  // 別のレスポンスに積むと本体へ渡らないので、resを組み立てた後に付ける
  match *get_read(&LOAD_STATUS) {
    LoadStatus::RestoredFromBackup => add_notice_description(
      &mut res,
      "セーブデータが破損していたため、バックアップから復元しました。",
    ),
    LoadStatus::PartialSuccess(ref failed_fields) => add_error_description(
      &mut res,
      &format!(
        "セーブデータの一部が読み込めなかったため、初期値を使用しました。お手数ですがバグ報告をお願いいたします。詳細: {}",
        failed_fields.join(", ")
      ),
    ),
    _ => {}
  }

  Ok(res)
}

/// 二回目以降の来店の挨拶。毎回目にするので数本から抽選する
fn boot_greetings() -> Vec<String> {
  [
    "h1111307いらっしゃいまセ、{user_name}様！",
    "h1111307いらっしゃいまセ、{user_name}様！",
    "h1111307いらっしゃいまセ、{user_name}様！",
    "\
        h1000000モグモグ……ンー、うまい！\\n\
        ……h1112104ン？h1111408ア！h1121507これは失礼、{user_name}様。\\n\
        h1121204遅めのごはん中でしタ。\
        ",
  ]
  .iter()
  .map(|s| s.to_string())
  .collect()
}

/// ユーザの退店。末尾の \- がないと終了が中断されるので必ず付ける
pub(crate) fn on_close(_req: &Request) -> Result<Response, ShioriError> {
  let talks = close_greetings();
  let index = choose_one(&talks, true).ok_or(ShioriError::ArrayAccessError)?;

  new_response_with_value_with_translate(
    format!("{}\\-", talks[index]),
    TranslateOption::simple_translate(),
  )
}

fn close_greetings() -> Vec<String> {
  [
    "\
      h1111307ありがとうございましタ、お客様！\\n\
      h1113206またのご来店をお待ちしておりまス。\
      ",
    "\
      h1111301おや、もうお帰りで？h1111307ではお気をつけて。\\n\
      h1113304戸締まりはお忘れなく！\
      ",
    "\
      h1111307またどうぞ、{user_name}様！\\n\
      h1113304鍵をなくしたら、いつでもお呼びくださイ。\\n\
      h1111207マ、なくさないのが一番でスがネ。\
      ",
  ]
  .iter()
  .map(|s| s.to_string())
  .collect()
}

pub(crate) fn on_vanish_selecting(_req: &Request) -> Result<Response, ShioriError> {
  Ok(new_response_nocontent())
}

pub(crate) fn on_vanish_selected(_req: &Request) -> Result<Response, ShioriError> {
  Ok(new_response_nocontent())
}

pub(crate) fn on_vanish_cancel(_req: &Request) -> Result<Response, ShioriError> {
  Ok(new_response_nocontent())
}
