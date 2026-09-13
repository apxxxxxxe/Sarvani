#!/usr/bin/env python3
"""トークの締めを並べて、終わり方が偏っていないかを見る。

使い方（リポジトリルートで実行）:
  cd ghost/master && cargo run --bin dump_talks
  python ghost/master/tools/list_endings.py ghost/master/all_talks.txt
  python ghost/master/tools/list_endings.py ghost/master/all_talks.txt --all

既定ではランダムトーク（見出しが「分類/名前」のもの）だけを見る。
--all を付けると初回起動や質問トークも含める。

一本ずつ書いていると、締め方の偏りには気づけない。ここでは合否を出さず、
締めの二行と最後の表情を並べ、内訳を数えるだけにしてある。
同じ締めが続いていないか、ポーズ2が山場の外にまで出ていないかを目で確かめる。
"""

import os
import re
import sys
from collections import Counter

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from check_surfaces import PARTS, RESPONSE, part_names, read  # noqa: E402

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8")

SURFACE_RE = re.compile(r"hr?(\d{7})")
TAG_RE = re.compile(r"\\[!_a-z]+\[[^\]]*\]|\\_?[a-z01*\-]")
TAIL_LINES = 2


def pose_names(source):
    """response.rs のコメント「ポーズ 0=非表示 1=通常立ち ...」を読む"""
    m = re.search(r"ポーズ\s+((?:\d=\S+\s*)+)", source)
    return {int(d): name for d, name in re.findall(r"(\d)=(\S+)", m.group(1))} if m else {}


def describe(code, tables, poses):
    n = int(code)
    pose = (n // 1000000) % 10
    if n == 1000000 or pose == 0:
        return "非表示", {}
    parts = {}
    for part, fn, digit_of in PARTS:
        explicit, default = tables[fn]
        parts[part] = explicit.get(digit_of(n), default)
    label = "%s %s・%s・%s" % (poses.get(pose, "ポーズ%d" % pose), parts["目"], parts["口"], parts["腕"])
    return label, dict(parts, ポーズ=poses.get(pose, str(pose)))


def talks(text, include_all):
    for m in re.finditer(r"^# (.+)\n(.*)$", text, re.M):
        name, body = m.group(1), m.group(2)
        if include_all or "/" in name:
            yield name, body


def last_speaker(body):
    """最後に文字を置いたのが本体か \\1 か。サーフェスコードは \\0 に戻す"""
    scope = "0"
    speaker = "0"
    for token in re.split(r"(\\[01]|hr?\d{7})", body):
        if token in ("\\0", "\\1"):
            scope = token[1]
        elif SURFACE_RE.fullmatch(token):
            scope = "0"
        elif TAG_RE.sub("", token).strip():
            speaker = scope
    return "\\" + speaker


def tail(body):
    page = re.split(r"\\x", body)[-1]
    page = SURFACE_RE.sub("", page).replace("\\n[half]", "\\n")
    lines = [TAG_RE.sub("", line).strip() for line in page.split("\\n")]
    return [line for line in lines if line][-TAIL_LINES:]


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    include_all = "--all" in sys.argv
    target = args[0] if args else "ghost/master/all_talks.txt"

    source = read(RESPONSE)
    tables, poses = part_names(source), pose_names(source)

    count = 0
    with_pose2 = 0
    stats = {key: Counter() for key in ("ポーズ", "目", "口", "腕", "話者", "句読点")}

    for name, body in talks(read(target), include_all):
        codes = SURFACE_RE.findall(body)
        if not codes:
            continue
        count += 1
        if any(int(c) // 1000000 % 10 == 2 for c in codes):
            with_pose2 += 1

        label, parts = describe(codes[-1], tables, poses)
        for key, value in parts.items():
            if key in stats:
                stats[key][value] += 1
        speaker = last_speaker(body)
        stats["話者"][speaker] += 1
        lines = tail(body)
        stats["句読点"][lines[-1][-1] if lines else "?"] += 1

        print("■ %s" % name)
        print("  最後の顔: h%s  %s  / 最後の話者 %s" % (codes[-1], label, speaker))
        for line in lines:
            print("  │ %s" % line)

    print()
    print("---- %d 本" % count)
    print("ポーズ2を含む: %d 本" % with_pose2)
    for key, counter in stats.items():
        print("締めの%s: %s" % (key, "  ".join("%s %d" % kv for kv in counter.most_common())))


if __name__ == "__main__":
    main()
