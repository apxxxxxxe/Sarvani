r"""里々ゴーストの辞書から、触り反応を抜き出す。

里々はマウスのイベントを次の名前の節に振る（里々 docs「デフォルトの動作」）。
  OnMouseDoubleClick → ＊(スコープ)(当たり判定名)つつかれ   例: ＊0Headつつかれ
  OnMouseMove        → ＊(スコープ)(当たり判定名)なでられ   （＄なでられ反応回数 を超えたら）
  OnMouseWheel       → ＊(スコープ)(当たり判定名)ころころ
＄なでられ時実行イベント を使うゴーストは ＊なでられ時の反応 に書くので、それも拾う。
OnMouse で始まる節を自前で書いているゴーストもあるので、それは別に数える。
多くのゴーストは触りの節を「＞頭なでられトーク」のような振り分けにして、本文を別名の節に置く。
そこで「＞名前」と「（名前）」を3段まで辿り、飛び先の節（＊）と単語群（＠）も拾う。
メニュー・ランダムトーク・On〜 への飛び先は辿らない（触り反応ではないので）。

usage: PYTHONIOENCODING=utf-8 python extract_touch.py <favorite_dir> <out_dir>
出力: <out_dir>/<ゴースト名>.txt（節ごとに「=== 辞書ファイル名 [節の名前]」の見出し）と、標準出力に集計
"""
import os, sys, glob, re, collections

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

TOUCH = re.compile(r'^([0-9]?)(.*?)(つつかれ|なでられ|ころころ)$')

SKIP = re.compile(r'メニュー|めにゅ|ランダムトーク|^On|コミュニケート|呼び出し|話を聞く')
JUMP = re.compile(r'^＞([^	【]+)')
CALL = re.compile(r'（([^（）]+)）')

def sections(master):
    for p in sorted(glob.glob(os.path.join(master, '**', 'dic*.txt'), recursive=True)):
        cur = None
        for line in read(p).replace('\r\n', '\n').split('\n'):
            if line.startswith('＊') or line.startswith('＠'):
                if cur is not None:
                    yield cur
                parts = re.split(r'\t|【タブ】', line[1:], maxsplit=1)
                name = parts[0].strip()
                cond = parts[1].strip() if len(parts) > 1 else ''
                cur = (os.path.relpath(p, master), line[0] + name, [], cond)
                continue
            if cur is not None:
                cur[2].append(line)
        if cur is not None:
            yield cur

for g in sorted(os.listdir(root)):
    m = os.path.join(root, g, 'ghost', 'master')
    if not os.path.isdir(m):
        continue
    counts = collections.Counter()
    onmouse = collections.Counter()
    by_name = collections.defaultdict(list)
    roots = []
    for f, name, lines, cond in sections(m):
        body = '\n'.join(lines).strip()
        if not body:
            continue
        # 見出しの条件（＊名前<TAB>（進行度）＞＝３）は表示にだけ残す
        disp = name + ('\t' + cond if cond else '')
        by_name[name[1:]].append((f, disp, body))
        if name[0] != '＊':
            continue
        if TOUCH.match(name[1:]) or name[1:] == 'なでられ時の反応':
            roots.append((f, disp, body))
            counts[name[1:]] += 1
        elif name[1:].startswith('OnMouse'):
            roots.append((f, disp, body))
            onmouse[name[1:]] += 1
    picked = list(roots)
    seen = set(n[1:].split('\t')[0] for _, n, _ in roots)
    frontier = roots
    for depth in range(3):
        nxt = []
        for f, name, body in frontier:
            targets = []
            for l in body.split('\n'):
                j = JUMP.match(l)
                if j:
                    targets.append(j.group(1).strip())
                targets += [c.strip() for c in CALL.findall(l)]
            for t in targets:
                if t in seen or t not in by_name or SKIP.search(t):
                    continue
                seen.add(t)
                nxt.extend(by_name[t])
        picked.extend(nxt)
        frontier = nxt
    if not picked:
        print(f'{g}\t-')
        continue
    with open(os.path.join(out, g + '.txt'), 'w', encoding='utf-8') as w:
        for f, name, body in picked:
            w.write(f'=== {f} [{name}]\n{body}\n\n')
    c = ' '.join(f'{k}:{v}' for k, v in sorted(counts.items()))
    o = ' '.join(f'{k}:{v}' for k, v in sorted(onmouse.items()))
    print(f'{g}\t{sum(counts.values())}\t{c}\t{o}')
