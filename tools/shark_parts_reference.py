"""Record pinned Java arrow damage at each separate Great White Shark body part."""
import argparse
import re
import time
from pathlib import Path
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 63 15")
    command("tick freeze")
    command("fill 3 91 1 47 91 16 minecraft:stone")
    command("fill 3 92 1 47 97 16 minecraft:air")
    result = {}
    shark = "@e[type=naturalist:great_white_shark,tag=SharkPartFixture,limit=1]"
    def health():
        rows = command(f"data get entity {shark} Health")
        return next(float(match[1]) for row in rows if (match := re.search(r"data: ([0-9.eE+-]+)f", row)))
    for index, (name, z) in enumerate((("body", 6.5), ("head", 8.5), ("inner_tail", 5.0), ("tail", 3.6), ("outside", 2.5))):
        x = 8.5 + index * 8
        command("kill @e[tag=SharkPartFixture]")
        command("tick step 21")
        time.sleep(1.2)
        command(f"summon naturalist:great_white_shark {x} 92 6.5 {{NoAI:1b,NoGravity:1b,Rotation:[0f,0f],Tags:[SharkPartFixture]}}")
        command("tick step 2")
        time.sleep(0.3)
        before = health()
        command(f"summon minecraft:arrow {x + 1.3} 92.8 {z}"
                + " {NoGravity:1b,Motion:[-1.4d,0d,0d],damage:3.0d,Tags:[SharkPartFixture]}")
        readings = []
        for _ in range(4):
            command("tick step 1")
            time.sleep(0.2)
            readings.append(health())
        print(name, before, readings, flush=True)
        result[name] = readings[-1]
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "shark-overworld-parts-isolated"])
