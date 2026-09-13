use crate::events::translate::on_translate;
use crate::system::error::ShioriError;
use crate::system::roulette::RouletteCell;
use crate::system::variables::*;
use core::fmt::{Display, Formatter};
use std::collections::HashSet;

use shiorust::message::{parts::HeaderName, parts::*, traits::*, Request, Response};

/// 本体側の既定バルーン。場所ごとの切り替えを廃したので固定値
pub(crate) const SAKURA_BALLOON: &str = "\\b[0]";
pub(crate) const REMOVE_BALLOON_NUM: &str = "\\0\\![set,balloonnum,,,]";
pub(crate) const STICK_SURFACE: &str = "\
  \\C\
  \\1\
  \\![reset,sticky-window]\
  \\![set,alignmenttodesktop,free]\
  \\![move,--X=0,--Y=0,--time=0,--base=0]\
  \\![set,sticky-window,1,0]\
  \\0\
  ";

pub(crate) fn on_stick_surface(_req: &Request) -> Response {
  // \1のサーフェスを\0に重ねて固定する
  new_response_with_value_with_notranslate(STICK_SURFACE.to_string(), TranslateOption::none())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum TranslateOption {
  DoTranslate,
  CompleteBalloonSurface,
}

impl TranslateOption {
  fn new(options: Vec<TranslateOption>) -> HashSet<TranslateOption> {
    options.into_iter().collect()
  }

  pub fn none() -> HashSet<TranslateOption> {
    TranslateOption::new(vec![])
  }

  pub fn balloon_surface_only() -> HashSet<TranslateOption> {
    TranslateOption::new(vec![TranslateOption::CompleteBalloonSurface])
  }

  pub fn simple_translate() -> HashSet<TranslateOption> {
    TranslateOption::new(vec![
      TranslateOption::DoTranslate,
      TranslateOption::CompleteBalloonSurface,
    ])
  }
}

pub(crate) fn add_notice_description(res: &mut Response, error: &str) {
  res
    .headers
    .insert(HeaderName::from("ErrorDescription"), error.to_string());
  res
    .headers
    .insert(HeaderName::from("ErrorLevel"), "notice".to_string());
}

pub(crate) fn add_error_description(res: &mut Response, error: &str) {
  res
    .headers
    .insert(HeaderName::from("ErrorDescription"), error.to_string());
  res
    .headers
    .insert(HeaderName::from("ErrorLevel"), "error".to_string());
}

pub(crate) fn new_response() -> Response {
  let mut headers = Headers::new();
  headers.insert(
    HeaderName::Standard(StandardHeaderName::Charset),
    String::from("UTF-8"),
  );
  Response {
    version: Version::V30,
    status: Status::OK,
    headers,
  }
}

pub(crate) fn new_response_nocontent() -> Response {
  let mut r = new_response();
  r.status = Status::NoContent;
  r
}

pub(crate) fn new_response_with_value_with_notranslate(value: String, option: HashSet<TranslateOption>) -> Response {
  let balloon_completion = if option.contains(&TranslateOption::CompleteBalloonSurface) {
    SAKURA_BALLOON.to_string()
  } else {
    String::new()
  };

  let mut v = balloon_completion + value.as_str();
  // \\Cが含まれているなら文頭に\\Cを補完
  if v.contains("\\C") {
    v = format!("\\C{}", v.replace("\\C", ""));
  }

  let mut r = new_response();
  r.headers.insert(HeaderName::from("Value"), v);
  r
}

pub(crate) fn new_response_with_value_with_translate(value: String, option: HashSet<TranslateOption>) -> Result<Response, ShioriError> {
  let balloon_completion = if option.contains(&TranslateOption::CompleteBalloonSurface) {
    SAKURA_BALLOON.to_string()
  } else {
    String::new()
  };

  let v = if option.contains(&TranslateOption::DoTranslate) {
    on_translate(value)?
  } else {
    value
  };

  let mut v = balloon_completion + v.as_str();
  // \\Cが含まれているなら文頭に\\Cを補完
  if v.contains("\\C") {
    v = format!("\\C{}", v.replace("\\C", ""));
  }

  let mut r = new_response();
  r.headers.insert(HeaderName::from("Value"), v);
  Ok(r)
}

pub(crate) fn choose_one(values: &[impl RouletteCell], update_weight: bool) -> Option<usize> {
  if values.is_empty() {
    return None;
  }
  let u = get_write(&TALK_BIAS).roulette(values, update_weight);
  u
}

// return all combinations of values
// e.g. [a, b], [c, d], [e, f] => "ace", "acf", "ade", "adf", "bce", "bcf", "bde", "bdf"
#[allow(dead_code)]
pub(crate) fn all_combo(values: &Vec<Vec<String>>) -> Vec<String> {
  let mut result = Vec::new();
  let mut current = Vec::new();
  all_combo_inner(values, &mut result, &mut current, 0);
  result.iter().map(|v| v.join("")).collect()
}

#[allow(dead_code)]
fn all_combo_inner(values: &Vec<Vec<String>>, result: &mut Vec<Vec<String>>, current: &mut Vec<String>, index: usize) {
  if index == values.len() {
    result.push(current.clone());
    return;
  }
  for v in values[index].iter() {
    current.push(v.to_string());
    all_combo_inner(values, result, current, index + 1);
    current.pop();
  }
}

pub(crate) fn get_references(req: &Request) -> Vec<&str> {
  let mut references: Vec<&str> = Vec::new();
  const MAX_REF: usize = 10; // とりあえず10個まで取得
  for i in 0..MAX_REF {
    if let Some(value) = req
      .headers
      .get(&HeaderName::from(&format!("Reference{}", i)))
    {
      references.push(value);
    } else {
      references.push("");
    }
  }
  // 最後の空でない参照のインデックスを取得し、それ以降の要素を削除
  let last_valid_index = references.iter().rposition(|&s| !s.is_empty()).unwrap_or(0);
  references.truncate(last_valid_index + 1);
  references
}

// ============================================================
// サーフェス記法（h1111102 形式）のデコード
//
//   h [ポーズ][顔色][眉][腕][口][目2桁]
//
//   ポーズ 0=非表示 1=通常立ち 2=くねくね 3=天を仰ぐ 4=突っ伏す
//   顔色   1=通常 2=赤面
//   眉     1=通常眉 2=困り眉 3=驚き眉 4=怒り眉
//   腕     1=通常手 2=胸に手 3=シー手
//   口     1=閉じ口 2=笑い口 3=笑い開き口 4=開き口
//   目     01=こっち目 03=あっち目 04=こっち半目 06=あっち半目
//          07=笑い目 08=驚き目 09=閉じ目 10=ウインク目
//
// ポーズ桁は素体サーフェスそのものを決め、残りの桁は素体に重ねるパーツを
// bind 命令で指定する。パーツ分割ができているのはポーズ1（通常立ち）と
// ポーズ2（くねくね）で、未分割のポーズは bind アニメーションを持たないため、
// bind 命令は空振りして一枚絵がそのまま出る。
// ポーズ2は胸に手・シー手の素材がまだないため、shell 側で通常手に代用させている。
// 対応する shell 側の定義は shell/master/surfaces.txt と descript.txt を参照。
// ============================================================

/// ポーズ桁に対応する素体サーフェスID
fn base_surface(pose: i32) -> i32 {
  pose * 1_000_000 + 100
}

pub(crate) fn eye_name(code: i32) -> &'static str {
  match code {
    3 => "あっち目",
    4 => "こっち半目",
    6 => "あっち半目",
    7 => "笑い目",
    9 => "閉じ目",
    8 => "驚き目",
    10 => "ウインク目",
    _ => "こっち目",
  }
}

