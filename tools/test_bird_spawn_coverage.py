"""Check that Horizon Bird biome tags reproduce Java Bird row multiplicity."""
import json
from pathlib import Path


ROOT = Path(__file__).parents[1]


def main() -> None:
    java = json.loads((ROOT / "fixtures/bird-spawn-tables-java.json").read_text())
    expected = {
        biome: sum(row[1] == "naturalist:bird" for row in rows)
        for biome, rows in java.items()
    }
    observed = {biome: 0 for biome in expected}
    for ordinal in range(1, 7):
        tag = json.loads(
            (ROOT / f"data/naturalist/tags/worldgen/biome/bird_spawn_{ordinal}.json").read_text()
        )
        for biome in tag["values"]:
            observed[biome] += 1
    assert observed == expected, {b: (expected[b], observed[b]) for b in expected if expected[b] != observed[b]}

    pools = json.loads((ROOT / "fixtures/variant-order-java.json").read_text())["naturalist:bird"]["pools"]
    # The variant fixture includes dappled_forest as a pool probe even though
    # the pinned Java spawn table has no Bird row there.
    assert {b for b, rows in pools.items() if rows} >= {b for b, count in expected.items() if count}
    print(f"Bird biome coverage: {len(expected)} biomes, {sum(expected.values())} Java rows")


if __name__ == "__main__":
    main()
