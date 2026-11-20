"""Record pinned Java Hippo traits, pet interactions, water breeding and boats."""
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
    command("tick step 40")
    time.sleep(3.0)
    marker = "hippo_threat_fixture "
    for row in command("hippo_threat_fixture"):
        if marker + "{" in row:
            return json.loads(row.split(marker, 1)[1])
    raise RuntimeError("Hippo fixture emitted no result")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "HippoThreatProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "hippo-overworld-threat"])
