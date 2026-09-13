use std::collections::HashMap;

use crate::events::talk::{Talk, TalkType};

use super::DerivaliveTalk;
use crate::system::windows::get_local_time;

struct RandomTalk {
  id: String,
  text: String,
  required_condition: Option<fn() -> bool>,
  callback: Option<fn()>,
}

// 季節や時間帯に触れるトークの required_condition。時刻は利用者の PC のローカル時刻

/// 昼前。「お昼は〜にしましょウ」が通る時間
fn is_before_lunch() -> bool {
  (10..=12).contains(&get_local_time().wHour)
}

/// 昼過ぎ。客のいない午後にうとうとする時間
fn is_afternoon() -> bool {
  (13..=16).contains(&get_local_time().wHour)
}

/// 昼以降。「今朝」を振り返れる時間
#[allow(dead_code)]
fn is_after_noon() -> bool {
  get_local_time().wHour >= 12
}

/// 夕方。商店街に夕飯の匂いがする時間
fn is_evening() -> bool {
  (16..=18).contains(&get_local_time().wHour)
}

/// 春。「暖かい」と言える季節
fn is_spring() -> bool {
  (3..=5).contains(&get_local_time().wMonth)
}

/// いま喋ってよいトーク。required_condition（季節や時間帯）を満たすものだけ。抽選に使う
pub(crate) fn random_talks(talk_type: TalkType) -> Option<Vec<Talk>> {
  talks_of(talk_type, true)
}

/// 条件を問わず、定義されているトークすべて。既読の整理、本数の集計、親トークの検索、書き出しに使う。
/// ここで条件を見ると、時間帯の外で起動したときに既読の記録が消えてしまう
pub(crate) fn all_random_talks(talk_type: TalkType) -> Option<Vec<Talk>> {
  talks_of(talk_type, false)
}

fn talks_of(talk_type: TalkType, respect_condition: bool) -> Option<Vec<Talk>> {
  let strings: Vec<RandomTalk> = match talk_type {
    TalkType::Work => work_talks(),
    TalkType::Lore => lore_talks(),
    TalkType::Lock => lock_talks(),
    TalkType::AboutMe => about_me_talks(),
    TalkType::WithYou => with_you_talks(),
  };

  let mut talks = Vec::new();
  for st in strings {
    if let Some(expr) = st.required_condition {
      if respect_condition && !expr() {
        continue;
      }
    }
    talks.push(Talk::new(
      Some(talk_type),
      st.id,
      st.text.to_string(),
      st.callback,
    ));
  }
  Some(talks)
}

