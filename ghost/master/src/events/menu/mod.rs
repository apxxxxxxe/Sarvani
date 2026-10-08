use crate::events::input::InputId;
use crate::events::talk::playable_unseen_talks;
use crate::events::talk::randomtalk::{derivative_talks_per_talk_type, random_talks};
use crate::events::TalkType;
use crate::system::error::ShioriError;
use crate::system::response::*;
use crate::system::variables::{get_read, get_write, EventFlag, FLAGS, RANDOM_TALK_INTERVAL, TALK_COLLECTION, USER_NAME};
use crate::{check_error, DERIVATIVE_TALK_REQUESTABLE};
use shiorust::message::{Request, Response};

pub(crate) mod questions;

pub(crate) fn on_menu_exec(_req: &Request) -> Response {
  let current_talk_interval = *get_read(&RANDOM_TALK_INTERVAL);
  let mut selections = Vec::new();

  for i in [1, 3, 5, 7, 10, 0].iter() {
    if current_talk_interval == i * 60 {
      selections.push(format!(
        "\\f[underline,1]{}\\f[underline,0]",
        show_minute(i),
      ));
    } else {
      selections.push(format!(
        "\\q[{},OnTalkIntervalChanged,{}]",
        show_minute(i),
        i * 60,
      ));
    };
  }

  let talk_interval_selector = format!(
    "\
      ◆トーク頻度  【現在 {}】\\n\
      {}\
      ",
    show_minute(&(current_talk_interval / 60)),
    selections.join("  ")
  );

  let buttons = format!(
    "\\_l[0,0]\\f[align,right]{}\\__q[script:\\e]{}\\__q",
    if get_read(&FLAGS).check(&EventFlag::FirstBoot) {
      format!("\\__q[OnConfigMenuExec]{}\\__q ", Icon::Cog)
    } else {
      "".to_string()
    },
    Icon::Cross
  );

  let m = format!("\\_q{}{}", REMOVE_BALLOON_NUM, {
    format!(
      "\
          \\_l[0,1.5em]\
          \\![*]\\q[なにか話して,OnAiTalk]\\n\
          \\![*]\\q[話しかける,OnTalk]\\n\
          \\![*]\\q[トーク統計,OnCheckTalkCollection]\
          \\_l[0,@2em]\
          \\![*]\\q[手紙を書く,OnWebClapOpen]\
          \\_l[0,@2em]\
          {}\
          {}\
          \\0\\_l[0,0]\
          ",
      talk_interval_selector, buttons,
    )
  },);

  new_response_with_value_with_notranslate(m, TranslateOption::balloon_surface_only())
}

pub(crate) fn on_config_menu_exec(_req: &Request) -> Response {
  let m = format!(
    "\
      \\_q\\_l[0,0]\\f[align,right]\\__q[OnMenuExec]{}\\__q \\__q[script:\\e]{}\\__q\
      \\_l[0,1.5em]\
      \\![*]\\q[呼び名を変える,OnChangingUserName]\\n\
      \\![*]\\q[リクエストボタンの表示,OnDerivativeTalkRequestButtonToggled]【現在 {}】\\n\
      ",
    Icon::ArrowLeft,
    Icon::Cross,
    if *get_read(&DERIVATIVE_TALK_REQUESTABLE) {
      "表示"
    } else {
      "非表示"
    },
  );

  new_response_with_value_with_notranslate(m, TranslateOption::balloon_surface_only())
}

fn show_minute(m: &u64) -> String {
  match m {
    0 => "黙る".to_string(),
    _ => format!("{}分", m),
  }
}

pub(crate) fn on_talk_interval_changed(req: &Request) -> Result<Response, ShioriError> {
  let refs = get_references(req);
  let v = check_error!(refs[0].parse::<u64>(), ShioriError::ParseIntError);
  *get_write(&RANDOM_TALK_INTERVAL) = v;

  Ok(on_menu_exec(req))
}

pub(crate) fn on_check_talk_collection(_req: &Request) -> Response {
  let mut lines = Vec::new();
  let mut sum = 0;
  let mut all_sum = 0;
  let talk_collection = get_read(&TALK_COLLECTION);
  lines.push("[トーク統計]\\n".to_string());
  for talk_type in TalkType::all() {
    let empty = std::collections::HashSet::new();
    let seen = talk_collection.get(&talk_type).unwrap_or(&empty);
    // いま required_condition（季節や時間帯）を満たすトークだけを数える。
    // 時間外のものまで数えると、未読が残っているのにいつまでも出てこない表示になる。
    // 派生トークは、自身の条件と親トークの条件の両方を満たすものを数える
    let available = random_talks(talk_type).unwrap_or_default();
    let derivatives = derivative_talks_per_talk_type()
      .get(&talk_type)
      .cloned()
      .unwrap_or_default();
    let available_ids = available.iter().map(|t| t.id.as_str()).chain(
      derivatives
        .iter()
        .filter(|d| d.required_condition.is_none_or(|f| f()) && available.iter().any(|t| t.id == d.parent_id))
        .map(|d| d.id.as_str()),
    );
    let (len, all_len) = available_ids.fold((0, 0), |(len, all_len), id| {
      (len + seen.contains(id) as usize, all_len + 1)
    });
    // 未読が残っていても、いまは時間外のものだけなら再生ボタンは出さない（押しても何も出ないため）
    let anal = if !playable_unseen_talks(talk_type, seen).is_empty() {
      format!(
        "\\n  \\f[height,13]\\q[未読トーク再生,OnCheckUnseenTalks,{}]\\f[default]",
        talk_type as u32
      )
    } else {
      "".to_string()
    };
    lines.push(format!("{}: {}/{}{}", talk_type, len, all_len, anal));
    sum += len;
    all_sum += all_len;
  }

  new_response_with_value_with_notranslate(
    format!(
      "\\_q{}\\n[150]\
        ---\\n[150]\
        TOTAL: {}/{}\\n[200]\
        \\q[戻る,OnMenuExec]",
      lines.join("\\n"),
      sum,
      all_sum
    ),
    TranslateOption::balloon_surface_only(),
  )
}

pub(crate) fn on_changing_user_name(_req: &Request) -> Result<Response, ShioriError> {
  new_response_with_value_with_translate(
    format!(
      "\\_q\\![open,inputbox,{},0]新しい呼び名を入力してください。\\n現在:{}",
      InputId::UserName,
      *get_read(&USER_NAME)
    ),
    TranslateOption::simple_translate(),
  )
}

pub(crate) fn on_derivative_talk_request_button_toggled(req: &Request) -> Response {
  let is_derivative_talks_enabled;
  {
    is_derivative_talks_enabled = *get_read(&DERIVATIVE_TALK_REQUESTABLE);
  }
  *get_write(&DERIVATIVE_TALK_REQUESTABLE) = !is_derivative_talks_enabled;

  on_config_menu_exec(req)
}
