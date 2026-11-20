"""Sample original Java Tortoise command spawn finalization and explicit NBT."""
from argparse import ArgumentParser
from pathlib import Path
import time
import java_reference


def capture(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    time.sleep(2)
    command("tick freeze")
    target = "@e[type=naturalist:tortoise,limit=1]"
    output = {}
    for mode, nbt in (("normal", ""), ("explicit_wild", " {NoAI:1b,Health:17.0f}"),
                      ("explicit_tamed", " {NoAI:1b,Health:17.0f,Owner:[I;1,2,3,4]}")):
        output[mode] = []
        for _ in range(2):
            command("kill @e[type=naturalist:tortoise]")
            command("summon naturalist:tortoise 8.5 150 8.5" + nbt)
            stats = []
            for attr in ("max_health", "attack_damage"):
                report = command(f"attribute {target} minecraft:{attr} base get")
                stats.append(next(float(line.split(" is ", 1)[1]) for line in report if " is " in line))
            report = command(f"data get entity {target} Health")
            stats.append(next(float(line.split("following entity data: ", 1)[1].removesuffix("f"))
                              for line in report if "following entity data: " in line))
            output[mode].append(stats)
    return output


if __name__ == "__main__":
    cli = ArgumentParser(description=__doc__)
    for key in ("server", "java_home", "output"):
        cli.add_argument(key, type=Path)
    args = cli.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "tortoise-fixture"])
