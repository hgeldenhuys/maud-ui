"""Browser-free source sweep against the committed 0.9.0 baseline; no git writes."""
from collections import Counter
from pathlib import Path
import json
import re
import subprocess


def before(path):
    return subprocess.check_output(['git', 'show', f'HEAD:{path}'], text=True)


def css_in(path, source):
    if path.suffix == '.css':
        return re.sub(r'/\*[\s\S]*?\*/', '', source)
    inline = [m[1] for m in re.finditer(r'style="([^"\n]*)"', source)]
    inline += [m[1] for m in re.finditer(r'style=\(format!\("([^"\n]*)"', source)]
    # Only CSS functions; Rust examples and JavaScript data are separate concerns.
    inline += [m[1] for m in re.finditer(r'fn \w*css\(\)[^{]*\{\s*r#+"([\s\S]*?)"#+', source)]
    return re.sub(r'/\*[\s\S]*?\*/', '', '\n'.join(inline))


def local_declarations(css):
    result = Counter()
    for match in re.finditer(r'([\w-]+)\s*:\s*([^;{}]+)(?:;|$)', css):
        prop, value = match[1], match[2].strip()
        rhythm = re.fullmatch(r'(margin|padding)(-[\w-]+)?|((row|column)-)?gap|font-size|border-radius|box-shadow', prop)
        alias = re.search(r'var\(--mui-(primary|muted-fg|fg-muted|muted-foreground|destructive|card-bg|radius)[,)]', value)
        paint = re.search(r'#[0-9a-fA-F]{3,8}\b|linear-gradient\(|drop-shadow\(', value)
        if alias or paint or (rhythm and re.search(r'\d+(\.\d+)?(?:rem|px)\b', value)):
            result[f'{prop}: {value}'] += 1
    return result


files = sorted([*Path('static/styles').rglob('*.css'), *Path('src/primitives').glob('*.rs'), *Path('src/blocks').rglob('*.rs'), Path('src/showcase/mod.rs')])
changes = []
for path in files:
    if path.name in ('tokens.css', 'swatch.rs'):
        continue  # Named palette definitions and deliberate raw-color specimens.
    old = local_declarations(css_in(path, before(path)))
    new = local_declarations(css_in(path, path.read_text()))
    removed = old - new
    if removed:
        changes.append({'file': str(path), 'removed_local_declarations': sum(removed.values()), 'declarations': dict(sorted(removed.items()))})

removed_rules = [
    {'file': 'static/styles/defaults.css', 'count': 5, 'reason': 'Competing chip and badge defaults now live in their component styles'},
    {'file': 'static/styles/base.css', 'count': 1, 'reason': 'Obsolete showcase header h1 reset'},
    {'file': 'src/showcase/mod.rs', 'count': 1, 'reason': 'Second obsolete showcase header h1 reset'},
    {'file': 'static/styles/blocks/page-header.css', 'count': 2, 'reason': 'Cramped phone search-trigger padding and five-rem switcher cap'},
    {'file': 'static/styles/components/code_block.css', 'count': 1, 'reason': 'Two independent palettes consolidated into one shared role mapping'},
]
report = {
    'baseline': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
    'scope': 'Every primitive/block and canonical stylesheet, plus gallery CSS; exported assets excluded from counts',
    'removed_rule_blocks': sum(row['count'] for row in removed_rules),
    'removed_rules': removed_rules,
    'additional_consolidation': 'Six badge selectors removed from shared badge/alert overrides; alerts retain those rules',
    'removed_or_replaced_local_declarations': sum(row['removed_local_declarations'] for row in changes),
    'changed_style_files': len(changes),
    'changes': changes,
    'retained': ['Raw swatch palettes and checkerboard transparency specimen', 'Third-party data-series/drawing colors and terminal ANSI palette', 'Layout geometry, accessible hidden-field offsets, ratios, and 16px phone text inputs', 'Hover-card preview clearance of 6rem'],
}
Path('docs/design-r2-style-audit.json').write_text(json.dumps(report, indent=2) + '\n')
print(f"{report['removed_rule_blocks']} obsolete rule blocks removed; {report['removed_or_replaced_local_declarations']} local declarations replaced/removed in {report['changed_style_files']} files.")