pub(crate) fn mouth_name(code: i32) -> &'static str {
  match code {
    2 => "笑い口",
    3 => "笑い開き口",
    4 => "開き口",
    5 => "いひ口",
    6 => "ぺろ口",
    _ => "閉じ口",
  }
}

pub(crate) fn arm_name(code: i32) -> &'static str {
  match code {
    2 => "胸に手",
    3 => "シー手",
    _ => "通常手",
  }
}

pub(crate) fn eyebrow_name(code: i32) -> &'static str {
  match code {
    2 => "困り眉",
    3 => "驚き眉",
    4 => "怒り眉",
    _ => "通常眉",
  }
}

pub(crate) fn face_color_name(code: i32) -> &'static str {
  match code {
    2 => "赤面",
    _ => "通常",
  }
}

/// 7桁サーフェスコードをデコードし、bind命令群を生成する
pub(crate) fn generate_bind_script(from_surface: i32, dest_surface: i32) -> String {
  let dest_eyes = dest_surface % 100;
  let dest_mouth = (dest_surface / 100) % 10;
  let dest_arm = (dest_surface / 1000) % 10;
  let dest_eyebrow = (dest_surface / 10000) % 10;
  let dest_face = (dest_surface / 100000) % 10;
  let dest_pose = (dest_surface / 1000000) % 10;

  // 同一コードの場合は話者0への切り替えのみ
  if from_surface == dest_surface {
    return "\\0".to_string();
  }

  // h1000000 は非表示指定。
  // 素体サーフェスにbindを重ねる方式では表現できないため、空のサーフェスを直接指定する。
  // ポーズ桁が0のコードは描画すべき素体が決まらないので、同じく非表示として扱う
  if dest_surface == HIDDEN_SURFACE_CODE || dest_pose == 0 {
    return format!(
      "\\0\\![lock,repaint]\\s[{}]\\![unlock,repaint]",
      TRANSPARENT_SURFACE
    );
  }

  format!(
    "\\0\\![lock,repaint]\\s[{}]\\![bind,顔色,{},1]\\![bind,眉,{},1]\\![bind,腕,{},1]\\![bind,口,{},1]\\![bind,目,{},1]\\![unlock,repaint]",
    base_surface(dest_pose),
    face_color_name(dest_face),
    eyebrow_name(dest_eyebrow),
    arm_name(dest_arm),
    mouth_name(dest_mouth),
    eye_name(dest_eyes),
  )
}

