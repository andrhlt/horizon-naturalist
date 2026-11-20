"""Capture Naturalist 2.0.7 insect substrate and light spawn rules on an isolated Java server."""
import argparse
import json
from pathlib import Path
import time

import deer_behavior
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add -32 -16 47 63")
    time.sleep(1.0)
    command("time set noon")
    command("fill 0 92 0 15 96 15 minecraft:air")
    command("fill 0 92 0 15 92 15 minecraft:stone")
    command("setblock 2 92 2 minecraft:mud")
    command("fill -16 93 18 32 96 51 minecraft:air")
    command("fill -16 95 18 32 95 51 minecraft:stone")
    command("setblock 2 92 34 minecraft:grass_block")
    command("setblock 11 92 34 minecraft:stone")
    time.sleep(1.0)
    command("tick freeze")
    prefix = "insect_spawn_fixture "
    return next(json.loads(row.split(prefix, 1)[1])
                for row in command("insect_spawn_fixture") if prefix in row)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    with deer_behavior.reference_probe(args.server, args.java_home, "InsectSpawnProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "insect-spawn-light-fixture"])
