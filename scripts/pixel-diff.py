#!/usr/bin/env python3
"""pixel-diff REF.png CANDIDATE.png [--out diff.png] [--tol 24]
Prints % of pixels within tolerance (per-channel max abs diff <= tol), mean abs diff,
and the 8x6 grid cells with the worst match so you know WHERE to look. Writes a heatmap."""
import sys, argparse
from PIL import Image, ImageChops
a=argparse.ArgumentParser(); a.add_argument('ref'); a.add_argument('cand'); a.add_argument('--out',default='diff.png'); a.add_argument('--tol',type=int,default=24)
o=a.parse_args()
R=Image.open(o.ref).convert('RGB'); C=Image.open(o.cand).convert('RGB')
if R.size!=C.size:
    if R.height != C.height:
        print(f"HEIGHT MISMATCH ref {R.height} cand {C.height}")
    print(f"SIZE MISMATCH ref {R.size} cand {C.size}; resizing candidate to ref for the score (fix the viewport instead)"); C=C.resize(R.size)
D=ImageChops.difference(R,C); w,h=R.size
r,g,b=D.split()
maximum=ImageChops.lighter(ImageChops.lighter(r,g),b)
hist=maximum.histogram()
tot=w*h; ok=sum(hist[:max(0,min(256,o.tol+1))]); s=sum(v*n for v,n in enumerate(hist))
off=maximum.point(lambda v: 255 if v>o.tol else 0)
gw,gh=8,6; cells=[[0]*gw for _ in range(gh)]
# Ceil boundaries reproduce the old x*gw//w and y*gh//h bins,
# including images whose dimensions are not divisible by the grid size.
for j in range(gh):
    for i in range(gw):
        box=((i*w+gw-1)//gw, (j*h+gh-1)//gh,
             ((i+1)*w+gw-1)//gw, ((j+1)*h+gh-1)//gh)
        cells[j][i]=off.crop(box).histogram()[255]
print(f"match {ok/tot*100:.2f}% of pixels within tol {o.tol}; mean abs diff {s/tot:.2f}/255")
# The absolute count is what visual-check gates on: captures are deterministic (80 of 84
# pairs were pixel-identical on 2026-09-23), and a lost "/" separator is ~300 pixels of a
# 2-megapixel page, which any percentage threshold rounds away.
print(f"differ {tot-ok} pixels")
worst=sorted(((cells[j][i],i,j) for j in range(gh) for i in range(gw)),reverse=True)[:6]
cw,ch=w//gw,h//gh
for n,i,j in worst:
    if n: print(f"  worst cell col{i} row{j} (x {i*cw}-{(i+1)*cw}, y {j*ch}-{(j+1)*ch}): {n/max(1,cw*ch)*100:.1f}% off")
heat=D.point(lambda v: min(255,v*4)); heat.save(o.out); print(f"wrote {o.out}")