#[allow(dead_code)]
pub(crate) enum Icon {
  Cog,
  Cross,
  ArrowRight,
  ArrowLeft,
  Bubble,
  Info,
}

impl Display for Icon {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "\
        \\f[height,14]\\f[name,icomoon.ttf]\
        \\_u[0xE{}]\
        \\f[name,default]\\f[height,default]\
        ",
      self.to_code()
    )
  }
}

impl Icon {
  fn to_code(&self) -> u32 {
    match self {
      Icon::Cog => 902,
      Icon::Cross => 903,
      Icon::ArrowRight => 904,
      Icon::ArrowLeft => 905,
      Icon::Bubble => 906,
      Icon::Info => 907,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generate_bind_script_hidden() {
    // h1000000 は非表示サーフェスへ直接切り替える
    let result = generate_bind_script(1111102, 1000000);
    assert!(result.contains("\\s[0]"));
    assert!(!result.contains("\\![bind,"));
  }

  #[test]
  fn test_generate_bind_script_hidden_without_pose() {
    // ポーズ桁が0のコードは素体が決まらないので非表示として扱う
    let result = generate_bind_script(1111102, 0);
    assert!(result.contains("\\s[0]"));
    assert!(!result.contains("\\![bind,"));
  }

  #[test]
  fn test_generate_bind_script_from_hidden() {
    // 非表示からの復帰は素体サーフェス＋bindで組み立て直す
    // h1111201 = 通常立ち, 顔色1, 通常眉, 通常手, 笑い口(2), こっち目(01)
    let result = generate_bind_script(1000000, 1111201);
    assert!(result.contains("\\s[1000100]"));
    assert!(result.contains("\\![bind,口,笑い口,1]"));
    assert!(result.contains("\\![bind,目,こっち目,1]"));
  }

  #[test]
  fn test_generate_bind_script_pose_with_default_parts() {
    // 一枚絵ポーズは h3000000 のように下位桁を省いて書ける。
    // 下位桁が全て0でも、非表示になるのは h1000000 だけ
    let result = generate_bind_script(1111101, 3000000);
    assert!(result.contains("\\s[3000100]"));
    assert!(!result.contains("\\s[0]"));
  }

  #[test]
  fn test_generate_bind_script_same_surface() {
    // 同一コードの場合は話者0への切り替えのみ
    let result = generate_bind_script(1111102, 1111102);
    assert_eq!(result, "\\0");
  }

  #[test]
  fn test_generate_bind_script_all_parts() {
    // h1123308 = 通常立ち, 顔色1, 困り眉(2), シー手(3), 笑い開き口(3), 驚き目(08)
    let result = generate_bind_script(1111101, 1123308);
    assert!(result.contains("\\s[1000100]"));
    assert!(result.contains("\\![bind,顔色,通常,1]"));
    assert!(result.contains("\\![bind,眉,困り眉,1]"));
    assert!(result.contains("\\![bind,腕,シー手,1]"));
    assert!(result.contains("\\![bind,口,笑い開き口,1]"));
    assert!(result.contains("\\![bind,目,驚き目,1]"));
  }

  #[test]
  fn test_generate_bind_script_pose_selects_base_surface() {
    // ポーズ桁が素体サーフェスを決める
    assert!(generate_bind_script(1111101, 2111101).contains("\\s[2000100]"));
    assert!(generate_bind_script(1111101, 3111101).contains("\\s[3000100]"));
    assert!(generate_bind_script(1111101, 4111101).contains("\\s[4000100]"));
  }

  #[test]
  fn test_generate_bind_script_no_blink_completion() {
    // まばたき補完は行わないので、目のbindは常に1回だけ・遅延なし
    let result = generate_bind_script(1111101, 1111102);
    assert_eq!(result.matches("\\![bind,目,").count(), 1);
    assert!(!result.contains("\\_w["));
  }

  #[test]
  fn test_generate_bind_script_unknown_code_falls_back() {
    // Haineから引き継いだ未定義コードでも既定のパーツに落ちて壊れない
    let result = generate_bind_script(1111101, 1111211);
    assert!(result.contains("\\![bind,目,こっち目,1]"));
  }

  #[test]
  fn test_eye_name_mapping() {
    assert_eq!(eye_name(1), "こっち目");
    assert_eq!(eye_name(3), "あっち目");
    assert_eq!(eye_name(4), "こっち半目");
    assert_eq!(eye_name(6), "あっち半目");
    assert_eq!(eye_name(7), "笑い目");
    assert_eq!(eye_name(8), "驚き目");
    assert_eq!(eye_name(9), "閉じ目");
    assert_eq!(eye_name(10), "ウインク目");
    assert_eq!(eye_name(99), "こっち目");
  }

  #[test]
  fn test_mouth_name_mapping() {
    assert_eq!(mouth_name(1), "閉じ口");
    assert_eq!(mouth_name(2), "笑い口");
    assert_eq!(mouth_name(3), "笑い開き口");
    assert_eq!(mouth_name(4), "開き口");
  }

  #[test]
  fn test_arm_and_eyebrow_name_mapping() {
    assert_eq!(arm_name(1), "通常手");
    assert_eq!(arm_name(2), "胸に手");
    assert_eq!(arm_name(3), "シー手");
    assert_eq!(eyebrow_name(1), "通常眉");
    assert_eq!(eyebrow_name(2), "困り眉");
    assert_eq!(eyebrow_name(3), "驚き眉");
  }
}
