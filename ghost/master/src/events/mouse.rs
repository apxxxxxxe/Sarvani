use crate::check_error;
use crate::events::menu::on_menu_exec;
use crate::system::error::ShioriError;
use crate::system::response::*;
use crate::system::status::Status;
use crate::system::variables::{get_read, get_write, EventFlag, TouchInfo, CHAIN_TALK_STATE, FLAGS, GHOST_UP_TIME, LAST_TOUCH_INFO, TOUCH_INFO, WAITING_FIRST_USER_NAME};
use shiorust::message::{Parser, Request, Response};

#[macro_export]
macro_rules! get_touch_info {
  ($info:expr) => {
    get_write(&TOUCH_INFO)
      .entry($info.to_string())
      .or_insert($crate::system::variables::TouchInfo::new())
  };
}

pub(crate) fn new_mouse_response(req: &Request, info: String) -> Result<Response, ShioriError> {
  let status = Status::from_request(req);

  // 初回起動の名前入力待ちのあいだは、メニューも触り反応も出さない
  if *get_read(&WAITING_FIRST_USER_NAME) {
    return Ok(new_response_nocontent());
  }

  if info != get_read(&LAST_TOUCH_INFO).as_str() {
    if let Some(touch_info) = get_write(&TOUCH_INFO).get_mut(get_read(&LAST_TOUCH_INFO).as_str()) {
      touch_info.reset_if_timeover()?;
    }
    *get_write(&LAST_TOUCH_INFO) = info.clone();
  }

  if !get_read(&FLAGS).check(&EventFlag::FirstBoot) {
    if info.as_str().contains("doubleclick") && !status.talking {
      let dummy_req = check_error!(
        Request::parse(DUMMY_REQUEST),
        ShioriError::ParseRequestError
      );
      return Ok(on_menu_exec(&dummy_req));
    } else {
      return Ok(new_response_nocontent());
    }
  }

  let response = mouse_dialogs(req, info.clone())?;

  // 一括で回数を増やす
  get_write(&TOUCH_INFO)
    .entry(info)
    .or_insert(TouchInfo::new())
    .add();

  Ok(response)
}

fn common_choice_process(dialogs: Vec<String>) -> Result<Response, ShioriError> {
  let index = choose_one(&dialogs, true).ok_or(ShioriError::ArrayAccessError)?;
  new_response_with_value_with_translate(
    format!("{}{}", REMOVE_BALLOON_NUM, dialogs[index].clone()),
    TranslateOption::simple_translate(),
  )
}

pub(crate) fn mouse_dialogs(req: &Request, info: String) -> Result<Response, ShioriError> {
  // チェイントーク発火チェック
  if let Some(chain_response) = check_chain_talk(&info) {
    return chain_response;
  }

  // 通常の触り反応候補。回数では変えず、部位ごとに抽選するだけ
  let common_response = match info.as_str() {
    "0headnade" => Some(common_choice_process(zero_head_nade())),
    "0bustnade" => Some(common_choice_process(zero_bust_response())),
    "0bustdoubleclick" => Some(common_choice_process(zero_bust_response())),
    "0handnade" => Some(common_choice_process(zero_hand_response())),
    "0handdoubleclick" => Some(common_choice_process(zero_hand_response())),
    "0shouldernade" => Some(zero_shoulder_response()),
    "0shoulderdoubleclick" => Some(zero_shoulder_response()),
    _ => None,
  };

  // その他特殊な条件で発生する触り反応
  let other_response = if info.starts_with('0') && info.contains("doubleclick") {
    // 触り反応のない部分をダブルクリックでメニュー
    Some(Ok(on_menu_exec(req)))
  } else {
    None
  };

  common_response
    .or(other_response)
    .unwrap_or_else(|| Ok(new_response_nocontent()))
}

fn zero_head_nade() -> Vec<String> {
  vec![
    "h1111209ンー……h1111306悪くない気分でスね。".to_string(),
    "h1111307ワオ、撫でられるのは好きでスよ！\\nh1111204大人になっても嬉しいものでス。".to_string(),
    "h1111209ワタクシを鍵と錠だけの朴念仁だとお思いでスか？\\nh1113304アナタを好きになるかもしれませんヨ？".to_string(),
  ]
}

fn zero_bust_response() -> Vec<String> {
  vec![
    "h1111209そんなことでワタクシの秘密は暴けませんヨ。".to_string(),
    "h1122201スケベでスねえ。h1142504やり返しまスよ？".to_string(),
    "h1112209ポケットには飴ちゃんが入っていまスよ。\\nh1112204それとも、欲しいのはもっと別のものでスか？".to_string(),
  ]
}

/// 手の当たり判定は腕を上げているとき（胸に手・シー手・ポーズ2）にだけ出る
fn zero_hand_response() -> Vec<String> {
  vec![
    "h1122204……なんだか触り方がいやらしいでスよ。".to_string(),
    "h1122209油、付いちゃいまスよ。".to_string(),
    "h1112204どうしたんでス？むにむにと……。".to_string(),
  ]
}

/// 肩を叩いて呼ばれたとして、一言返してからメニューを開く
fn zero_shoulder_response() -> Result<Response, ShioriError> {
  let dialogs = [
    "h1111201はい、お呼びでスか？",
    "h1111204ここにおりまスよ。",
    "h1111204ご用件を伺いまス。",
  ]
  .iter()
  .map(|s| format!("{}\\n\\![embed,OnMenuExec]", s))
  .collect();
  common_choice_process(dialogs)
}

