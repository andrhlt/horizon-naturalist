"""Capture Java zombies pursuing Naturalist nests through actual server ticks."""
import argparse
from pathlib import Path
import time
import java_reference


def capture(command, _):
    command("forceload add 0 0")
    command("difficulty normal")
    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:random_tick_speed 0")
    command("time set 18000")
    command("fill 1 91 1 15 91 14 minecraft:stone")
    command("fill 1 92 1 15 97 14 minecraft:air")
    command("tick freeze")
    result = {}
    for egg, allowed in (("alligator", True), ("tortoise", True), ("tortoise", False)):
        command("gamerule minecraft:mob_griefing " + str(allowed).lower())
        command(f"setblock 8 92 8 naturalist:{egg}_egg[eggs=4]")
        command("summon minecraft:zombie 5.5 92 8.5 {PersistenceRequired:1b,Tags:[raider]}")
        command("tick step 222")
        time.sleep(12)
        check = command("execute if block 8 92 8 minecraft:air")
        result[f"{egg}_{allowed}"] = any("test passed" in line.lower() for line in check)
        command("kill @e[tag=raider]")
        command("tick step 24")
        time.sleep(2)
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "naturalist-zombie-egg-fixture"])
