"""Capture loaded Java biome spawn rows for ported land wildlife and vanilla livestock."""
import argparse
import json
from pathlib import Path
import java_reference


def capture(command, _):
    marker = "land_spawn_table_fixture "
    rows = command("land_spawn_table_fixture")
    result = next(json.loads(row.split(marker, 1)[1])
                  for row in rows if marker + "{" in row)
    assert len(result) >= 60 and result["minecraft:forest"], result.keys()
    return {biome: sorted(entries) for biome, entries in result.items()}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "LandSpawnTableProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "naturalist-land-spawn-tables"])
