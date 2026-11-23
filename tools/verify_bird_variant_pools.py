#!/usr/bin/env python3
"""Compare the compiled Bird pool declaration with the pinned Java fixture."""

import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "src/current/shared_spawn.rs"
FIXTURE = ROOT / "fixtures/variant-order-java.json"
ROW = re.compile(r'^\s*\("([^"]+)", &\[(.*)\]\),$')
PAIR = re.compile(r'\("([^"]+)",\s*(\d+)\)')


def source_pools():
    pools = {}
    in_table = False
    for line in SOURCE.read_text().splitlines():
        if line.startswith("const BIRD_POOLS"):
            in_table = True
            continue
        if in_table and line == "];":
            break
        if not in_table:
            continue
        match = ROW.match(line)
        if match:
            pools[match.group(1)] = [(name, int(weight)) for name, weight in PAIR.findall(match.group(2))]
    return pools


def main():
    java = json.loads(FIXTURE.read_text())["naturalist:bird"]["pools"]
    expected = {biome: [(name, weight) for name, weight in rows] for biome, rows in java.items()}
    actual = source_pools()
    errors = []
    if set(actual) != set(expected):
        errors.append(f"biome coverage differs: source={len(actual)} Java={len(expected)}")
    for biome in sorted(set(actual) | set(expected)):
        if actual.get(biome) != expected.get(biome):
            errors.append(f"{biome}: source={actual.get(biome)!r} Java={expected.get(biome)!r}")
    result = {"java_biomes": len(expected), "source_biomes": len(actual), "errors": errors}
    print(json.dumps(result, indent=2, sort_keys=True))
    raise SystemExit(bool(errors))


if __name__ == "__main__":
    main()
