r"""里々ゴーストの辞書から、ランダムトークを抜き出す。

ランダムトークの置き方はゴーストごとに二通りある。
1. 名前なし「＊」の節にそのまま書く
2. 名前なし「＊」は振り分けだけで、「＞ランダムトーク夢」のように飛んだ先の名前つき節に書く
   （同じ名前の節を何本も並べると、里々がそこから抽選する）
2 を拾うため、名前なし節から「＞名前」（名前なし節の中では「（名前）」の呼び出しも）を3段まで辿り、
同名の節が3本以上ある名前をトークの束とみなす。
それでも5本未満なら、単語群や変数を挟んで辿れない振り分けとみなし、同名の節が5本以上ある名前を全部拾う
（このときはメニューや触り反応が混じりうる。見出しの節の名前で見分ける）。
本文が「＞」や「＄」だけの振り分け節は捨てる。

usage: PYTHONIOENCODING=utf-8 python extract_satori.py <favorite_dir> <out_dir>
出力: <out_dir>/<ゴースト名>.txt。トークごとに「=== 辞書ファイル名 [節の名前]」の見出しを付ける
"""
import os, sys, glob, re, statistics, collections

root, out = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)

def read(p):
    b = open(p, 'rb').read()
    for enc in ('utf-8-sig', 'cp932'):
        try:
            return b.decode(enc)
        except UnicodeDecodeError:
            pass
    return b.decode('cp932', errors='replace')

JUMP = re.compile(r'^＞([^\t【]+)')
CALL = re.compile(r'（([^（）]+)）')

def sections(master):
    secs = []  # (file, name, lines)
    for p in sorted(glob.glob(os.path.join(master, '**', 'dic*.txt'), recursive=True)):
        cur = None
        for line in read(p).replace('\r\n', '\n').split('\n'):
            if line.startswith('＊') or line.startswith('＠'):
                if cur is not None:
                    secs.append(cur)
                name = re.split(r'\t|【タブ】', line[1:])[0].strip()
                cur = (os.path.basename(p), name, []) if line.startswith('＊') else None
                continue
            if cur is not None:
                cur[2].append(line)
        if cur is not None:
            secs.append(cur)
    return secs

def is_talk(lines):
    body = [l for l in lines if l.strip() and not l.startswith('＃') and not l.startswith('＞') and not l.startswith('＄')]
    return body

summary = []
for g in sorted(os.listdir(root)):
    m = os.path.join(root, g, 'ghost', 'master')
    if not os.path.isdir(m):
        continue
    secs = sections(m)
    if not secs:
        continue
    by_name = collections.defaultdict(list)
    for s in secs:
        by_name[s[1]].append(s)
    picked, seen = [], set()
    frontier = [s for s in by_name.get('', [])]
    for depth in range(4):
        nxt = []
        for f, name, lines in frontier:
            for l in lines:
                targets = [j.group(1).strip() for j in [JUMP.match(l)] if j]
                if depth == 0:
                    targets += [c.strip() for c in CALL.findall(l)]
                for t in targets:
                    if t and t not in seen and t in by_name and not t.startswith('On'):
                        seen.add(t)
                        nxt.extend(by_name[t])
        frontier = nxt
    pool_names = [''] + [n for n in seen if len(by_name[n]) >= 3]
    for n in pool_names:
        for f, name, lines in by_name.get(n, []):
            body = is_talk(lines)
            if body:
                picked.append((f, name, body))
    if len(picked) < 5:
        # 振り分けが単語群や変数を挟んでいて辿れないとき。同名の節が5本以上ある名前を全部トークの束とみなす
        for n, ss in by_name.items():
            if n and len(ss) >= 5 and not n.startswith('On') and n not in pool_names:
                for f, name, lines in ss:
                    body = is_talk(lines)
                    if body:
                        picked.append((f, name, body))
    if not picked:
        continue
    lens = [len(b) for _, _, b in picked]
    summary.append((g, len(picked), statistics.median(lens), max(lens)))
    with open(os.path.join(out, g + '.txt'), 'w', encoding='utf-8') as fo:
        for f, name, b in picked:
            fo.write('=== %s [%s]\n%s\n\n' % (f, name, '\n'.join(b)))

for s in sorted(summary, key=lambda x: -x[1]):
    print('%-28s talks=%4d median_lines=%4.1f max=%d' % s)
