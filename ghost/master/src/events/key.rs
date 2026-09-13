use crate::events::aitalk::on_ai_talk;
use crate::events::talk::first_boot::FIRST_BOOT_TALK;
use crate::events::talk::random_talks_analysis;
use crate::system::error::ShioriError;
use crate::system::response::*;
use crate::system::variables::*;
use shiorust::message::{Request, Response};
use std::collections::HashMap;

pub(crate) fn on_key_press(req: &Request) -> Result<Response, ShioriError> {
  let refs = get_references(req);
  match refs[0] {
    "t" => {
      if !get_read(&FLAGS).check(&EventFlag::FirstBoot) {
        Ok(new_response_nocontent())
      } else {
        on_ai_talk(req)
      }
    }
    "c" => {
      if *get_read(&DEBUG_MODE) {
        Ok(new_response_with_value_with_notranslate(
          random_talks_analysis(),
          TranslateOption::balloon_surface_only(),
        ))
      } else {
        Ok(new_response_nocontent())
      }
    }
    // 初回起動トークの動作確認。名前入力の待ちも立てるので、
    // 入力ボックスとその後の返しまで本番と同じ流れを試せる
    "f" => {
      if *get_read(&DEBUG_MODE) {
        *get_write(&WAITING_FIRST_USER_NAME) = true;
        Ok(new_response_with_value_with_translate(
          FIRST_BOOT_TALK.to_string(),
          TranslateOption::simple_translate(),
        )?)
      } else {
        Ok(new_response_nocontent())
      }
    }
    "d" => {
      if *get_read(&DEBUG_MODE) {
        // 全変数をリセット
        *get_write(&WAITING_FIRST_USER_NAME) = false;
        *get_write(&TOTAL_BOOT_COUNT) = 0;
        *get_write(&TOTAL_TIME) = 0;
        *get_write(&RANDOM_TALK_INTERVAL) = 0;
        *get_write(&USER_NAME) = "".to_string();
        *get_write(&TALK_COLLECTION) = HashMap::new();
        *get_write(&CUMULATIVE_TALK_COUNT) = 0;
        *get_write(&FLAGS) = EventFlags::default();
        Ok(new_response_with_value_with_notranslate(
          format!("\\![change,ghost,{}]", GHOST_NAME),
          TranslateOption::none(),
        ))
      } else {
        Ok(new_response_nocontent())
      }
    }
    _ => Ok(new_response_nocontent()),
  }
}
