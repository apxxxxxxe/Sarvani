#!/usr/bin/env python3
"""トークがデフォルトバルーンに収まっているかを見る。

デフォルトバルーンは \\0 側が1行24文字の10行、\\1 側が1行24文字の5行。
はみ出した分はスクロールになるので、1トークはこの中に収めたい。

使い方:
  cd ghost/master && cargo run --bin dump_talks
  python ghost/master/tools/check_balloon.py ghost/master/all_talks.txt

行数の数え方:
  - 半角は0.5文字、全角は1文字として24文字で折り返す
  - \\n は1行、\\n[half] は半行の送り
  - \\x と \\c でバルーンが空になるので、そこから数え直す（ページごとの最大を見る）
  - \\x の後はスコープが \\0 に戻る
"""

import re
import sys
import unicodedata

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8")

WIDTH = 24
LIMIT = {0: 10, 1: 5}

# 高さに関わらないタグ。長いものから順に消す
TAG_RE = re.compile(
    r"""
    \\!\[[^\]]*\]      # \![open,inputbox,...] など
  | \\_l\[[^\]]*\]
  | \\__w\[[^\]]*\]
  | \\_w\[[^\]]*\]
  | \\f\[[^\]]*\]
  | \\s\[[^\]]*\]
  | \\b\[[^\]]*\]
  | \\q\[[^\]]*\]
  | \\__q(\[[^\]]*\])?
  | \\_q
  | \\_a(\[[^\]]*\])?
  | \\[Cet*\-]         # \C \e \t \* \-
    """,
    re.VERBOSE,
)

SCOPE_RE = re.compile(r"\\([01])|\\p\[(\d+)\]")

# サーフェスコード。generate_bind_script が必ず \0 付きのスクリプトに展開するので、
# 書いた時点でスコープが \1 でも、ここで本体側へ戻る
SURFACE_RE = re.compile(r"hr?\d{7}")


def char_width(c):
    return 1.0 if unicodedata.east_asian_width(c) in "WFA" else 0.5


class Balloon:
    """1つのスコープの積み上がり具合"""

    def __init__(self):
        self.height = 1.0  # いま何行目にいるか
        self.column = 0.0
        self.max_height = 0.0
        self.used = False
        self.used_height = 0.0  # 文字が置かれた最後の行の高さ
        self.pages = [[[1.0, ""]]]  # ページごとの [その行までの高さ, 本文]

    def feed(self, advance):
        self.height += advance
        self.column = 0.0
        self.pages[-1].append([self.height, ""])

    def put(self, c):
        self.used = True
        w = char_width(c)
        if self.column + w > WIDTH:
            self.feed(1.0)
        self.column += w
        self.used_height = self.height
        self.pages[-1][-1][1] += c

    def flush(self):
        # 末尾の空行は何も表示しないので数えない
        self.max_height = max(self.max_height, self.used_height)

    def clear(self):
        self.flush()
        self.height = 1.0
        self.column = 0.0
        self.used_height = 0.0
        self.pages.append([[1.0, ""]])


def measure(script):
    balloons = {0: Balloon(), 1: Balloon()}
    scope = 0
    pages = 1
    i = 0
    n = len(script)

    while i < n:
        rest = script[i:]

        m = SCOPE_RE.match(rest)
        if m:
            scope = int(m.group(1) or m.group(2))
            i += m.end()
            continue

        m = SURFACE_RE.match(rest)
        if m:
            scope = 0
            i += m.end()
            continue

        if rest.startswith("\\n[half]"):
            balloons[scope].feed(0.5)
            i += len("\\n[half]")
            continue

        if rest.startswith("\\n"):
            balloons[scope].feed(1.0)
            i += 2
            continue

        if rest.startswith("\\x[noclear]"):
            i += len("\\x[noclear]")
            continue

        if rest.startswith("\\x") or rest.startswith("\\c"):
            for b in balloons.values():
                b.clear()
            if rest.startswith("\\x"):
                scope = 0
                pages += 1
            i += 2
            continue

        m = TAG_RE.match(rest)
        if m:
            i += m.end()
            continue

        balloons[scope].put(script[i])
        i += 1

    for b in balloons.values():
        b.flush()
    return balloons, pages


def show(name, script):
    """折り返した結果を並べて見せる。書き直すときの物差し"""
    balloons, _ = measure(script)
    print("# " + name)
    for scope in (0, 1):
        b = balloons[scope]
        if not b.used:
            continue
        for page_no, rows in enumerate(b.pages, start=1):
            if all(text == "" for _, text in rows):
                continue
            label = "\\%d" % scope
            if len(b.pages) > 1:
                label += " page%d" % page_no
            used = max(h for h, text in rows if text)
            print("%s  (%g行 / %d行)" % (label, used, LIMIT[scope]))
            for height, text in rows:
                if not text and height > used:
                    continue
                over = "!" if height > LIMIT[scope] else " "
                print("  %s%5.1f | %s" % (over, height, text))
        print()


def main():
    args = sys.argv[1:]
    target = None
    if "--show" in args:
        i = args.index("--show")
        target = args[i + 1]
        args = args[:i] + args[i + 2 :]
    path = args[0] if args else "ghost/master/all_talks.txt"
    with open(path, encoding="utf-8") as f:
        lines = [line.rstrip("\n") for line in f]

    entries = []
    name = "(no name)"
    for line in lines:
        if line.startswith("# "):
            name = line[2:]
        elif line.strip():
            entries.append((name, line))

    if target is not None:
        hit = [(n, s) for n, s in entries if target in n]
        if not hit:
            print("no talk matches: " + target)
            return
        for name, script in hit:
            show(name, script)
        return

    over = 0
    header = "talk".ljust(30) + "sakura".rjust(9) + "kero".rjust(9) + "  pages"
    print(header)
    print("-" * len(header))
    for name, script in entries:
        balloons, pages = measure(script)
        cells = []
        bad = False
        for scope in (0, 1):
            b = balloons[scope]
            if not b.used:
                cells.append("-")
                continue
            limit = LIMIT[scope]
            if b.max_height > limit:
                bad = True
                cells.append("%g/%d !" % (b.max_height, limit))
            else:
                cells.append("%g/%d" % (b.max_height, limit))
        if bad:
            over += 1
        print(name.ljust(30) + cells[0].rjust(9) + cells[1].rjust(9) + "  " + str(pages))

    print("-" * len(header))
    print("---- %d talk(s), %d over the balloon" % (len(entries), over))


if __name__ == "__main__":
    main()
