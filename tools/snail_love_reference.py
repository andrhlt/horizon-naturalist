"""Read live breeding state around a Java Snail NoAI data reload."""
import argparse
from pathlib import Path
import time
import deer_behavior
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    time.sleep(1)
    command("tick freeze")
    target = "@e[tag=snail_love_test,limit=1]"
    command(f"kill {target}")
    command('summon naturalist:snail 8.5 100 8.5 {NoAI:1b,InLove:600,LoveCause:[I;1,2,3,4],Tags:[snail_love_test]}')

    def read():
        return [next(line.rsplit("following entity data: ", 1)[1].strip()
                     for line in command(f"data get entity {target} {key}") if "following entity data:" in line)
                for key in ("InLove", "LoveCause")]

    before = read()
    command(f"data merge entity {target} {{NoAI:0b}}")
    after = read()
    return {"before": before, "after": after}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    deer_behavior.capture = capture
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "snail-love-fixture"])
