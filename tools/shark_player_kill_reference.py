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
    command("fill 3 92 7 16 96 7 minecraft:glass")
    command("fill 5 92 8 14 95 15 minecraft:water")
    command("tick step 40")
    time.sleep(3.0)
    marker = "shark_kill_player "
    for row in command("shark_kill_player"):
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
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "SharkKillPlayerProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "shark-overworld-player-kill"])
