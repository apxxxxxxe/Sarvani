use crate::system::error::ShioriError;
use crate::system::response::*;
use crate::system::variables::*;
use shiorust::message::{Request, Response};
use std::fmt;
use std::fmt::{Display, Formatter};

pub(crate) enum InputId {
  UserName,
}

impl Display for InputId {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::UserName => write!(f, "user_name"),
    }
  }
}

impl InputId {
  pub fn from_str(s: &str) -> Option<Self> {
    match s {
      "user_name" => Some(Self::UserName),
      _ => None,
    }
  }
}

pub(crate) fn on_user_input(req: &Request) -> Result<Response, ShioriError> {
  let refs = get_references(req);
  let input_id = if let Some(input_id) = InputId::from_str(refs[0]) {
    input_id
  } else {
    error!("Unknown input id: {}", refs[0]);
    return Ok(new_response_nocontent());
  };
  // 空のまま確定されると、get_references が末尾の空の Reference1 を切り詰めて refs が1要素になる
  let text = refs.get(1).unwrap_or(&"").to_string();
  let responser = match input_id {
    InputId::UserName => input_user_name,
  };
  responser(text)
}

/// 名前を教えてもらえなかったときの呼び名。{user_name}様 で「お客様」になる
const DEFAULT_USER_NAME: &str = "お客";

fn input_user_name(text: String) -> Result<Response, ShioriError> {
  // 空のまま確定されたときは、断られたのと同じに扱う
  if text.trim().is_empty() {
    if *get_read(&WAITING_FIRST_USER_NAME) {
      return refuse_user_name();
    }
    // 呼び名の変更から来た場合は、いまの呼び名を残す
    return Ok(new_response_nocontent());
  }

  *get_write(&USER_NAME) = text.clone();

  // 初回起動トークの末尾で尋ねた分だけは専用の返し。以降は呼び名の変更として扱う
  if *get_read(&WAITING_FIRST_USER_NAME) {
    *get_write(&WAITING_FIRST_USER_NAME) = false;
    let m = format!(
      "\
        h1111307{}様！\\n\
        h1111304しかと控えましタ。\\n\\n[half]\
        h1113209名もまた、一つの鍵でスからネ。\\n\
        h1111207お預かりしたものは、けっして粗末にいたしませン。\\n\\n[half]\
        h1113309ところで、お急ぎでなければ。\\n\
        h1142306そのガラス棚の中のもの、\\n\
        ぜんぶワタクシのコレクションでしてネ！\\n\
        h1121207ぜひご覧になっていってくださイ。\\n\
        h1112306解説や、h1112309なんなら単なるお喋りでも歓迎でスよ！\\n\
        h1142504ワタクシ、いつだって話し相手に飢えておりまスから！\\n\
        ",
      text
    );
    return new_response_with_value_with_translate(m, TranslateOption::simple_translate());
  }

  let m = format!(
    "\
      h1111207かしこまりまシタ。\\n\
      h1113206{}様、と。しかと覚えましタ。\
      \\1\\_q(ユーザ名を{}に設定しました)\
      ",
    text, text
  );
  new_response_with_value_with_translate(m, TranslateOption::simple_translate())
}

/// 入力ボックスを閉じられた・時間切れになったとき。
/// 応答しないと、時間切れの場合に "timeout" を入力値とした OnUserInput が来てしまう
pub(crate) fn on_user_input_cancel(req: &Request) -> Result<Response, ShioriError> {
  let refs = get_references(req);
  if !matches!(InputId::from_str(refs[0]), Some(InputId::UserName)) {
    return Ok(new_response_nocontent());
  }

  // 初回起動の名前入力を断られた場合。待ちを解いて先へ進める
  if *get_read(&WAITING_FIRST_USER_NAME) {
    return refuse_user_name();
  }

  Ok(new_response_nocontent())
}

/// 初回起動で名前を教えてもらえなかったとき。
/// 呼び名を「お客」にして、以後は「お客様」と呼ぶ
fn refuse_user_name() -> Result<Response, ShioriError> {
  *get_write(&WAITING_FIRST_USER_NAME) = false;
  *get_write(&USER_NAME) = DEFAULT_USER_NAME.to_string();

  new_response_with_value_with_translate(
    "\
      h1111308おや！\\n\
      h1113207いいえ、結構でスとも。無理には伺いませン。\\n\
      h1111207名を明かさないのも、立派な錠前でスからネ。\\n\
      h1111304では、お客様とお呼びしまス。\\n\\n[half]\
      h1113309ところで、お急ぎでなければ。\\n\
      h1142306そのガラス棚の中のもの、\\n\
      ぜんぶワタクシのコレクションでしてネ！\\n\
      h1121207ぜひご覧になっていってくださイ。\\n\
      h1112306解説や、h1112309なんなら単なるお喋りでも歓迎でスよ！\\n\
      h1142504ワタクシ、いつだって話し相手に飢えておりまスから！\
      "
    .to_string(),
    TranslateOption::simple_translate(),
  )
}

pub(crate) fn on_window_state_restore(_req: &Request) -> Result<Response, ShioriError> {
  // トーク間隔をリセット
  *get_write(&LAST_RANDOM_TALK_TIME) = *get_read(&GHOST_UP_TIME);

  new_response_with_value_with_translate("h1111101".to_string(), TranslateOption::simple_translate())
}
