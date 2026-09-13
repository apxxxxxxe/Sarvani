use crate::check_error;
use crate::events::talk::BranchTalk;
use crate::system::error::ShioriError;
use crate::system::response::*;
use shiorust::message::{Request, Response};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Question(pub(crate) u32);

impl Question {
  const HOW_OLD_ARE_YOU: Self = Self(0); // 年齢は？
  const WHAT_IS_YOUR_SEX: Self = Self(1); // 性別は？
  const HOW_TALL_ARE_YOU: Self = Self(2); // 身長は？
  const HOW_MUCH_DO_YOU_WEIGHT: Self = Self(3); // 体重は？
  const WHO_IS_LAILA: Self = Self(4); // ライラって？
  const WHAT_IS_YOUR_FAVORITE_FOOD: Self = Self(5); // 好きな食べ物は？

  fn theme(&self) -> String {
    match *self {
      Question::HOW_OLD_ARE_YOU => "年齢は？".to_string(),
      Question::WHAT_IS_YOUR_SEX => "性別は？".to_string(),
      Question::HOW_TALL_ARE_YOU => "身長は？".to_string(),
      Question::HOW_MUCH_DO_YOU_WEIGHT => "体重は？".to_string(),
      Question::WHO_IS_LAILA => "ライラって？".to_string(),
      Question::WHAT_IS_YOUR_FAVORITE_FOOD => "好きな食べ物は？".to_string(),
      _ => {
        error!("Unknown question theme: {:?}", self);
        String::new()
      }
    }
  }

  fn to_script(self) -> String {
    format!("\\![*]\\__q[OnTalkAnswer,{}]{}\\__q", self.0, self.theme())
  }

  /// 回答トークの定義。本文と掘り下げ選択肢（→その後のトーク）をここに一体で書く。
  /// 掘り下げを足すときは BranchTalk::leaf を BranchTalk::node に替えて choices を並べる。
  /// 「回答後に質問一覧へ戻る」動作は with_menu_return が終端ノードへ一括付与するので書かない。
  pub(crate) fn branch_talk(&self) -> BranchTalk {
    match *self {
      Question::HOW_OLD_ARE_YOU => BranchTalk::leaf(
        "\
        \\1『年齢は？』\\n\
        h111120728でス！h1111204店長としては若造でしょウか。\\n\
        h1111206祖父から継いでしばらく経ちまスが、\\n\
        h1113209まだまだ修行中でス。\
        ",
      ),
      Question::WHAT_IS_YOUR_SEX => BranchTalk::leaf(
        "\
        \\1『性別は？』\\n\
        \\0h1111101性別？h2111304エー、どちらだと思いまス？\\n\
        h2111206こっちだったら嬉しいとか、そーゆーのはありまスか？\\n\
        ……h1113209ま、秘密ですヨ。\\n\
        h1113304どうしてもと言うなら、好きに想像してくださいネ。\
        ",
      ),
      Question::HOW_TALL_ARE_YOU => BranchTalk::leaf(
        "\
        \\1『身長は？』\\n\
        h1112103168センチくらいでスかね？\\n\
        だいぶ前でスが、h1112209ま、たぶん変わってないでしょウ。\
        ",
      ),
      Question::HOW_MUCH_DO_YOU_WEIGHT => BranchTalk::leaf(
        "\
            \\1『体重は？』\\n\
            h111130448キロでス！\\n\
            h1141209ちょうど今朝お風呂上がりに測りましたヨ。\
         ",
      ),
      // ユーザが女神の信仰を知る唯一の場。ランダムトーク側は
      // ここを通っていない前提で書いてあるので、説明はここに集める
      Question::WHO_IS_LAILA => BranchTalk::node(
        "\
        \\1『ライラにかけて…とか、何のこと？』\\n\
        \\0h1111308よくぞお尋ねくださいまシタ！\\n\
        h1111207ワタクシの故郷の神でス。\\n\
        h1113206あちらは神が大勢おりましてネ、\\n\
        その一柱が、秘密を司る女神ライラ。\\n\\n[half]\
        h1111204隠されたもの、明かされぬもの、\\n\
        胸の裡にしまわれたまま終わるもの。\\n\
        h1112309そういったものはすべて、\\n\
        ライラがお預かりになっていると申しまス。\\n\\n[half]\
        h2121209美しい神でしょウ？\
        ",
        vec![
          BranchTalk::choice(
            "熱心に信じてるんだね",
            BranchTalk::leaf(
              "\
              \\1『熱心に信じてるんだね』\\n\
              \\0h1141308ハイ、勿論！\\n\
              h1131208鍵屋をやっているのもライラ、\\n\
              ひいては秘密への興味ゆえでス。\\n\\n[half]\
              h1111207秘密を守るものは何か、それは錠と鍵でス。\\n\
              h1113206人が女神にお預けした秘密を、\\n\
              この世で形にしているのが錠と鍵なのでスよ。\\n\\n[half]\
              h2111207ワタクシが錠を一つ拵えるたび、\\n\
              この世にライラの居場所が一つ増える！\\n\
              h2121306これほど良い仕事が、他にありまスか。\
              ",
            ),
          ),
          BranchTalk::choice(
            "他にも神様がいるの？",
            BranchTalk::leaf(
              "\
              \\1『他にも神様がいるの？』\\n\
              \\0h1111207おりますヨ、たくさン。\\n\
              h1113206海の神、炉の神、道に迷った者の神。\\n\
              h1112209さまざまなものに神の存在を見出すのは、\\n\
              h1112304こちらの国でも同じようですネ。\
              ",
            ),
          ),
        ],
      ),
      Question::WHAT_IS_YOUR_FAVORITE_FOOD => BranchTalk::leaf(
        "\
            \\1『好きな食べ物は？』\\n\
            h1111207唐揚げでス！h1113303あ、あとメンチカツ、コロッケ、\\n\
            h1113309焼き鳥、みぞれ煮、ポテトフライ……。\\n\
            h1113604お肉としょっぱい物はだいたい好きでスよ！\
            ",
      ),
      _ => {
        error!("Unknown question talk: {:?}", self);
        BranchTalk::leaf("")
      }
    }
  }
}

