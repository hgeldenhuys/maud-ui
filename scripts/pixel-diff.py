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
    print(f"SIZE MISMATCH ref {R.size} cand {C.size}; resizing candidate to ref for the score (fix the viewport instead)"); C=C.resize(R.size)
D=ImageChops.difference(R,C); px=D.load(); w,h=R.size
tot=w*h; ok=0; s=0
gw,gh=8,6; cells=[[0]*gw for _ in range(gh)]
for y in range(h):
    for x in range(w):
        r,g,b=px[x,y]; m=max(r,g,b); s+=m
        if m<=o.tol: ok+=1
        else: cells[y*gh//h][x*gw//w]+=1
print(f"match {ok/tot*100:.2f}% of pixels within tol {o.tol}; mean abs diff {s/tot:.2f}/255")
worst=sorted(((cells[j][i],i,j) for j in range(gh) for i in range(gw)),reverse=True)[:6]
cw,ch=w//gw,h//gh
for n,i,j in worst:
    if n: print(f"  worst cell col{i} row{j} (x {i*cw}-{(i+1)*cw}, y {j*ch}-{(j+1)*ch}): {n/(cw*ch)*100:.1f}% off")
heat=D.point(lambda v: min(255,v*4)); heat.save(o.out); print(f"wrote {o.out}")
