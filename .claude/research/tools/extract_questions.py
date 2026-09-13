r"""ゴーストの辞書から、質問メニュー（ユーザがゴーストに尋ねる一覧）の選択肢を抜き出す。

辞書を節（里々の ＊／＠、YAYA の関数）に分け、選択肢を4つ以上持ち、見出しか冒頭に
「質問」「聞く」「尋ねる」「Q&A」などがある節を質問メニューとみなす。選択肢は
\q[ラベル,ID]・\__q[...]・行頭の ＿ラベル・\_a[...]ラベル\_a から拾う。

拾えないもの:
- 選択肢を関数で組み立てる YAYA（URE_2・railway の「100の質問」など）
- 単語群からランダムに出す選択肢（「（想定問）」のように括弧のまま残る）
設定画面・アンケート・ゴースト側からユーザへの質問も混ざるので、見出しで見分ける。

usage: PYTHONIOENCODING=utf-8 python extract_questions.py <ghost_dir> <out.tsv> [ゴースト名...]
  ゴースト名を省くと、辞書に「質問」を含むゴーストを全部見る。
出力: 1行1節の TSV（ゴースト名 / 辞書ファイル / 節の見出し / 選択肢を " / " で連結）
"""
import os, sys, re

root, out = sys.argv[1], sys.argv[2]
names = sys.argv[3:]

EXTS = ('.txt', '.dic', '.aym', '.kis')
SKIP = ('readme', 'descript', 'install', 'updates')
CHOICE = re.compile(r'\\_*q\[([^,\]]{1,60}),[^\]]*\]|^＿(.{1,60})$|\\_a\[[^\]]*\]([^\\]{1,40})\\_a', re.M)
SPLIT = re.compile(r'(?m)^(?=[＊＠]|\S+\s*\{\s*$|\S+\s*$\n\{)')
TOPIC = re.compile(r'質問|聞|訊|尋|Q&A|Ｑ')


def read(p):
  b = open(p, 'rb').read()
  for enc in ('utf-8', 'cp932'):
    try:
      return b.decode(enc)
    except UnicodeDecodeError:
      pass
  return b.decode('cp932', 'replace')


def dic_files(g):
  gm = os.path.join(root, g, 'ghost', 'master')
  for dp, _, fn in os.walk(gm):
    for f in fn:
      fl = f.lower()
      if fl.endswith(EXTS) and not fl.startswith(SKIP):
        yield gm, os.path.join(dp, f)


if not names:
  for g in sorted(os.listdir(root)):
    if any('質問' in read(p) for _, p in dic_files(g)):
      names.append(g)

rows = 0
with open(out, 'w', encoding='utf-8', newline='\n') as w:
  for g in names:
    for gm, p in dic_files(g):
      t = read(p)
      if '質問' not in t and '聞きたい' not in t and '訊' not in t:
        continue
      for b in SPLIT.split(t):
        head = b.split('\n', 1)[0].replace('\t', ' ').replace('\r', '')[:60]
        labs = [(m.group(1) or m.group(2) or m.group(3)).split('\t')[0].strip() for m in CHOICE.finditer(b)]
        labs = [l for l in labs if l]
        if len(labs) >= 4 and TOPIC.search(head + ''.join(labs[:3]) + b[:300]):
          w.write('\t'.join([g, os.path.relpath(p, gm), head, ' / '.join(labs)]) + '\n')
          rows += 1
print(f'{len(names)} ghosts, {rows} menus -> {out}')
