"""Compare loaded variant order and every biome's weighted pool to Java."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def compare(version, java):
    entity_types = version / "native-content/naturalist/data/naturalist/horizon/entity_type"
    available = {f"naturalist:{file.stem}" for file in entity_types.glob("*.json")}
    variants = version / "native-content/naturalist/data/naturalist/naturalist"
    species = sorted(f"naturalist:{entry.name.removesuffix('_variant')}"
                     for entry in variants.glob("*_variant")
                     if f"naturalist:{entry.name.removesuffix('_variant')}" in available)
    biomes = sorted({biome for row in java.values() for biome in row["pools"]})
    assert species and biomes
    with tempfile.TemporaryDirectory(prefix="nat-variant-pools-", ignore_cleanup_errors=True) as work:
        home = Path(work)
        socket = home / "variant-pools.sock"

        def call(*args):
            proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                  capture_output=True, text=True, check=True, timeout=85)
            return json.loads(proc.stdout.splitlines()[-1])

        try:
            call("launch", "--headless", "--game", version / "horizon-client",
                 "--resources", version / ".local/horizon-client/26.3", "--home", home,
                 "--game-dir", home, "--world", "Naturalist Variant Tables",
                 "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            differences = []
            compare_count = 0
            for kind in species:
                expected = java[kind]
                for i in range(0, len(biomes), 12):
                    keys = biomes[i:i + 12]
                    marker = "Naturalist variant pools "
                    result = call("command", f"naturalist_fixture_variant_pool {kind} {' '.join(keys)}")
                    messages = [msg for msg in result["chat"] if marker in msg]
                    assert len(messages) == 1, (kind, result)
                    actual = json.loads(messages[0].split(marker, 1)[1])
                    if i == 0 and actual["order"] != expected["order"]:
                        differences.append((kind, "registry order", expected["order"], actual["order"]))
                    for biome in keys:
                        want = expected["pools"].get(biome, [])
                        got = actual["pools"].get(biome, [])
                        if want != got:
                            differences.append((kind, biome, want, got))
                        compare_count += 1
            return {"species": len(species), "biomes": len(biomes), "weighted_pools": compare_count,
                    "differences": differences}
        finally:
            if socket.exists():
                call("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    result = compare(args.version, json.loads(args.reference.read_text()))
    print(json.dumps(result, indent=2))
    assert not result["differences"]
