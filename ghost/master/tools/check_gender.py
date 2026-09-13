"""サルバニの性別を決めつける語が紛れていないか調べる。

サルバニは男でも女でも通るように書く決まりになっている
(.claude/lore/sarvani.md の「性別は決めない」)。トーク本文だけでなく
資料の地の文でも崩れやすいので、機械的に拾う。

使い方（リポジトリルートで実行）:
    python ghost/master/tools/check_gender.py
    python ghost/master/tools/check_gender.py .claude/lore/sarvani.md

引数を省くと CLAUDE.md、.claude/lore の .md、ghost/master/src の .rs を見る。

見落とすもの:
- 「ワタクシ」以外の一人称や、服装・体格の描写のような文脈依存の崩れ
- 規則そのものを説明している節は飛ばすので、そこに紛れた違反は拾えない
どちらも人が読んで判断する。
"""

import io
import os
import re
import sys

# 単独で出たら疑う語
WORDS = [
    "彼は", "彼が", "彼の", "彼を", "彼に", "彼女",
    "男性", "女性", "青年", "少年", "少女",
    "息子", "娘", "紳士", "淑女",
]
# 「女神」「彼女」を除いたうえで残る単独の男・女
BARE = ["男", "女"]

# この見出しの節は規則の説明なので飛ばす
SKIP_HEADING = re.compile(r"^#+\s*(性別は決めない|先に一つ)")


def scan_md(path):
    out = []
    skipping = False
    for i, line in enumerate(io.open(path, encoding="utf-8"), 1):
        if line.startswith("#"):
            skipping = bool(SKIP_HEADING.match(line))
        if skipping:
            continue
        out.append((i, line))
    return out


def scan_rs(path):
    # 文字列リテラルの中だけを見る。識別子やコメントは対象外
    src = io.open(path, encoding="utf-8").read()
    out = []
    for i, line in enumerate(src.split("\n"), 1):
        for lit in re.findall(r'"([^"]*)"', line):
            out.append((i, lit))
    return out


def main():
    if len(sys.argv) > 1:
        targets = sys.argv[1:]
    else:
        targets = [".claude/CLAUDE.md"]
        # .claude/skills は他所から持ってきた文書なので見ない
        for root, _, files in os.walk(".claude/lore"):
            targets += [os.path.join(root, f) for f in files if f.endswith(".md")]
        for root, _, files in os.walk("ghost/master/src"):
            targets += [os.path.join(root, f) for f in files if f.endswith(".rs")]

    hits = 0
    for path in sorted(targets):
        lines = scan_rs(path) if path.endswith(".rs") else scan_md(path)
        for i, text in lines:
            for w in WORDS:
                if w in text:
                    print("%s:%d  %s" % (path, i, w))
                    hits += 1
            bare = text.replace("女神", "").replace("彼女", "")
            for w in BARE:
                if w in bare:
                    print("%s:%d  単独の「%s」  %s" % (path, i, w, text.strip()[:50]))
                    hits += 1
    print("---- %d file(s), %d hit(s)" % (len(targets), hits))
    return 1 if hits else 0


if __name__ == "__main__":
    sys.exit(main())
