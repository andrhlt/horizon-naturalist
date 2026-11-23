"""Compare ordered Java biome spawn rows with an exported Horizon bundle.

Usage: python3 tools/compare_spawn_bundles.py fixtures/bird-spawn-tables-java.json HORIZON_BIOMES [--candidate ID]

JAVA_ROWS is the captured 67-biome Java ``bird-spawn-tables-java.json`` shape. HORIZON_BIOMES
is the ``worldgen/biomes.json`` export from the exact candidate. The comparison
keeps row order because weighted selection consumes random draws in that order.
"""
import argparse
import json
from collections import Counter
from pathlib import Path


CATEGORY_ROUTES = {
    "naturalist:anglerfish": "water_ambient",
    "naturalist:bass": "water_ambient",
    "naturalist:blobfish": "water_ambient",
    "naturalist:butterfly": "ambient",
    "naturalist:catfish": "water_ambient",
    "naturalist:clam": "water_ambient",
    "naturalist:dragonfly": "ambient",
    "naturalist:firefly": "ambient",
    "naturalist:giant_isopod": "water_ambient",
    "naturalist:great_white_shark": "water_creature",
    "naturalist:jellyfish": "water_ambient",
    "naturalist:piranha": "water_ambient",
    "naturalist:ray": "water_ambient",
    "naturalist:starfish": "water_ambient",
    "naturalist:whale": "water_creature",
}
def counts(count):
    if isinstance(count, int):
        return count, count
    return (
        count.get("min_inclusive", count.get("value", 1)),
        count.get("max_inclusive", count.get("value", 1)),
    )


def horizon_rows(document):
    if isinstance(document, str):
        document = json.loads(document)
    attributes = document.get("attributes", {}).get(
        "minecraft:gameplay/natural_mob_spawns", {}
    )
    attributes = attributes.get("argument", attributes)
    categories = attributes.get("spawns_by_category", {})
    rows = {}
    moved = []
    for category, entries in categories.items():
        for entry in entries:
            species = entry["type"]
            destination = CATEGORY_ROUTES.get(species, category)
            row = [destination, species, entry["weight"], *counts(entry["count"])]
            if destination != category:
                moved.append((destination, row))
            else:
                rows.setdefault(destination, []).append(row)
    for category, row in moved:
        rows.setdefault(category, []).append(row)
    return rows


def compare(java, horizon):
    mismatches = {}
    for biome, expected_rows in java.items():
        actual_by_category = horizon_rows(horizon[biome])
        expected_types = {row[1] for row in expected_rows}
        categories = {row[0] for row in expected_rows}
        for category in categories:
            expected = [row for row in expected_rows if row[0] == category]
            actual = [
                row
                for row in actual_by_category.get(category, [])
                if row[1] in expected_types
            ]
            if actual != expected:
                mismatches.setdefault(biome, {})[category] = {
                    "same_multiset": Counter(map(tuple, actual))
                    == Counter(map(tuple, expected)),
                    "java": expected,
                    "horizon": actual,
                }
    return mismatches


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("java_rows", type=Path)
    parser.add_argument("horizon_biomes", type=Path)
    parser.add_argument("--candidate", default="unspecified")
    args = parser.parse_args()
    java = json.loads(args.java_rows.read_text())
    horizon = json.loads(args.horizon_biomes.read_text())
    missing_biomes = sorted(java.keys() - horizon.keys())
    if missing_biomes:
        parser.error(f"Horizon bundle is missing Java biomes: {missing_biomes}")
    mismatches = compare(java, horizon)
    print(
        json.dumps(
            {
                "candidate": args.candidate,
                "biomes_compared": len(java),
                "java_rows": sum(map(len, java.values())),
                "mismatched_biomes": len(mismatches),
                "mismatched_categories": sum(map(len, mismatches.values())),
                "mismatches": mismatches,
            },
            indent=2,
        )
    )
    return int(bool(mismatches))


if __name__ == "__main__":
    raise SystemExit(main())
