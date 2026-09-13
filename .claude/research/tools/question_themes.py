r"""extract_questions.py の出力から、主題ごとに「その問いを持つゴーストの数」を数える。

見出しか辞書のファイル名に「質問」「聞く」「尋ね」「QA」などがある節だけを質問メニューとみなし、そのうち
設定・アンケート・占いなどの節と、一覧として読めないゴーストは除く。主題の判定は語の一致だけなので、目安として使う。

usage: PYTHONIOENCODING=utf-8 python question_themes.py <questions.tsv> [最小問数=6]
"""
import sys, re, collections

src = sys.argv[1]
min_q = int(sys.argv[2]) if len(sys.argv) > 2 else 6

# 見出しがこれに当たる節を質問メニューとみなす。ただし NOT_MENU に当たるものは除く
MENU = re.compile(r'質問|聞く|聞きたい|尋ね|ＱＡ|QandA|QA|Question|question|aboutU')
NOT_MENU = re.compile(r'動物占い|receive|Enquete|Kaiwa')
# 選択肢の文言を拾えない、または質問一覧ではないもの（2026-10-07 の調査で確かめた）
NOT_LIST = {'Lost_You_Somewhere', 'thorn-lander', 'URE_2', 'railway', 'emily4', 'probatio_diabolica', 'nlogger_sfm',
            'heath_and_olive', 'of_kouko', '#scavenger', 'E_DaR', 'kyoko', 'Menagerie', '6_v', 'rereregion', 'musuwaka'}
THEMES = {
  '性・下着': '下着|パンツ|スリーサイズ|バスト|体位|エッチ|生理|フェチ|おっぱい|勝負下着|発情',
  '年齢': '年齢|何歳|年は|いくつ？|大まかな年齢',
  '誕生日': '誕生日',
  '身長体重': '身長|体重',
  '食べ物': '食べ物|好物|好き嫌い|お酒|カクテル',
  '恋愛': '恋|好みのタイプ|好みの異性|結婚|彼氏|好きな人',
  '家族': '家族|両親|父|母|兄弟|きょうだい',
  '5つの質問': '恋人と親友|一番嬉しかった|好きな何か',
  'ユーザ評': '（ユーザ名）|（一人称）|自分のことをどう|ユーザとの関係|私のこと',
  '疑問でない': 'したい$|して$|しよう|ね$|だよ$|いいな$|ごめん|させて|て。$',
}

qs = collections.defaultdict(set)
for line in open(src, encoding='utf-8', newline='').read().split('\n'):
  if not line:
    continue
  g, dic, head, labs = line.split('\t', 3)
  head = dic + " " + head  # 辞書のファイル名（blood_04-1QA.txt など）も手がかりにする
  if not MENU.search(head) or NOT_MENU.search(head) or g in NOT_LIST:
    continue
  for l in labs.split(' / '):
    l = l.replace('φ', '').lstrip('○◯●□◆・「').rstrip('」').strip()
    if l:
      qs[g].add(l)

ghosts = sorted((g for g in qs if len(qs[g]) >= min_q), key=lambda g: -len(qs[g]))
print(f'{len(ghosts)} ghosts（選択肢 {min_q} 以上。戻る・閉じる等も含む数）')
print(' '.join(f'{g}:{len(qs[g])}' for g in ghosts))
for name, pat in THEMES.items():
  hit = [g for g in ghosts if any(re.search(pat, l) for l in qs[g])]
  print(f'{name}\t{len(hit)}\t{" ".join(hit)}')