/// チェイントーク発火チェック。
/// チェイン待機中かつ対象部位が一致し、制限時間内なら発火。
fn check_chain_talk(info: &str) -> Option<Result<Response, ShioriError>> {
  let state = get_read(&CHAIN_TALK_STATE).clone();
  if let Some(chain) = state {
    let now = *get_read(&GHOST_UP_TIME);
    if now <= chain.expires_at && info == chain.target_part {
      // チェイン発火
      *get_write(&CHAIN_TALK_STATE) = None;
      if let Some(cb) = chain.callback {
        cb();
      }
      return Some(new_response_with_value_with_translate(
        format!("{}{}", REMOVE_BALLOON_NUM, chain.chain_text),
        TranslateOption::simple_translate(),
      ));
    }
    // 期限切れならクリア
    if now > chain.expires_at {
      *get_write(&CHAIN_TALK_STATE) = None;
    }
  }
  None
}

/// 触った回数で段階を変える道具。いまの触り反応は回数で変えないので使っていない
#[allow(dead_code)]
pub(crate) fn phased_talks(count: u32, phased_talk_list: Vec<Vec<String>>) -> (Vec<String>, bool) {
  let dialog_lengthes = phased_talk_list
    .iter()
    .map(|x| x.len() as u32)
    .collect::<Vec<u32>>();
  let dialog_cumsum = dialog_lengthes
    .iter()
    .scan(0, |sum, x| {
      *sum += x;
      Some(*sum)
    })
    .collect::<Vec<u32>>();

  for i in 0..dialog_cumsum.len() - 1 {
    if count < dialog_cumsum[i] {
      return (phased_talk_list[i].clone(), false);
    }
  }
  (phased_talk_list.last().unwrap().to_owned(), true)
}

const DUMMY_REQUEST: &str = "GET SHIORI/3.0\r\n\
  Charset: UTF-8\r\n\
  Sender: SSP\r\n\
  SenderType: internal,raise\r\n\
  SecurityLevel: local\r\n\
  Status: choosing,balloon(0=0)\r\n\
  ID: OnFirstBoot\r\n\
  BaseID: OnBoot\r\n\
  Reference0: 1\r\n\r\n";

#[cfg(test)]
mod tests {
  use super::*;
  use crate::system::variables::{ChainTalkState, CHAIN_TALK_STATE, GHOST_UP_TIME};
  use std::sync::atomic::{AtomicBool, Ordering};

  static CALLBACK_CALLED: AtomicBool = AtomicBool::new(false);

  fn reset_chain_state() {
    *get_write(&CHAIN_TALK_STATE) = None;
    CALLBACK_CALLED.store(false, Ordering::SeqCst);
  }

  fn set_test_chain(target: &str, expires_at: u64, with_callback: bool) {
    *get_write(&CHAIN_TALK_STATE) = Some(ChainTalkState {
      target_part: target.to_string(),
      chain_text: "チェインテスト".to_string(),
      expires_at,
      callback: if with_callback {
        Some(|| {
          CALLBACK_CALLED.store(true, Ordering::SeqCst);
        })
      } else {
        None
      },
    });
  }

  /// チェイントーク機構の全テスト。
  /// グローバル変数を使うため1つのテスト関数にまとめて直列実行する。
  #[test]
  fn test_chain_talk_mechanism() {
    // 1. 正しい部位・制限時間内で発火する
    reset_chain_state();
    *get_write(&GHOST_UP_TIME) = 10;
    set_test_chain("0handnade", 30, true);

    let result = check_chain_talk("0handnade");
    assert!(result.is_some(), "チェインが発火するべき");
    assert!(
      CALLBACK_CALLED.load(Ordering::SeqCst),
      "コールバックが呼ばれるべき"
    );
    assert!(
      get_read(&CHAIN_TALK_STATE).is_none(),
      "発火後にチェイン状態がクリアされるべき"
    );

    // 2. 別部位では発火しない
    reset_chain_state();
    *get_write(&GHOST_UP_TIME) = 10;
    set_test_chain("0handnade", 30, true);

    let result = check_chain_talk("0headnade");
    assert!(result.is_none(), "別部位ではチェインが発火しないべき");
    assert!(
      !CALLBACK_CALLED.load(Ordering::SeqCst),
      "コールバックが呼ばれないべき"
    );
    assert!(
      get_read(&CHAIN_TALK_STATE).is_some(),
      "チェイン状態が残っているべき"
    );

    // 3. 制限時間超過で発火しない
    reset_chain_state();
    *get_write(&GHOST_UP_TIME) = 31;
    set_test_chain("0handnade", 30, true);

    let result = check_chain_talk("0handnade");
    assert!(result.is_none(), "期限切れではチェインが発火しないべき");
    assert!(
      !CALLBACK_CALLED.load(Ordering::SeqCst),
      "コールバックが呼ばれないべき"
    );
    assert!(
      get_read(&CHAIN_TALK_STATE).is_none(),
      "期限切れでチェイン状態がクリアされるべき"
    );

    // 4. コールバックなしでも発火する
    reset_chain_state();
    *get_write(&GHOST_UP_TIME) = 10;
    set_test_chain("0shoulderdown", 30, false);

    let result = check_chain_talk("0shoulderdown");
    assert!(
      result.is_some(),
      "コールバックなしでもチェインは発火するべき"
    );
    assert!(
      get_read(&CHAIN_TALK_STATE).is_none(),
      "発火後にチェイン状態がクリアされるべき"
    );

    // 5. チェイン状態がなければNone
    reset_chain_state();
    *get_write(&GHOST_UP_TIME) = 10;
    let result = check_chain_talk("0handnade");
    assert!(result.is_none(), "チェイン状態がなければNoneを返すべき");
  }
}
