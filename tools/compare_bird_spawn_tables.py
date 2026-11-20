import json
import sys
from pathlib import Path
from collections import Counter
java=json.loads(Path(sys.argv[1]).read_text())
horizon=json.loads(Path(sys.argv[2]).read_text())
known={'minecraft:cow','minecraft:pig','minecraft:sheep','minecraft:chicken','minecraft:fox','minecraft:rabbit'}
assert java.keys() <= horizon.keys(), sorted(java.keys() - horizon.keys())
fail={}
for biome,jrows in java.items():
    doc=json.loads(horizon[biome]); attr=doc.get('attributes',{}).get('minecraft:gameplay/natural_mob_spawns',{}); data=attr.get('argument',attr); rows=data.get('spawns_by_category',{}).get('creature',[])
    actual=[]
    for r in rows:
        kind=r['type']
        if not (kind in known or kind.startswith('naturalist:')): continue
        count=r['count']; lo,hi=(count,count) if isinstance(count,int) else (count.get('min_inclusive',count.get('value',1)),count.get('max_inclusive',count.get('value',1)))
        actual.append(['creature',kind,r['weight'],lo,hi])
    expected=[r for r in jrows if r[0]=='creature']
    if actual!=expected:
        fail[biome]={'same_multiset':Counter(map(tuple,actual))==Counter(map(tuple,expected)),'java':expected,'horizon':actual}
print(json.dumps({'biomes_compared':len(java),'mismatch_count':len(fail),'mismatches':fail},indent=2))

if fail:
    raise SystemExit(1)
