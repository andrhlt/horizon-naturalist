"""Capture Naturalist 2.0.7 Dragonfly flight and Firefly glow observations on an isolated Java server."""
import argparse
import json
from pathlib import Path
import time

import deer_behavior
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    time.sleep(1.0)
    command("time set midnight")
    command("tick freeze")
    command("fill 0 92 0 15 92 15 minecraft:stone")
    command("fill 0 93 0 15 98 15 minecraft:air")
    prefix = "insect_fixture_wings "
    result = next(json.loads(row.split(prefix, 1)[1])
                  for row in command("insect_fixture_wings") if prefix in row)
    command("summon naturalist:dragonfly 8.5 98 8.5 {Tags:[summon_wing]}")
    rows = command("data get entity @e[tag=summon_wing,limit=1] NoGravity")
    result["summon_dragonfly_no_gravity"] = any("following entity data: 1b" in row for row in rows)
    result["summon_console"] = rows
    result["summon_motion_path"] = command("data get entity @e[tag=summon_wing,limit=1] Motion")
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    with deer_behavior.reference_probe(args.server, args.java_home, "WingProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "insect-wings-fixture"])
