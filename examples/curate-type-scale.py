#!/usr/bin/env python3
"""One-time 0.10.1 migration from local markup sizes to named scale roles."""
from pathlib import Path
import re, json
root=Path(__file__).resolve().parents[1]
changes=[]
roles={'--mui-text-display-size':'display','--mui-text-h1-size':'h1','--mui-text-h2-size':'h2','--mui-text-h3-size':'h3','--mui-text-body-size':'body','--mui-text-small-size':'small','--mui-text-caption-size':'caption'}
for path in (root/'src').rglob('*.rs'):
    if path.name=='generated_props.rs':continue
    source=path.read_text()
    def replace(m):
        style=m[1]
        size=re.search(r'font-size\s*:\s*var\((--mui-[\w-]+)\)\s*;?',style)
        if not size or size[1] not in roles:return m[0]
        role=roles[size[1]]
        rest=style[:size.start()]+style[size.end():]
        changes.append({'file':str(path.relative_to(root)),'role':role})
        return f'data-mui-type="{role}"'+(f' style="{rest}"' if rest.strip() else '')
    updated=re.sub(r'style="([^"\n]*)"',replace,source)
    if updated!=source:path.write_text(updated)
if changes:
    (root/'docs/night-2-typography-audit.json').write_text(json.dumps({'inline_sizes_replaced':len(changes),'changes':changes},indent=2)+'\n')
print(f'Moved {len(changes)} inline font-size declarations to named scale roles.')
