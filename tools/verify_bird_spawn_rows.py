"""Check Naturalist Bird rows in the biome bundles built by a Horizon launch."""
import argparse
import json
from pathlib import Path


EXPECTED = {
    "dappled_forest": 0,
    "forest": 30,
    "plains": 20,
    "savanna": 20,
    "old_growth_pine_taiga": 10,
    "desert": 10,
}


def bird_weights(path):
    source = json.loads(path.read_text())
    result = {}
    for biome in EXPECTED:
        record = json.loads(source[f"minecraft:{biome}"])
        values = record.get("attributes", {}).get("minecraft:gameplay/natural_mob_spawns", {})
        values = values.get("argument", values)
        creatures = values.get("spawns_by_category", {}).get("creature", [])
        weights = [row["weight"] for row in creatures if row["type"] == "naturalist:bird"]
        result[biome] = sum(weights)
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("patched", type=Path)
    parser.add_argument("--baseline", type=Path)
    parser.add_argument("--sparrow-blacklisted-in-plains", action="store_true")
    args = parser.parse_args()
    expected = EXPECTED.copy()
    if args.sparrow_blacklisted_in_plains:
        expected["plains"] = 10
    actual = bird_weights(args.patched)
    result = {"patched": actual}
    if args.baseline:
        result["baseline"] = bird_weights(args.baseline)
        assert result["baseline"]["dappled_forest"] == 30, result
    print(json.dumps(result, indent=2))
    assert actual == expected, result
