"""Measure original Shark and a positive-control Cow in matching flowing water."""
import argparse
import re
import time
from pathlib import Path
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 31 15")
    command("tick freeze")
    command("fill 2 91 1 31 91 16 minecraft:stone")
    command("fill 2 92 1 31 97 16 minecraft:air")
    motions = {}
    for x, name, kind in ((8, "shark", "naturalist:great_white_shark"), (20, "control", "minecraft:cow")):
        for dx, level in ((-1, 0), (0, 3), (1, 6)):
            command(f"setblock {x+dx} 92 6 minecraft:water[level={level}]")
        command(f"summon {kind} {x+0.5} 92.01 6.5 {{NoAI:1b,NoGravity:1b,Rotation:[0f,0f],Tags:[SharkCurrent{name}]}}")
    command("tick step 1")
    time.sleep(0.3)
    for x, name in ((8, "shark"), (20, "control")):
        rows = command(f"data get entity @e[tag=SharkCurrent{name},limit=1] Motion")
        position = next([float(v) for v in re.findall(r"(-?[0-9.eE+-]+)d", row)]
                        for row in rows if "following entity data:" in row)
        motions[name] = [round(v, 6) for v in position]
    return motions


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "shark-overworld-current-first-tick"])
