r"""extract_satori.py の出力から、ゴーストごとにトークを無作為に抜いて読みやすく並べる。
ウェイト（ｗ９ \w9 \_w[900]）は消す。行頭の「：」は話者交代なので「｜」に置き換えて見せる。

usage: PYTHONIOENCODING=utf-8 python sample_talks.py <talks_dir> <n> <seed> <ghost> [<ghost> ...]
"""
import os, sys, re, random

d, n, seed = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
WAIT = re.compile(r'ｗ[０-９0-9]|ｗ|\\w\d|\\_w\[\d+\]|φ')
for g in sys.argv[4:]:
    p = os.path.join(d, g + '.txt')
    if not os.path.exists(p):
        print('#####', g, '(なし)')
        continue
    talks = open(p, encoding='utf-8').read().split('=== ')[1:]
    random.seed(seed)
    print('#####', g, len(talks))
    for t in random.sample(talks, min(n, len(talks))):
        head, _, body = t.partition('\n')
        lines = [WAIT.sub('', l).replace('：', '｜', 1) if l.startswith('：') else WAIT.sub('', l) for l in body.strip().split('\n')]
        print('--', head.split('[')[-1].rstrip(']'))
        print('\n'.join(lines)[:600])
