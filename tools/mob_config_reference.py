"""Capture configured entity removal in Naturalist 2.0.7 NeoForge."""
from argparse import ArgumentParser
import json
from pathlib import Path
import time
import java_reference


def exercise(command, wait):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    command("tick freeze")
    names = ("naturalist:deer", "naturalist:capybara", "naturalist:butterfly",
             "naturalist:caterpillar", "minecraft:cow")
    summoned = {}
    for index, species in enumerate(names):
        command(f"kill @e[tag=config_animal_{index}]")
        result = command(f"summon {species} {5.5 + index} 150 8.5 "
                         f"{{NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[config_animal_{index}]}}")
        summoned[species] = any("Summoned new" in line for line in result)
    command("tick step 3")
    wait()
    survivors = {}
    for index, species in enumerate(names):
        result = command(f"data get entity @e[tag=config_animal_{index},limit=1] Health")
        survivors[species] = any("following entity data" in line for line in result)
    return {"summoned": summoned, "survivors": survivors}


def main():
    cli = ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        cli.add_argument(name, type=Path)
    args = cli.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = lambda command, _: exercise(command, lambda: time.sleep(.3))
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "deer-config-fixture"])


if __name__ == "__main__":
    main()
