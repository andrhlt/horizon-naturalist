"""Measure original Shark parent displacement from blocks touching only its parts."""
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
    result = {}
    for index, (label, block_z) in enumerate((("open", None), ("head", 8), ("tail", 3))):
        command("kill @e[tag=SharkPartObstacle]")
        command("tick step 21")
        time.sleep(1.2)
        x = 8 + 8 * index
        if block_z is not None:
            command(f"setblock {x} 92 {block_z} minecraft:stone")
        command(f"summon naturalist:great_white_shark {x + 0.5} 92 6.5 {{NoAI:1b,NoGravity:1b,Rotation:[0f,0f],Tags:[SharkPartObstacle]}}")
        command("tick step 1")
        time.sleep(0.3)
        rows = command("data get entity @e[tag=SharkPartObstacle,limit=1] Pos")
        position = next([float(v) for v in re.findall(r"(-?[0-9.eE+-]+)d", row)]
                        for row in rows if "following entity data:" in row)
        result[label] = [round(a - b, 8) for a, b in zip(position, (x + 0.5, 92.0, 6.5))]
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "shark-overworld-obstruction"])
