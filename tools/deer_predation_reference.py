"""Compare wild and tamed wolves encountering their Naturalist deer prey."""
from argparse import ArgumentParser
from pathlib import Path
import time
import java_reference


def scenario(command, wait):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 47 15")
    command("difficulty normal")
    command("time set noon")
    command("tick freeze")
    command("kill @e[tag=wolf_deer_fixture]")
    command("fill 0 100 0 47 100 15 minecraft:stone")
    command("fill 0 101 0 47 107 15 minecraft:air")
    for label, x, owner in (("wild", 7, ""), ("tame", 43, "Owner:[I;1,2,3,4],")):
        command(f"summon naturalist:deer {x}.5 101 8.5 "
                f"{{Health:10f,NoAI:1b,Tags:[wolf_deer_fixture,{label}_prey]}}")
        command(f"summon minecraft:wolf {x}.5 101 6.75 "
                f"{{{owner}PersistenceRequired:1b,Tags:[wolf_deer_fixture,{label}_wolf]}}")
    command("tick step 140")
    wait()
    result = {}
    for label in ("wild", "tame"):
        rows = command(f"data get entity @e[tag={label}_prey,limit=1] Health")
        health = next((float(row.split("following entity data: ", 1)[1].removesuffix("f"))
                       for row in rows if "following entity data: " in row), 0.0)
        result[f"{label}_wolf_hurts_deer"] = health < 10.0
    return result


def main():
    cli = ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        cli.add_argument(name, type=Path)
    args = cli.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = lambda command, _: scenario(command, lambda: time.sleep(8.2))
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "naturalist-wolf-predation-fixture"])


if __name__ == "__main__":
    main()
