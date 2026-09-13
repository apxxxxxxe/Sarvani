"""トーク本文の7桁サーフェスコードが、シェルの絵として成立するかを見る。

使い方（リポジトリルートで実行）:
    python ghost/master/tools/check_surfaces.py ghost/master/src/events/talk/randomtalk.rs
    python ghost/master/tools/check_surfaces.py ghost/master/all_talks.txt

コードが絵になるまでの道筋をそのまま辿る。

    7桁コードの各桁
      → 部品名（response.rs の eye_name などの対応表）
      → bindgroup 番号（descript.txt の bindgroup*.name）
      → 素体サーフェスの bind アニメーション（surfaces.txt）

どこで切れたかを述べる。判定はすべてシェルとソースから読むので、
シェルの部品が増えたり bindgroup 番号が変わったりしても追従する。

- NAME NOT IN SHELL: その部品名の bindgroup が descript.txt にない。bind が不発
- NOT IN POSE: bindgroup はあるが、そのポーズの素体に bind アニメーションがない。
  そのパーツだけ消える。addid 側で成立している場合は OK と出す
- FALLBACK: 対応表に無い桁。response.rs の既定の部品に落ちる。
  壊れはしないが、書いた人の意図とは違うはず
"""
import io
import re
import sys

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8")

RESPONSE = "ghost/master/src/system/response.rs"
DESCRIPT = "shell/master/descript.txt"
SURFACES = "shell/master/surfaces.txt"

# 桁の並びは response.rs の generate_bind_script と揃えてある。
# (部位名, 対応表の関数名, 桁の取り出し方)
PARTS = [
    ("顔色", "face_color_name", lambda n: (n // 100000) % 10),
    ("眉", "eyebrow_name", lambda n: (n // 10000) % 10),
    ("腕", "arm_name", lambda n: (n // 1000) % 10),
    ("口", "mouth_name", lambda n: (n // 100) % 10),
    ("目", "eye_name", lambda n: n % 100),
]


def read(path):
    return io.open(path, encoding="utf-8").read()


def part_names(source):
    """response.rs の *_name 関数から、桁と部品名の対応を読む"""
    tables = {}
    for m in re.finditer(r"fn (\w+_name)\(code: i32\) -> &'static str \{(.*?)\n\}", source, re.S):
        body = m.group(2)
        explicit = {int(d): name for d, name in re.findall(r"(\d+) => \"([^\"]+)\"", body)}
        default = re.search(r"_ => \"([^\"]+)\"", body)
        tables[m.group(1)] = (explicit, default.group(1) if default else None)
    return tables


def bindgroups(descript):
    """部品名から bindgroup 番号へ。addid も拾う"""
    names = {}
    for gid, part, name in re.findall(r"bindgroup(\d+)\.name,([^,\r\n]+),([^\r\n]+)", descript):
        names[(part.strip(), name.strip())] = gid
    addid = dict(re.findall(r"bindgroup(\d+)\.addid,(\d+)", descript))
    return names, addid


def pose_animations(surfaces):
    """ポーズごとの素体サーフェスが持つ bind アニメーション番号"""
    blocks = {}
    for m in re.finditer(r"^surface(\d+)\s*\{(.*?)^\}", surfaces, re.S | re.M):
        blocks[int(m.group(1))] = set(re.findall(r"animation(\d+)\.interval", m.group(2)))
    # base_surface(pose) = pose * 1_000_000 + 100
    return {
        surface // 1000000: anims
        for surface, anims in blocks.items()
        if surface >= 1000000 and surface % 1000000 == 100
    }


def main():
    tables = part_names(read(RESPONSE))
    names, addid = bindgroups(read(DESCRIPT))
    poses = pose_animations(read(SURFACES))

    target = sys.argv[1] if len(sys.argv) > 1 else "ghost/master/all_talks.txt"
    codes = sorted(set(re.findall(r"hr?(\d{7})", read(target))))

    bad = 0
    for code in codes:
        n = int(code)
        pose = (n // 1000000) % 10

        if n == 1000000 or pose == 0:
            print("h%s  hidden" % code)
            continue
        if pose not in poses:
            print("h%s  NG: ポーズ%d の素体サーフェスがない" % (code, pose))
            bad += 1
            continue

        have = poses[pose]
        # パーツ未分割のポーズは bind を持たない。一枚絵がそのまま出るので、
        # 下位桁が何であっても構わない（h3000000 のような書き方も有効）
        if not (set(names.values()) | set(addid.values())) & have:
            print("h%s  pose%d  一枚絵（bind なし）" % (code, pose))
            continue

        problems = []
        for part, fn, digit_of in PARTS:
            digit = digit_of(n)
            explicit, default = tables[fn]
            # 桁1（目は01）は既定の部品を指す書き方として正しい
            if digit in explicit:
                name = explicit[digit]
            elif digit == 1:
                name = default
            else:
                problems.append("FALLBACK: %s%d → %s" % (part, digit, default))
                name = default

            gid = names.get((part, name))
            if gid is None:
                problems.append("NAME NOT IN SHELL: %s の %s" % (part, name))
                continue
            if gid not in have and addid.get(gid) not in have:
                problems.append("NOT IN POSE: %s %s (bindgroup%s)" % (part, name, gid))

        if problems:
            print("h%s  pose%d  %s" % (code, pose, " / ".join(problems)))
            bad += 1
        else:
            print("h%s  pose%d  OK" % (code, pose))

    print("---- %d code(s), %d problem(s)" % (len(codes), bad))


if __name__ == "__main__":
    main()
