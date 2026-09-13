r"""extract_touch.py の出力から、触り反応1本ごとの表情の変え方と間を数える。

数えるもの:
  表情2回以上 : 1本のなかでサーフェス指定（（１２）や \s[12]）が2つ以上ある本の割合
  「…」入り   : 「…」を含む本の割合
  問いで終わる : 最後が「？」で終わる本の割合
  名前呼び     : （ユーザ名）（ユーザ呼称）を含む本の割合
単語群（＠）は1行1本、名前つき節は1節1本として数える。

usage: python touch_faces.py <touches_dir> <ゴースト名>...
"""
import sys, re, os, statistics
sys.stdout.reconfigure(encoding='utf-8')
d = sys.argv[1]
SURF = re.compile(r'（[０-９0-9]+）|\\s\[\d+\]')
for g in sys.argv[2:]:
    t = open(os.path.join(d, g + '.txt'), encoding='utf-8').read()
    talks = []
    for b in t.split('\n=== '):
        head, _, body = b.partition('\n')
        if '[＠' in head:
            talks += [l for l in body.split('\n') if SURF.search(l)]
        elif re.search(r'^：', body, re.M) and SURF.search(body):
            talks.append(body)
    n = len(talks)
    faces = [len(SURF.findall(x)) for x in talks]
    multi = sum(1 for f in faces if f >= 2)
    ell = sum(1 for x in talks if '…' in x)
    q = sum(1 for x in talks if re.search(r'[？?]\s*(ｗ\d|\\w\d|\\_w\[\d+\])*\s*$', x.strip()))
    name = sum(1 for x in talks if '（ユーザ名）' in x or '（ユーザ呼称）' in x)
    pct = lambda k: k * 100 // max(n, 1)
    print(f'{g}: 本数{n} 表情2回以上{multi}({pct(multi)}%) 表情中央{statistics.median(faces) if faces else 0} '
          f'「…」入り{pct(ell)}% 問いで終わる{pct(q)}% 名前呼び{pct(name)}%')
