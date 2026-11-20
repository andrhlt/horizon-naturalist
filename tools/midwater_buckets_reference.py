"""Record the pinned MidwaterBuckets's contact damage, swim pulse and bucket color."""
import argparse
import json
from pathlib import Path
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 16 0")
    command("difficulty normal")
    command("tick freeze")
    command("fill 4 91 3 14 91 15 minecraft:stone")
    command("fill 4 92 3 14 98 15 minecraft:air")
    command("fill 5 92 8 14 95 15 minecraft:water")
    marker = "midwater_buckets_fixture "
    for row in command("midwater_buckets_fixture"):
        if marker + "{" in row:
            return json.loads(row.split(marker, 1)[1])
    raise RuntimeError("MidwaterBuckets fixture emitted no result")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "MidwaterBucketsProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "midwater_buckets-overworld-fixture"])
