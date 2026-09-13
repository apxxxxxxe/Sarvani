# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

伺かゴースト「鍵屋サルバニ」。SHIORI を Rust で実装している。

## このリポジトリの成り立ち

同作者のゴースト [Crave The Grave（ハイネ）](../Haine/) のエンジンを下敷きに始めた別ゴースト。2026-09-24 に元ゴースト由来の記述を一掃した（コミット `031382c`〜`b0db707`）。

**残した汎用エンジン**: トーク抽選、ルーレット（`system/roulette.rs`）、メニュー枠、翻訳層（`events/translate.rs`）、セーブ土台（`system/variables.rs`）、BranchTalk、チェイントーク、`phased_talks`。

**元ゴーストごと捨てた機構**: 場所概念（部屋移動・没入モード）、ナイトテーブル（お茶とクッキーの状態機械）、トーク分類の段階解放、初回ランダムトーク連鎖、回想メニュー、ハロウィン仮装。復活させたくなったら Haine 側の履歴を見ればよいので、こちらに痕跡は残していない。

sakura が サルバニ、kero は「あなた」（`char2.menu,hidden`）。

## いま書けているところ

本数は動くので、正確な数は `cargo run --bin dump_talks` か `randomtalk.rs` を見る。下は分厚さの目安。

| 場所 | 状態 |
|---|---|
| `randomtalk.rs` の `Work`（仕事と店） | 6本。いちばん厚い |
| 同 `Lore`（女神と伝承） | 3本 |
| 同 `Lock`（錠前のコレクション） | 4本 |
| 同 `WithYou`（お客様との会話） | 4本 |
| 同 `AboutMe`（サルバニ自身） | 8本。メンチカツ・あくびなど、生活の隙を見せる2行ほどの短いもの |
| `menu/questions.rs` | 6問。年齢・性別・身長・体重・好きな食べ物は一言で答える短いもの。「ライラって？」だけ掘り下げ選択肢つき。答えた数字は `sarvani.md` の「質問トークで本人が答えたこと」 |
| `talk/first_boot.rs` | 初回起動の出会い。南京錠のかかった小箱を開ける一件。末尾でユーザ名を尋ねる（下の「初回起動の名前入力」） |
| `bootend.rs` の `on_boot` / `on_close` | 来店と退店の短い挨拶。来店は一言、退店は3本から抽選 |
| `events/mouse.rs` | 頭3・胸3・手3本。胸と手はダブルクリックも撫でと同じ台詞。肩は撫でもダブルクリックも一言返してメニュー。書き方は `sarvani_voice.md` の「触り反応」 |
| `talk/anchor.rs` | 定義ゼロ（`None` を返すだけ） |

`#[allow(dead_code)]` が付いている関数（`derivative_talk_by_id`、`all_combo`、`EventFlags::delete` など）は、トークを書き足すときに使う道具。消さずに残してある。

## 資料の所在

### 創作設定 `.claude/lore/`
- `sarvani_voice.md` — トーク本文の話法。読者が持っている前提、人物（陽気さ、癖）、感情の出し方、締め方の散らし方、ユーザと依頼人の配置、口調、ポーズの当て方。**トークを書く前に読む**
- `sarvani.md` — 人物の出自と来歴、作者が書いた台詞、暮らしの在庫（作者が選んだ具体的な細部）。`AboutMe` はここが本体になる。**性別は決めない**という制約もここ
- `laila.md` — 秘密の女神ライラ。`Lore`（女神と伝承）の分類はこれが本体になる
- `shop.md` — 店の台帳。屋号（志賀鍵店）、街（伏原）、商い、預かりサービス、錠のコレクション、店に来る人。`Work` と `Lock` を書くときに既出と食い違わせないために見る

開発詳細（dev）の子ファイルは元ゴーストのものだったので持ち込んでいない。

### 参考ゴーストの調査 `.claude/research/`
- `favorite_ghosts/` — 作者の手元の参考ゴースト55体（`C:\Users\user1\Ukagaka\Ukagaka-Ghost\Favorite`）の調査メモ。コンビかソロかの分類、ランダムトークの型。**参考ゴーストを調べる前に `README.md` を読む**。同じ調査を繰り返さないための置き場
- `ssp_ghosts/` — 手元にインストールされているゴースト483体（`C:\Users\user1\Ukagaka\ssp\ghost`）の調査メモ。いまは `question_talks.md`（質問トークの傾向）だけ。**質問トークを増やす前に読む**
- `tools/` — 里々のランダムトークを抜き出す、\1 の使われ方を数える、などの調査用スクリプト

