use crate::events::aitalk::on_ai_talk;
use crate::system::error::ShioriError;
use crate::system::response::*;
use crate::system::status::Status;
use crate::system::variables::RANDOM_TALK_INTERVAL;
use crate::system::variables::{get_read, get_write, EventFlag, CURRENT_SURFACE, FLAGS, GHOST_UP_TIME, IDLE_SECONDS, LAST_RANDOM_TALK_TIME, TOTAL_TIME, WAITING_FIRST_USER_NAME};
use crate::system::windows::get_local_time;
use shiorust::message::{Request, Response};

pub(crate) fn on_minute_change(_req: &Request) -> Response {
  new_response_nocontent()
}

pub(crate) fn on_second_change(req: &Request) -> Result<Response, ShioriError> {
  // 最小化中かどうかに関わらず実行する処理
  *get_write(&TOTAL_TIME) += 1;
  *get_write(&GHOST_UP_TIME) += 1;

  // 初回起動トークが終わるまではランダムトークなし。
  // 名前の入力待ちも同じ扱い（入力ボックスの上から喋り出さないように）
  if !get_read(&FLAGS).check(&EventFlag::FirstBoot) || *get_read(&WAITING_FIRST_USER_NAME) {
    return Ok(new_response_nocontent());
  }

  let refs = get_references(req);
  let idle_secs = match refs[4].parse::<i32>() {
    Ok(v) => v,
    Err(_) => return Err(ShioriError::ParseIntError),
  };
  *get_write(&IDLE_SECONDS) = idle_secs;

  let status = Status::from_request(req);

  debug!("status: {}", status);
  {
    let random_talk_interval = *get_read(&RANDOM_TALK_INTERVAL);
    if random_talk_interval > 0 && (*get_read(&GHOST_UP_TIME) - *get_read(&LAST_RANDOM_TALK_TIME)) >= random_talk_interval && !status.minimizing {
      return on_ai_talk(req);
    }
  }

  let mut text = String::new();
  {
    if (*get_read(&GHOST_UP_TIME)).is_multiple_of(60) && !status.talking {
      // 1分ごとにサーフェスを重ね直す
      text += STICK_SURFACE;
    }
  }

  let now = get_local_time();
  if now.wMinute == 0 && now.wSecond == 0 {
    text += &format!("\\1\\_q{}時だ", now.wHour);
  }

  if text.is_empty() {
    Ok(new_response_nocontent())
  } else {
    new_response_with_value_with_translate(text, TranslateOption::simple_translate())
  }
}

pub(crate) fn on_surface_change(req: &Request) -> Result<Response, ShioriError> {
  let refs = get_references(req);
  let surface = match refs[0].parse::<i32>() {
    Ok(v) => v,
    Err(_) => return Err(ShioriError::ParseIntError),
  };

  *get_write(&CURRENT_SURFACE) = surface;

  Ok(new_response_nocontent())
}
