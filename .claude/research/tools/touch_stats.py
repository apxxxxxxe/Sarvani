r"""extract_touch.py の出力から、触り反応の作りを数える。

数えるもの（どれも正規表現による目安。誤検出はある）:
  部位    : ＊0xxxなでられ/つつかれ/ころころ の当たり判定名の数（\0 側だけ）
  本数    : 台詞を持つ節と単語群の行の合計（振り分けだけの節は数えない）
  中央行  : 台詞を持つ節の行数の中央値（単語群は1行1本として数えない）
  回数    : 触りの節で ＄X＝（X）＋… の加算をしている
  段階    : 見出しや振り分けの条件に 好感度・進行度・親密度・好意・友好・恋人・回数 がある
  場面    : 条件に 現在地・状態・シェル・服・上映中・時間 がある
  拒否    : 「触り不可」「拒否」「拒絶」「禁止」の名前の節がある、または「お触り」の語がある
  選択肢  : 触り反応の中に \q[ か ＿ の選択肢がある（メニュー本体の節は除く）
  地の文  : φ（…φ） か 行頭が「（」で数字でない行（ユーザ側の描写）がある
  沈黙    : 台詞が … と待ちだけの行がある

usage: PYTHONIOENCODING=utf-8 python touch_stats.py <touches_dir>
"""
import os, sys, re, glob, statistics

d = sys.argv[1]
TOUCH = re.compile(r'^＊([01])(.*?)(なでられ|つつかれ|ころころ)')
SAY = re.compile(r'^(：|\\p\[|\\[01]|（[０-９0-9－\-]+）)', re.M)
STAGE = re.compile(r'好感度|進行度|親密|好意|友好|恋人|回数|カウント|段階|経験値')
SCENE = re.compile(r'現在地|状態|シェル|現在の服|上映中|現在時|時間帯')
REFUSE = re.compile(r'触り不可|拒否|拒絶|禁止|お触り|触んな')
CHOICE = re.compile(r'\\q\[|^＿|^□\[|^☆\\q', re.M)
NARR = re.compile(r'φ（|^：?（[^０-９0-9－\-（）][^）]*）\s*$', re.M)
SILENT = re.compile(r'^：?(（[０-９0-9]+）)?[…\.。ｗ０-９0-9\\w\[\]_]+$', re.M)

rows = []
for p in sorted(glob.glob(os.path.join(d, '*.txt'))):
    g = os.path.basename(p)[:-4]
    t = open(p, encoding='utf-8').read()
    blocks = [b for b in t.split('\n=== ') if b.strip()]
    parts, count, stage, scene, choice, lines = set(), False, False, False, False, []
    n = 0
    for b in blocks:
        head, _, body = b.partition('\n')
        name = head[head.find('[') + 1:head.rfind(']')]
        m = TOUCH.match(name)
        if m and m.group(1) == '0' and m.group(2):
            parts.add(m.group(2).lower())
        if m and re.search(r'^＄[^＝=\t]+[＝=]（[^）]+）[＋+]', body, re.M):
            count = True
        cond = head + '\n' + '\n'.join(l for l in body.split('\n') if l.startswith('＞'))
        stage |= bool(STAGE.search(cond))
        # 当たり判定のない所のダブルクリック（メニュー）は選択肢に数えない
        if not re.match(r'^＊[01]?つつかれ$', name) and 'メニュ' not in name:
            choice |= bool(CHOICE.search(body))
        scene |= bool(SCENE.search(cond))
        if name.startswith('＠'):
            n += len([l for l in body.split('\n') if l.strip() and not l.startswith('＃')])
        elif SAY.search(body):
            n += 1
            lines.append(len([l for l in body.split('\n') if l.strip() and not l.startswith(('＃', '＄', '＞', '\\_a'))]))
    rows.append((g, len(parts), n, statistics.median(lines) if lines else 0, count, stage, scene,
                 bool(REFUSE.search(t)), choice, bool(NARR.search(t)), bool(SILENT.search(t))))

hdr = ('ゴースト', '部位', '本数', '中央行', '回数', '段階', '場面', '拒否', '選択肢', '地の文', '沈黙')
print('| ' + ' | '.join(hdr) + ' |')
print('|' + '---|' * len(hdr))
for r in rows:
    print('| ' + ' | '.join(('✓' if v is True else '' if v is False else str(v)) for v in r) + ' |')
tot = len(rows)
print()
for i, h in enumerate(hdr[4:], 4):
    print(f'{h}: {sum(1 for r in rows if r[i])}/{tot}')
