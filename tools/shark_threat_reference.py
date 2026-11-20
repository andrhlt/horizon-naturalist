"""Read pinned Java Great White Shark threat choices at daylight and midnight."""
import argparse
import json
import time
from pathlib import Path
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 16 0")
    command("difficulty normal")
    command("tick freeze")
    command("fill 4 91 3 14 91 15 minecraft:stone")
    command("fill 4 92 3 14 98 15 minecraft:air")
    command("weather clear")
    out = {}
    for label, tick in (("day", 6000), ("night", 18000)):
        command(f"time set {tick}")
        command("tick step 24")
        time.sleep(2.0)
        marker = "shark_threat_fixture "
        rows = command(f"shark_threat_fixture {label}")
        out[label] = next(json.loads(row.split(marker,1)[1]) for row in rows if marker + "{" in row)
    return out


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "SharkThreatProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "shark-overworld-threat"])