/// 終端ノード（choices が空）の末尾に「\x で閉じて質問一覧に戻る」を付与する。
/// 掘り下げ表示中のノードに付けると \x がバルーンごと選択肢を消してしまうため、終端に限る。
fn with_menu_return(mut branch_talk: BranchTalk) -> BranchTalk {
  if branch_talk.choices.is_empty() {
    branch_talk.text.push_str("\\x\\![raise,OnTalk]");
  } else {
    branch_talk.choices = branch_talk
      .choices
      .into_iter()
      .map(|mut choice| {
        choice.next = with_menu_return(choice.next);
        choice
      })
      .collect();
  }
  branch_talk
}

pub(crate) const QUESTIONS: [Question; 6] = [
  Question::HOW_OLD_ARE_YOU,
  Question::WHAT_IS_YOUR_SEX,
  Question::HOW_TALL_ARE_YOU,
  Question::HOW_MUCH_DO_YOU_WEIGHT,
  Question::WHO_IS_LAILA,
  Question::WHAT_IS_YOUR_FAVORITE_FOOD,
];

pub(crate) fn on_talk(_req: &Request) -> Result<Response, ShioriError> {
  let mut questions = QUESTIONS.to_vec();
  questions.sort_by_key(|a| a.0);

  let mut m = "\\_q".to_string();
  for q in questions.iter_mut() {
    m.push_str(&q.to_script());
    m.push_str("\\n");
  }
  m.push_str("\\n\\q[戻る,OnMenuExec]");

  new_response_with_value_with_translate(m, TranslateOption::simple_translate())
}

pub(crate) fn on_talk_answer(req: &Request) -> Result<Response, ShioriError> {
  let refs = get_references(req);
  let q = Question(check_error!(
    refs[0].parse::<u32>(),
    ShioriError::ParseIntError
  ));
  new_response_with_value_with_translate(
    with_menu_return(q.branch_talk()).render(),
    TranslateOption::simple_translate(),
  )
}
