"""Favorite のゴーストを一覧し、\1 側にキャラの絵があるかを判定する材料を集める。
usage: PYTHONIOENCODING=utf-8 python survey_ghosts.py <favorite_dir>
出力: ゴースト名 / SHIORI / sakura名 / kero名 / kero既定サーフェス / シェルに surface10 系の画像があるか
"""
import os, sys, glob, re

root = sys.argv[1]

def read(p):
    b = open(p, 'rb').read()
    for enc in ('utf-8-sig', 'cp932'):
        try:
            return b.decode(enc)
        except UnicodeDecodeError:
            pass
    return b.decode('cp932', errors='replace')

def kv(text):
    d = {}
    for line in text.splitlines():
        if ',' in line and not line.startswith('//'):
            k, v = line.split(',', 1)
            d.setdefault(k.strip().lower(), v.strip())
    return d

for g in sorted(os.listdir(root)):
    gm = os.path.join(root, g, 'ghost', 'master', 'descript.txt')
    if not os.path.isfile(gm):
        continue
    d = kv(read(gm))
    shiori = d.get('shiori', '?')
    sname, kname = d.get('sakura.name', ''), d.get('kero.name', '')
    shells = glob.glob(os.path.join(root, g, 'shell', '*', 'descript.txt'))
    sh = os.path.dirname(shells[0]) if shells else None
    kdef, s10 = '', ''
    if sh:
        sd = kv(read(os.path.join(sh, 'descript.txt')))
        kdef = sd.get('kero.seriko.defaultsurface', '')
        pngs = [os.path.basename(p).lower() for p in glob.glob(os.path.join(sh, '*.png'))]
        s10 = 'png' if 'surface10.png' in pngs or 'surface0010.png' in pngs else ''
        surf = ''
        for p in glob.glob(os.path.join(sh, 'surface*.txt')):
            surf += read(p)
        if re.search(r'^\s*surface[^\n{]*\b10\b', surf, re.M):
            s10 += '+def'
    print('\t'.join([g, shiori, sname, kname, kdef, s10 or '-']))