### スキル
- `.claude/skills/stop-ai-slop-jp/` — 日本語の AI 臭を落とす。ゴースト非依存なのでそのまま使える。
- `.claude/skills/stop-ai-slop-talk-jp/` — 台詞版。**用例がすべてハイネのもので、サルバニには合わない**。使うなら用例を差し替えてから。

元ゴーストのトーク作成は `haine-mimicry`（手本駆動）というスキルが担っていたが、こちらには持ち込まれていない。サルバニの口調を固めるときに、同じ形の手本駆動スキルを立てるかどうかから決める。

## トークの書き方

### サーフェスコード

本文中の `h1111204` のような7桁コードを `system/response.rs` の `generate_bind_script` が分解し、素体サーフェス＋`\![bind,...]` に組み立て直す。桁の割り当ては上位から:

```
h 1 1 1 1 2 04
  │ │ │ │ │ └── 目（下2桁）
  │ │ │ │ └──── 口
  │ │ │ └────── 腕
  │ │ └──────── 眉
  │ └────────── 顔色
  └──────────── ポーズ（素体サーフェスを決める）
```

例: `h1123308` は ポーズ1・顔色1・困り眉(2)・シー手(3)・笑い開き口(3)・驚き目(08)。

部品名は `shell/master/descript.txt` の bindgroup 定義と一致していなければならない。未定義のコードは既定の部品に落ちるので壊れはしないが、意図した絵にはならない。

`h1000000` は非表示（完全透明サーフェスへ直接切り替え）。ポーズ桁が 0 のコードも素体が決まらないので同じ扱いになる。

ポーズは4種あるが、**トークに書いてよいのは1（通常立ち）と2（くねくね）だけ**。3（天を仰ぐ）と4（突っ伏す）はシェル側が作業中なので、対応が済むまで使わない。

ポーズ2の腕1は「頬に両手を添える」になる。素体側に `animation261001` は無く、`bindgroup261001.addid` の `191001` で成立している。検証もこの addid を辿るので、OK と出る。

### 動作確認のキー

`events/key.rs` の `OnKeyPress`。ゴーストフォルダに `debug` という名のファイル（またはフォルダ）があるとデバッグモードになり、下の `c` `d` `f` が効くようになる。無いときは押しても何も起きない。

| キー | 効果 | デバッグモード限定 |
|---|---|---|
| `t` | ランダムトークを1本喋る | — |
| `c` | トークの本数と既読数を一覧する | ✓ |
| `d` | セーブ内容を全部リセットしてゴーストを読み直す | ✓ |
| `f` | 初回起動トークを頭から流す。名前入力の待ちも立つので、入力ボックスとその後の返しまで確かめられる | ✓ |

### 書いた後の検査

```
python ghost/master/tools/check_surfaces.py ghost/master/src/events/talk/randomtalk.rs
python ghost/master/tools/check_gender.py
```

`check_surfaces.py` は本文中の7桁コードを拾い、**桁 → 部品名（`response.rs` の対応表）→ bindgroup 番号（`descript.txt`）→ 素体の bind アニメーション（`surfaces.txt`）** と辿って、どこで切れたかを述べる。判定はすべてソースとシェルから読むので、シェルの部品が増えても番号が変わっても追従する。`cargo run --bin dump_talks` が書き出す `all_talks.txt` を渡せば全トークをまとめて確認できる。

出るもの:

- `NAME NOT IN SHELL` — その部品名の bindgroup が `descript.txt` に無い。bind が不発になる
- `NOT IN POSE` — bindgroup はあるが、そのポーズの素体に bind アニメーションが無い。**そのパーツだけ消える**（袖が無い腕など）
- `FALLBACK` — 対応表に無い桁。`response.rs` の既定の部品に落ちるので壊れはしないが、書いた意図とは違う
- `一枚絵（bind なし）` — パーツ未分割のポーズ。下位桁は効かないので何を書いても同じ