fn work_talks() -> Vec<RandomTalk> {
  vec![
    RandomTalk {
      id: "錠を選ぶ".to_string(),
      text: "\
        \\0h1111207当店では貴重品のお預かりも承っておりまス！\\n\
        h1113203錠は当店のものからお選びいただけまスよ。\\n\
        \\1「コインロッカーで良いのでは……」\\n\
        \\0h1113209ノンノン、あれは無表情な番号たちでス。\\n\
        h1113209彼らは冷たく勤勉ですが、こちらを省みなイ。\\n\
        誰のものでもない孤高の鍵なのでスよ。\\n\\n[half]\
        h2121209一方こちらは、ご自分で数ある中から選んだ錠と鍵。\\n\
        それだけで心の持ちようは変わり、\\n\
        ポケットの中の鍵への情も湧くというものでス。\\n\
        h2111206使用後、購入するならば割引サービスもありますヨ。\\n\\n[half]\
        h1112506……ええ、ご利用者はめったにいませンがネ。\
          "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "こじ開ける".to_string(),
      text: "\
        \\0h1111104錠をこじ開けるのは、乱暴ですが有効な手段でス。\\n\
        h1121209文明人としては褒められませンがネ。\\n\\n[half]\
        h1111201ご依頼によっては、箱はどうなってもいいから\\n\
        中身を出したいということもあるのでス。\\n\
        h1121309ワタクシは涙をのんでこじ開けるのみ。\\n\
        h1111204惜しいでスが、引き取っても廃棄しかできませン。\\n\
        h1113203ワタクシにできることは、\\n\
        それぞれの錠を隅々まで覚えておくことダケ。\\n\
        h1112206そう、頭の中にはいつまでも置いておけまスよ。\\n\
        h1112207一つ残らず。\
          "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "合鍵の癖".to_string(),
      text: "\
          h1111207\\1(カランカラン…)\\n\\n[half]\
        \\0h1111307ありがとうございまシた！\\n\\n[half]\
        h1111203……いまの方には、合鍵を一本お作りしまシた。\\n\
        h1112309鍵というものは、切り出したばかりの状態では\\n\
        回すと硬いのでス。h1111306しかしそれは今だけ。\\n\
        h1121207これから毎日、あの鍵は錠に削られていきまス。\\n\
        一年も経てば、あの方の手になじんだ\\n\
        世界で一本きりの鍵になっているでしょウ。\\n\\n[half]\
        h2121209ああ、なんと美しい！h1212306ライラにかけて、\\n\
        この仕事をしていてよかったと心から思いまス。\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "百年前の職人".to_string(),
      text: "\
        \\0h1111207修理でお預かりした古い錠を分解したのでスが、\\n\
        部品に興味深い手癖が残っておりましタ。\\n\
        \\0hh1111301バネの当たる面に削りの跡がありまシて、\\n\
        h1112309規格通りではない、この職人だけの逃がしでスね。\\n\\n[half]\
        h2121209百年前の誰かが悩み、削った痕跡。\\n\
        h2121306そこに今触れている！h2121203なんたるロマンでしょうカ。\\n\
        h1111206手元に置いておけないのが残念でスよ、本当に。\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "ヘアピン".to_string(),
      text: "\
        \\0h1111207映画でよく、ヘアピン一本で錠を開けまスでしょう？\\n\
        h1111204確かにヘアピンでも開きまスが、必要な本数は二本。\\n\
        \\0h1111204一本で回す力をかけ、\\n\
        もう一本で、中のピンを一本ずつ持ち上げるのでス。\\n\\n[half]\
        h1112303正確な描写をしている作品もたまにありまスが、\\n\
        h1112207その点だけで作品への好感度はアゲアゲですネ！\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "鉛筆の芯".to_string(),
      text: "\
        h1111207鍵の回りが渋いときは、\\n\
        鍵を鉛筆の芯でこすってみてくださイ。\\n\
        h1113204油は厳禁ですヨ。h1123206埃が絡みますからネ。\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
  ]
}

/// 女神ライラと故郷の伝承。設定は .claude/lore/laila.md を見る。
/// ユーザはこの信仰を知らない前提なので、初出の語には短い注釈を添える
fn lore_talks() -> Vec<RandomTalk> {
  vec![
    RandomTalk {
      id: "開かずの神殿".to_string(),
      text: "\
        \\0h1111207ワタクシの故郷には、ライラの神殿がありましてネ。\\n\
        h1111309その神殿、建ってから三百年、\\n\
        一度も扉が開いておりませン。\\n\\n[half]\
        \\1「入れないの？」\\n\
        \\0h1111207入れませンとも！\\n\
        h1113206何重もの錠がかかり、鍵はすべて鋳溶かされ、\\n\
        錠が錠とわからぬよう隠されているのでス。\\n\\n[half]\
        h1111204中に何があるのか、誰も知らなイ。\\n\
        h1122207ですからネ、あれは本当に完璧な神殿なのでスよ。\\n\
        h1122304開かない箱こそ、女神の御身そのものでスから。\
          "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "祈りの作法".to_string(),
      text: "\
        \\0h1113206秘密の女神ライラへの祈りは、声に出さないのでス。\\n\
        \\1「黙って祈るってこと？」\\n\
        \\0h1111204いいえ、もっと厳密でス。\\n\\n[half]\
        h1112309願い事を紙に書き、錠のかかる箱に入れて閉じる。\\n\
        h1111207それでおしまイ、二度と開けませン。\\n\
        h1113204口に出した願いは、もう秘密ではありませンから。\\n\
        h1112309女神に届くのは、閉じたものだけなのでス。\\n\\n[half]\
        h1121209ワタクシも十二の年に一つ、故郷に置いてきまシタ。\\n\
        h1121207何を書いたかは、もう覚えておりませン。\\n\
        h1113306ですからネ、あれは正しく祈りになったのでスよ。\
          "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "トゥエラ".to_string(),
      text: "\
        \\0h1111201故郷の神々には、トゥエラという神も\\n\
        おりましてネ。h1113203証文と取引を司りまス。\\n\
        約束は書き記し、必要に応じて誰にでも見せよ、と。\\n\
        h1112309秘密を守れと説くライラとは、\\n\
        まるきり折り合いが悪イ。\\n\\n[half]\
        \\1「同じ国にいて大丈夫なの……？」\\n\
        \\0h1111307もちろんでス！h1113209昼はトゥエラ、夜はライラと\\n\
        故郷の商売人はたいそう難儀しまスが、\\n\
        同時に揺れ動くことこそを楽しみ、尊ぶのでス。\\n\\n[half]\
        h1242309もちろんワタクシは迷いませんがネ！\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
  ]
}

/// 錠前のコレクション。良い錠とは頑丈な錠だ、という常識を外した話を集める。
/// 「こじ開ける」の店の奥の棚、「開かずの神殿」の隠された錠と地続き
fn lock_talks() -> Vec<RandomTalk> {
  vec![
    RandomTalk {
      id: "鍵のない錠".to_string(),
      text: "\
        \\0h1111207ガラス棚の下から二段目は、\\n\
        鍵を失くした錠ばかりでス。\\n\
        \\1「開けられないの？」\\n\
        \\0h1111307開けられまスとも！h1143204ワタクシの腕をお疑いで？\\n\\n[half]\
        h1111206ですが、開けませン。h1112309鍵がないというだけで、\\n\
        あの錠たちはまだ役目のさなかにあるのでスよ。\\n\
        h1122309アア、なんと健気なこと！\\n\
        h1122206ライラにかけて、ワタクシには開けられませン。\
          "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "錠に見えない錠".to_string(),
      text: "\
        \\0h1111207お客様、その本棚の三段目の、左から四冊目の本。\\n\
        \\1「……取れない？」\\n\
        h1141308錠でス！h1131201カバーの下に鍵穴がありまスでしょう？\\n\
        h1111309棚の下部の収納が、それで施錠されているのでス。\\n\\n[half]\
        h1113606こういうものを集めておりましてネ。\\n\
        燭台、床板、階段の手すり。\\n\
        h1112309錠と見えぬ錠こそ、ライラの御業に一番近いのでス。\\n\\n[half]\
        h1111204この店にもいくつ仕込んであることか……。\\n\
        h1113307ヘヘ、h1113204お教えしませンよ。\
          "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "一番安い錠".to_string(),
      text: "\
        \\0h1111207ガラス棚の中央に、粗末な錠がありまスでしょう？\\n\
        h1111206これはワタクシにとって特別なモノでしてネ。\\n\
        \\1「高いの？」\\n\
        \\0h1113301いいエ、当時で銅貨三枚。h1113203針金一本で開きまス。\\n\\n[half]\
        h1111204しかしこれを買った者はこれで戸を閉め、\\n\
        これで安心、とぐっすり眠ったのでス。\\n\
        h1112309針金一本で開くと知らずにネ。\\n\\n[half]\
        h1141308ですがそれでいいのでス！\\n\
        h1131201錠の仕事は、こじ開けられぬことではありませン。\\n\
        h2121209それは「ここから先は入れない」と告げること。\\n\
        h2111206この安物は、それを立派にやり遂げたのでスよ。\
          "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "聴診器".to_string(),
      text: "\
        \\0h1111207金庫破りといえば、ダイヤルに聴診器でスね。\\n\
        h1111204古い金庫なら音で分かることもありまス。\\n\
        h1112309ですが本職は、耳より指で感じ取るのでス。\\n\\n[half]\
        h1121207ダイヤルを回すと、決まったところで\\n\
        かすかに手ごたえが変わりまス。\\n\
        h1112309それを一つずつ書き留めて、方眼紙に点を打つ。\\n\
        h2121209点が並ぶと、奥の円盤の溝がどこにあるか\\n\
        見えてきまして──\\n\\n[half]\
        h1113204オット、ここから先は泥棒の手引きでスね。\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
  ]
}

/// サルバニ自身の話題。ただし自分語りの場ではない。
/// ユーザとは店員と客の間柄がずっと続くので、素性や過去は背後に置いたままにする。
/// 加減は .claude/lore/sarvani_voice.md の「距離は変わらない」を見る
fn about_me_talks() -> Vec<RandomTalk> {
  vec![
    RandomTalk {
      id: "肉屋のメンチカツ".to_string(),
      text: "\
        h1111207向かいにお肉屋がありますでしょウ？\\n\
        h1113206あそこのメンチカツは絶品なのですヨ。\\n\
        h2121309揚げたてサクサクで、中はジュワッとジューシー。\\n\
        h1113607言葉にすれば月並ですが、あの味は格別でス。\\n\
        h1113504お客様もぜひお試しくださイ。\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "あくび".to_string(),
      text: "\
        h1112409フワーァ……。h1123201オット失礼。\\n\
        h1121209この時間は眠くていけませんネ。\
        "
      .to_string(),
      required_condition: Some(is_afternoon),
      callback: None,
    },
    RandomTalk {
      id: "どこかに行きたい".to_string(),
      text: "\
        h1111206どこかへパーッと出かけたいですネェ。旅行とか。\\n\
        h1121209じっとしているのは性に合いませン。\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "揚げ物の匂い".to_string(),
      text: "\
        h1111209ンー、揚げ物のいい匂いがしますネ。\\n\
        h1113606お昼はコロッケにしましょウ。\
        "
      .to_string(),
      required_condition: Some(is_before_lunch),
      callback: None,
    },
    RandomTalk {
      id: "探し物".to_string(),
      text: "\
        h1121204アレ、ドライバーはどこに置いたか……。\\n\
        h1111306アッ、あった。\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "今月の売上".to_string(),
      text: "\
        h1123204ええと、今月の売上がこれで、出費は……\\n\
        ……h1121207マ、なんとかなりまスね！\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "暖かい日".to_string(),
      text: "\
        h1111206今日は暖かいですネ。\\n\
        h2121309こういう日は、窓を開けて作業したくなりまス。\
        "
      .to_string(),
      required_condition: Some(is_spring),
      callback: None,
    },
    RandomTalk {
      id: "夕方の商店街".to_string(),
      text: "\
        h1111206夕方の商店街は、いい匂いばかりでスね。\\n\
        h2121309焼き鳥、コロッケ、お醤油の焦げる匂い……。\\n\
        \\1(ぐぅ〜)\\_w[1000]\\n\
        ！\
        h1111207ヘヘ、お腹がすきますよネ。\\n\
        "
      .to_string(),
      required_condition: Some(is_evening),
      callback: None,
    },
    RandomTalk {
      id: "暴力という錠".to_string(),
      text: "\
        h1121109文明人として、暴力には反対したいところでス。\\n\
        h1121206しかし、暴力に対抗できるのもまた暴力。\\n\
        h1123303こと守るために振るわれるものに関しては、\\n\
        あるいは最も強力な錠と言えるかもしれませんネ。\\n\\n[half]\
        h1122309イヤしかし、そんな必要はないのが一番でス。\\n\
        h1122504だってほら、見ての通り負けてしまいまスから、\\n\
        ひょろひょろのワタクシでは。\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
  ]
}

/// お客様との会話。ユーザを見ている話を集める。
/// 関心は濃いが距離は詰めない。踏み込まないことを楽しむ側に立たせる
fn with_you_talks() -> Vec<RandomTalk> {
  vec![
    RandomTalk {
      id: "新居の鍵".to_string(),
      text: "\
        \\0h1111207お客様、最後にお引っ越しなさったとき、\\n\
        錠は替えてもらいまシタか？\\n\
        h1111204……鍵を新しく受け取っただけ、でスか。\\n\
        h1121201前の方が合鍵を持っていないとは限りませン。\\n\
        h1111209錠ごと替えるのが安心でス。\\n\\n[half]\
        h1113204それに、新しい錠は清々しく、良いものですヨ。\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "透明な錠".to_string(),
      text: "\
        \\0h1111207これを持ってみてくださイ。\\n\
        \\1透明な樹脂の錠だった。中の部品まで透けて見える。\\n\
        \\0h1111204練習用でス。道具を二本お貸ししまスね。\\n\\n[half]\
        \\1言われるままに手を動かすと――\\n\\n[half]\
        (かちり)\\n\\n[half]\
        \\0h1111307開きまシタね！\\n\
        \\1「……意外と簡単」\\n\
        h1111207お宅の錠も、だいたい同じ作りでスよ。\\n\\n[half]\
        h1113204良い錠のご用命は、いつでもどうぞ。\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "ギザギザかくぼみか".to_string(),
      text: "\
        h1111207お家の鍵、ギザギザでス？それとも丸いくぼみ？\\n\
        h2121309くぼみのほうなら、良いのを付けていますネ！\\n\
        h2111206ディンプルキーといって、複雑で堅牢なタイプでス。\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
    RandomTalk {
      id: "字がきれい".to_string(),
      text: "\
        h1111104\\1サルバニは書類になにかを書きつけている。\\n\
        ……きれいな字だ。\\n\
        「すごく字がきれい」\\n\\n[half]\
        h1111304ヘヘ、ありがとうございまス。\\n\
        h1111209こっちに来て、たくさん練習しまシたからネ。\\n\
        h1111204字がちゃんとしているだけで\\n\
        評判がずいぶん違うのでスよ。h1111504お得でス。\\n\
        "
      .to_string(),
      required_condition: None,
      callback: None,
    },
  ]
}

pub(crate) fn derivative_talks() -> Vec<DerivaliveTalk> {
  vec![]
}

pub(crate) fn derivative_talks_per_talk_type() -> HashMap<TalkType, Vec<DerivaliveTalk>> {
  let all_talks = TalkType::all()
    .iter()
    .map(|t| all_random_talks(*t))
    .flat_map(|t| t.unwrap_or_default())
    .collect::<Vec<_>>();
  let mut talks: HashMap<TalkType, Vec<DerivaliveTalk>> = HashMap::new();
  for talk in derivative_talks() {
    let parent_talk = match all_talks.iter().find(|t| t.id == talk.parent_id) {
      Some(t) => t,
      None => {
        error!("Parent talk with id {} not found, skipping", talk.parent_id);
        continue;
      }
    };
    if let Some(tt) = parent_talk.talk_type {
      talks.entry(tt).or_default().push(talk);
    } else {
      error!(
        "Parent talk {} has no talk_type, skipping derivative",
        parent_talk.id
      );
    }
  }
  talks
}

/// 派生トークを書くための入口。derivative_talks() が空のあいだは呼び出し側がいない
#[allow(dead_code)]
pub(crate) fn derivative_talk_by_id(parent_id: &str) -> Option<Vec<DerivaliveTalk>> {
  derivative_talks()
    .into_iter()
    .filter(|t| {
      let condition_ok = match &t.required_condition {
        Some(condition) => condition(),
        None => true,
      };
      t.parent_id == parent_id && condition_ok
    })
    .collect::<Vec<_>>()
    .into()
}

pub(crate) fn get_parent_talk(derivative_talk: &DerivaliveTalk) -> Option<Talk> {
  let all_talks = TalkType::all()
    .iter()
    .map(|t| all_random_talks(*t))
    .flat_map(|t| t.unwrap_or_default())
    .collect::<Vec<_>>();
  let result = all_talks
    .into_iter()
    .find(|t| t.id == derivative_talk.parent_id);
  if result.is_none() {
    error!(
      "Parent talk with id {} not found",
      derivative_talk.parent_id
    );
  }
  result
}
