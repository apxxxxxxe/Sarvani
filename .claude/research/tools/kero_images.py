"""各ゴーストの master シェルで、\1 側の番号帯（surface10〜99）の画像に絵が入っているかを測る。
ImageMagick の trim 範囲が 0x0 なら全面同色（透明）とみなす。
usage: PYTHONIOENCODING=utf-8 python kero_images.py <favorite_dir>
出力: ゴースト / 10〜99帯の画像数 / 絵の入った枚数 / いちばん大きい絵の寸法
"""
import os, sys, glob, re, subprocess

root = sys.argv[1]
for g in sorted(os.listdir(root)):
    sh = os.path.join(root, g, 'shell', 'master')
    if not os.path.isdir(sh):
        continue
    files = []
    for p in glob.glob(os.path.join(sh, '*.png')):
        m = re.fullmatch(r'surface0*(\d+)\.png', os.path.basename(p), re.I)
        if m and 10 <= int(m.group(1)) <= 99:
            files.append(p)
    filled, best = 0, (0, '')
    for p in files:
        out = subprocess.run(['magick', p, '-format', '%@', 'info:'], capture_output=True, text=True).stdout
        m = re.match(r'(\d+)x(\d+)', out)
        if m:
            w, h = int(m.group(1)), int(m.group(2))
            if w * h > 400:
                filled += 1
                if w * h > best[0]:
                    best = (w * h, '%dx%d %s' % (w, h, os.path.basename(p)))
    print('%s\t%d\t%d\t%s' % (g, len(files), filled, best[1]))
