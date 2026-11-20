"""Compare actual loaded Horizon biome spawns to the pinned Java registry."""
import argparse
import json
from collections import Counter
from pathlib import Path
import subprocess
import tempfile


def load_table(bundle, reference, types=None):
    raw = json.loads(bundle.read_text())
    if types is None:
        types = {row[1] for rows in reference.values() for row in rows} | {"naturalist:caterpillar"}
    output = {}
    for biome in reference:
        record = json.loads(raw[biome])
        spawns = record.get("attributes", {}).get("minecraft:gameplay/natural_mob_spawns", {})
        spawns = spawns.get("argument", spawns)
        rows = []
        for category, entries in spawns.get("spawns_by_category", {}).items():
            for entry in entries:
                if entry["type"] not in types:
                    continue
                count = entry["count"]
                low, high = ((count, count) if isinstance(count, int)
                             else (count["min_inclusive"], count["max_inclusive"]))
                rows.append([category, entry["type"], entry["weight"], low, high])
        output[biome] = sorted(rows)
    return output


def capture(version, reference, types=None):
    with tempfile.TemporaryDirectory(prefix="nat-land-tables-", ignore_cleanup_errors=True) as work:
        home = Path(work)
        socket = home / "ctl.sock"

        def control(*args):
            process = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                     check=True, capture_output=True, text=True, timeout=65)
            return json.loads(process.stdout.splitlines()[-1])

        try:
            control("launch", "--headless", "--game", version / "horizon-client",
                    "--resources", version / ".local/horizon-client/26.3", "--home", home,
                    "--game-dir", home, "--world", "Naturalist Land Spawn Tables",
                    "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            bundle, = (home / ".horizon-cache/mod-content").glob("*/worldgen/biomes.json")
            return load_table(bundle, reference, types)
        finally:
            if socket.exists():
                control("quit")


def compare(actual, reference):
    differences = {}
    for name, rows in reference.items():
        expected_counts = Counter(map(tuple, rows))
        actual_counts = Counter(map(tuple, actual[name]))
        if actual_counts != expected_counts:
            differences[name] = {"missing": list((expected_counts - actual_counts).elements()),
                                 "additional": list((actual_counts - expected_counts).elements())}
    return differences


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path, help="Horizon version directory or compiled biome bundle")
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    reference = json.loads(args.reference.read_text())
    if args.source.is_file():
        actual = load_table(args.source, reference)
    else:
        kinds = args.source / "native-content/naturalist/data/naturalist/horizon/entity_type"
        installed = {f"naturalist:{file.stem}" for file in kinds.glob("*.json")}
        assert installed, kinds
        vanilla = {row[1] for rows in reference.values() for row in rows if row[1].startswith("minecraft:")}
        types = installed | vanilla
        reference = {biome: [row for row in rows if row[1] in types] for biome, rows in reference.items()}
        actual = capture(args.source, reference, types)
    differences = compare(actual, reference)
    print(json.dumps({"biomes": len(actual), "differences": differences}, indent=2), flush=True)
    assert not differences, f"spawn rows differ in {len(differences)} biomes"
