r"""抜き出したランダムトーク（extract_satori.py の出力）で、\1 側の使われ方を数える。

里々のトークは \1 が喋る状態で始まり、行頭の「：」で \0 と \1 を行き来する
（里々 docs「スコープとサーフェス」）。\0 \1 \p[n] でも切り替わる。
\1 の番で表情（（数字）や \s[数字]）を変えていれば、\1 に絵がある見込みが高い。

usage: PYTHONIOENCODING=utf-8 python kero_usage.py <talks_dir>
出力: ゴースト / トーク数 / \1 が喋るトークの割合 / \1 の番で表情を変える回数と番号 / \1 の台詞の例
"""
import os, sys, re, collections

d = sys.argv[1]
SURF = re.compile(r'（([0-9０-９]+)）|\\s\[(-?\d+)\]')
SCOPE = re.compile(r'(\\[01]|\\[hu]|\\p\[\d+\])')
Z2H = str.maketrans('０１２３４５６７８９', '0123456789')

for f in sorted(os.listdir(d)):
    talks = open(os.path.join(d, f), encoding='utf-8').read().split('=== ')[1:]
    n = len(talks)
    with1, kero_surf, samples = 0, collections.Counter(), []
    for t in talks:
        body = t.split('\n', 1)[1] if '\n' in t else ''
        speaker, used = 1, False
        for line in body.split('\n'):
            if line.startswith('＄'):
                continue
            if line.startswith('：'):
                speaker = 1 - speaker if speaker in (0, 1) else 0
                line = line[1:]
            for part in SCOPE.split(line):
                if part in ('\\0', '\\h', '\\p[0]'):
                    speaker = 0
                    continue
                if part in ('\\1', '\\u', '\\p[1]'):
                    speaker = 1
                    continue
                if part.startswith('\\p['):
                    speaker = 2
                    continue
                if speaker == 1 and part.strip():
                    used = True
                    if len(samples) < 4 and len(part.strip()) > 6:
                        samples.append(part.strip()[:50])
                    for m in SURF.finditer(part):
                        kero_surf[(m.group(1) or m.group(2)).translate(Z2H)] += 1
        with1 += used
    top = ','.join(k for k, _ in kero_surf.most_common(5))
    print('%-28s talks=%4d with1=%3d%% kero_surf=%4d [%s]' % (f[:-4], n, 100 * with1 // max(n, 1), sum(kero_surf.values()), top))
    for x in samples:
        print('      \\1| ' + x)
