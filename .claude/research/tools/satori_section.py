r"""里々の辞書から、名前が正規表現に合う節（＊）の本文を出す。質問の回答を読むのに使う。

選択肢の ID と節の名前が違うゴーストが多いので、まずメニュー側で \q[ラベル,ID] の ID を確かめる。

usage: PYTHONIOENCODING=utf-8 python satori_section.py <ghost_dir>/<ゴースト名> <正規表現> [最大件数=5]
例:    python satori_section.py "$G/Lakritze" '^＊質問(レンジ|連絡先)'
"""
import os, sys, re

gm = os.path.join(sys.argv[1], 'ghost', 'master')
pat = re.compile(sys.argv[2])
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 5


def read(p):
  b = open(p, 'rb').read()
  for enc in ('utf-8', 'cp932'):
    try:
      return b.decode(enc)
    except UnicodeDecodeError:
      pass
  return b.decode('cp932', 'replace')


n = 0
for dp, _, fn in os.walk(gm):
  for f in sorted(fn):
    if not f.lower().endswith(('.txt', '.dic')):
      continue
    for b in re.split(r'(?m)^(?=[＊＠])', read(os.path.join(dp, f))):
      if b.startswith('＊') and pat.search(b.split('\n', 1)[0]):
        print(f'--- {os.path.relpath(os.path.join(dp, f), gm)}')
        print(b.strip()[:1500])
        n += 1
        if n >= limit:
          sys.exit()