`list_endings.py` はトークの締めの二行と最後の表情を並べ、締めのポーズ・目・口・腕・話者の内訳を数える。合否は出さない。一本ずつ書いていると終わり方の偏りに気づけないので、書き足したら並べて眺める（ポーズ2が増えすぎていないか、同じ締めが続いていないか）。`all_talks.txt` を渡す。既定ではランダムトークだけを見て、`--all` で初回起動や質問も含める。

```
python ghost/master/tools/list_endings.py ghost/master/all_talks.txt
```

`check_gender.py` はサルバニの性別を決めつける語を探す。引数なしで CLAUDE.md・`.claude/lore`・`src` をまとめて見る。地の文で崩しやすいので、資料を書いたときも通す。

### 分量の目安

デフォルトバルーンは **`\0` 側が1行24文字の10行、`\1` 側が1行24文字の5行**。あふれた分はスクロールになり、ユーザが操作しないと後半が読めない。慣例としても1トークはバルーンに収まる程度が好ましいので、**この枠に収めることを目標にする**。

ただし、どこまで厳しく守るかは場所による。

| 場所 | 縛り |
|---|---|
| ランダムトーク | **長さを混ぜる。** 2行の一言、5行前後、8〜10行を混ぜ、10行は上限（`sarvani_voice.md` の「長さは混ぜる」）。勝手に始まるので、読む構えのないユーザにスクロールを強いない |
| 質問トーク | 緩い。ユーザが自分で選んで開くので、読む用意ができている |
| 初回起動 | 長くて当たり前。`\x` でページに割る |

```
cd ghost/master && cargo run --bin dump_talks
python ghost/master/tools/check_balloon.py ghost/master/all_talks.txt
```

`--show <名前の一部>` を付けると、そのトークを折り返した結果が行番号付きで並ぶ。どこであふれたかを見ながら削れる。

数え方で効いてくるもの:

- サーフェスコードは `generate_bind_script` が必ず `\0` 付きに展開するので、`\1` の地の文の直後にサーフェスコードを書けば本体側に戻る
- `\n` は1行、`\n[half]` は半行の送り。`\n\n[half]` の段落区切りは1.5行使う
- `\x` でバルーンが空になるので、そこから数え直しになる。長い話はページで割れる
- `\1` 側は5行しかないうえ、地の文がトークの終わりまで積み上がる。3〜4回地の文を入れるとあふれる

### バルーン

場所ごとの切り替えを廃したので、本体は `SAKURA_BALLOON`（`\b[0]`）固定、メニューは `\b[2]`。バルーンが決まったら割り直す。`install.txt` に `balloon.directory` は書いていない。

### 場面の割り当て

ユーザの来店は `OnBoot`（初回は `talk/first_boot.rs`）、退店は `OnClose`（`bootend.rs`）。**ランダムトークに来店と退店は書けない。** 詳しくは `.claude/lore/sarvani_voice.md` の「場面の割り当て」。

### 初回起動の名前入力

初回起動トークは末尾の `\![open,inputbox,user_name,0]` で終わる。返答は `events/input.rs` が受け、`OnUserInput` なら初回だけの専用の返し、`OnUserInputCancel` なら断られたときの返しになる。呼び名の変更（メニュー）から来た場合は従来の返しのまま。

**教えてもらえなかったときは呼び名を「お客」にする**（`DEFAULT_USER_NAME`）。`{user_name}様` が「お客様」になるので、そのまま普段の呼び方に落ちる。断られた場合と、空のまま確定された場合の両方でこうする。呼び名の変更から空で確定された場合だけは、いまの呼び名を残す。

入力を待つあいだは `WAITING_FIRST_USER_NAME`（揮発）が立っていて、`periodic.rs` がランダムトークを、`mouse.rs` がメニューと触り反応を止める。**入力を受けたときと断られたときの両方で下ろすこと。** 下ろし損ねるとその起動のあいだずっと黙る。

