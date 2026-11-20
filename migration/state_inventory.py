"""Inventory legacy injected state accessors and their direct Rust callers."""
from collections import Counter
from pathlib import Path
import re
import tomllib

ROOT = Path(__file__).resolve().parents[1]
manifest = tomllib.loads((ROOT / 'launcher/horizon-mixin.toml').read_text())
sources = sorted((*ROOT.glob('launcher/**/*.rs'), *ROOT.glob('src/**/*.rs')))
lines = [(p.relative_to(ROOT), p.read_text().splitlines()) for p in sources]
states = manifest['state']
print(f"{len(states)} state declarations: " + ', '.join(
    f'{count} {target}' for target, count in Counter(s['target'] for s in states).most_common()))
for state in states:
    method = '__horizon_mixins_naturalist_' + state['id'].replace('-', '_')
    uses = [(p, i) for p, text in lines for i, line in enumerate(text, 1)
            if re.search(r'\b' + re.escape(method) + r'(?:_mut)?\s*\(', line)]
    print(f"\n{state['id']} [{len(uses)} direct accesses]")
    for path, number in uses:
        print(f'  {path}:{number}')
