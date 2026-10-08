# 参考ゴースト（Favorite）の調査メモ

作者の手元にある参考ゴースト群 `C:\Users\user1\Ukagaka\Ukagaka-Ghost\Favorite`（55体）を調べた記録。同じ調査を繰り返さないために残す。

| ファイル | 中身 |
|---|---|
| [classification.md](classification.md) | 全ゴーストの一覧。コンビかソロか、\1 が何を表すか、トークの本数と長さ、一言の特徴 |
| [random_talk_types.md](random_talk_types.md) | ランダムトークの型。コンビとソロに分けて、型ごとに用例を挙げ、サルバニに使えるかを書いた |
| [touch_reactions.md](touch_reactions.md) | 触り反応の作り。仕組み（メニュー、回数、場面、遊び）、返しの型、表情と間、サルバニの voice との突き合わせ |
| [room08_nurse_touch.md](room08_nurse_touch.md) | room08 の看護師（\1）の触り反応を全部読んだもの。作者が触り反応の見直しでいちばんの参考にしたいとしたもの |
| `talks/` | 抜き出したランダムトーク本文。他作者の著作物なので gitignore してある。無ければ下の手順で作り直す |
| `touches/` | 抜き出した触り反応の本文。`talks/` と同じく gitignore |

話法そのものの結論（短く一発で、素直な熱、仕掛けを足さない）は [sarvani_voice.md](../../lore/sarvani_voice.md) の「ランダムトークは人物の隙を見せる場」に移してある。ここは調べた事実と型の一覧を置く場所。

## 道具

`.claude/research/tools/` に置いた。どれも `PYTHONIOENCODING=utf-8` を付けて動かす（付けないと Windows の端末で文字化けする）。

```
F='C:\Users\user1\Ukagaka\Ukagaka-Ghost\Favorite'
python .claude/research/tools/extract_satori.py "$F" .claude/research/favorite_ghosts/talks   # 里々のランダムトークを抜き出す
python .claude/research/tools/kero_usage.py .claude/research/favorite_ghosts/talks           # \1 の使われ方を数える
python .claude/research/tools/sample_talks.py .claude/research/favorite_ghosts/talks 5 1 chinatown4649 rabi   # 無作為に抜いて読む
python .claude/research/tools/survey_ghosts.py "$F"    # SHIORI と sakura/kero の名前
python .claude/research/tools/kero_images.py "$F"      # シェルの surface10〜99 に絵があるか（ImageMagick）
python .claude/research/tools/extract_touch.py "$F" .claude/research/favorite_ghosts/touches   # 触り反応を飛び先まで辿って抜き出す
python .claude/research/tools/touch_stats.py .claude/research/favorite_ghosts/touches           # 部位・本数・回数や段階の有無を表にする
python .claude/research/tools/touch_faces.py .claude/research/favorite_ghosts/touches chinatown4649   # 1本ごとの表情の変え方と間を数える
```

## 調べ方で分かったこと（次に調べる人へ）

- **55体中51体が里々**（Yumemi_1st は descript に shiori の行が無いが、辞書は里々の書式）。ほかは YAYA（HamirLore、lnx_ukaclock）と、華和梨系の辞書を持つ 54・mizore。Haine は作者自身の旧作の里々版。
- **里々のトークは \1 が喋る状態で始まる。** 行頭の「：」で \0 と \1 を行き来する（里々 docs「スコープとサーフェス」）。だから多くのゴーストはトークを「：」で書き出して \0 に渡している。これを知らずに数えると \0 と \1 が逆になる。
- **ランダムトークの置き場はゴーストごとに違う。** 名前なし「＊」に直接書くもの、名前なし「＊」から「＞ランダムトーク」「＞トーク」のように名前つき節へ飛ばすもの、単語群を挟むもの（HighColorMulti の `（トーク種別）`）がある。`extract_satori.py` は3通りとも拾う。辿れなかったときは同名の節が5本以上ある名前を全部拾うので、触り反応などが混じることがある。見出しの `[節の名前]` で見分ける。
- **シェルの画像では、コンビかどうかを決められない。** surface10 に絵があっても、\0 の差分パーツや小物に番号を使っているだけのことがある（maidSM は66枚あるがどれも小さな部品）。逆に、\1 に絵がなくてもトークの中で \1 が喋るゴーストもある。分類は**ランダムトークの中で \1 が自分の表情を持って喋るか**で決めた。
- 未読のもの: 54・mizore（華和梨系で辞書の形式が違う）、HamirLore（YAYA。ランダムトークは \0 だけの漢詩の書き下しなので、ソロ扱いでよい）、SilentBisque（\![...] でバルーン外に文字を出す特殊な作り）、desktopmmkk（トークが別の節を呼ぶだけで、中身まで辿れていない）、lnx_ukaclock（時計なので対象外）。

## 調べた日

- 2026-10-06: 触り反応（touch_reactions.md）、room08 看護師の触り反応（room08_nurse_touch.md）
- 2026-10-05: 一覧と分類、ランダムトークの型（このフォルダの初版）
- それ以前（同日の前半）: 里々ゴースト31体のトークの長さを数え、「短く一発で」の結論を出した。数字は voice の「長さは混ぜる」にある。その後、2行の案ばかりになったので長さを混ぜる方針に改めた