`\![enter,passivemode]` でも同じ抑止はできるが、[UKADOC](https://ssp.shillest.net/ukadoc/manual/list_sakura_script.html) にあるとおりパッシブモード中は最小化も終了もできなくなる。解除し損ねたときの被害が大きいので使っていない。なお入力ボックスを開いている間も他の操作は止まらない（SSP）。時間切れの `OnUserInputCancel` に応答しないと、`"timeout"` を入力値とした `OnUserInput` が来る。

### トークを足す

`randomtalk.rs` の `work_talks()` と同じ形で分類ごとに関数を生やし、`random_talks()` の `match` から引く。分類自体を増やすときは `events/talk.rs` の `TalkType` に足す（`Display` の実装も忘れずに）。

`cargo run --bin dump_talks` で全トークを `all_talks.txt` に書き出せる（gitignore 対象）。

**季節や時間帯に触れるトークには `required_condition` を付ける。**「今日は暖かい」「お昼は」「この時間は眠い」「今朝」のように、実際の季節や時刻と食い違いうる言葉があれば必ず付ける。判定の関数（`is_spring`、`is_before_lunch`、`is_afternoon`、`is_after_noon`、`is_evening`）は `randomtalk.rs` の頭にあり、時刻は `system::windows::get_local_time` で取る。合う関数がなければそこに足す。

条件で絞るのは抽選（`random_talks`）だけ。既読の整理・本数の集計・書き出しは条件を見ない `all_random_talks` を使う。抽選と同じ関数で既読を整理すると、時間帯の外で起動したときにそのトークの既読が消える。

## Build Commands

- **Primary build**: `pwsh.exe .\build.ps1` または `.\build.bat`
- **Rust build only**: `cd ghost/master && cargo build --release`
- **Format code**: `ghost/master/tools/fmt.sh`（WSL/Unix）または `pwsh ghost/master/tools/fmt.ps1`（Windows）。`cargo fmt` に加え、rustfmt が触れないトーク文字列内部のインデントも正規化する（`tools/normalize_string_indent.py`）
- **Lint code**: `cd ghost/master && cargo clippy`
- **Test**: `cd ghost/master && cargo test`

`.claude/settings.json` の PostToolUse フックが、`.rs` を書き換えるたびに `cargo fmt` を走らせる。

`.githooks/pre-push` は、成果物に影響する差分があるのに `Cargo.toml` のバージョンが上がっていなければ push を止める。

## Development Dependencies

- PowerShell
- Rust/Cargo
- ImageMagick (`magick` command)
- surfaces-mixer: `go install github.com/apxxxxxxe/surfaces-mixer@v0.3.0`

## Code Style

- 2スペースインデント（`ghost/master/rustfmt.toml`）
- `max_width = 220`。日本語トーク文字列を含む行が上限を超えると rustfmt がその文の整形を黙って諦めるため広げてある。代わりに `fn_call_width` などの1行化しきい値を旧デフォルトに固定し、コードが1行に潰れないようにしている
- clippy の type-complexity しきい値は 1000（`ghost/master/clippy.toml`）
- エラー処理と正規表現はマクロに寄せてある

### テスト

グローバル変数（`FLAGS`、`LOAD_STATUS`、`CHAIN_TALK_STATE`、`PENDING_BRANCHES` など）を触るテストは、並列実行で競合する。`events/aitalk.rs` の `test_boot`、`events/mouse.rs` の `test_chain_talk_mechanism`、`events/talk.rs` の `test_branch_talk_flow` のように、**1つのテスト関数にまとめて直列に検証する**。個別に分けると実行のたびに結果が変わる。

## 既知の穴

- `shell/master/old/` は元ゴーストのシェル画像。未追跡のまま置いてある。

## 文書を書くときの言葉遣い

`.claude/` 配下の資料や本ファイルを書く・直すときに守る。トーク本文（サルバニの台詞）の作法とは別。

1. **AI 臭の語法を持ち込まない**。禁止リストと判定の詳細は `.claude/skills/stop-ai-slop-jp/`。とくに紛れ込みやすいのは:
   - **false agency** — モノを主語に人間の動詞をさせる（「温度差が物語る」「課題が浮き彫りになる」）。誰が何をしたかに書き換える。
   - **必殺技造語・翻訳調動詞** — 普通の感想を漢字熟語や直訳調で膨らます（「凝縮」「結実」「示している」「収斂する」）。
   - **二項対比テンプレ** — 「単なる A ではなく B」は直接 B を書く。
2. **比喩を定義語にするなら、初出で普通の言葉に開く**。「手品化」「成形」のように比喩一語で意味を運ばせない。一度ふつうの言葉で説明してから、以降の反復ショートカットとして使う。すでに定義済みで反復が前提の核概念はそのまま使ってよい。
