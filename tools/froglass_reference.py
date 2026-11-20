"""Measure the original Froglass drops from real Frog predation."""
import argparse
from pathlib import Path
import time


COLORS = {"blue": "azure", "green": "verdant", "red": "crimson"}
AREA = "x=7,y=102,z=7,distance=..13"


def capture_froglass(command, wait):
    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:mob_drops true")
    command("gamerule minecraft:advance_time false")
    command("forceload add 0 0 15 15")
    wait(30)
    command("fill 2 101 2 13 101 13 minecraft:stone")
    command("fill 2 102 2 13 109 13 minecraft:air")
    command("time set 2000")
    result = {}
    for variant in (*COLORS, "kill_control", "drops_disabled"):
        command("kill @e[tag=froglass_fixture]")
        command(f"kill @e[type=minecraft:item,{AREA}]")
        wait(25)
        if variant == "drops_disabled":
            command("gamerule minecraft:mob_drops false")
        coat = variant if variant in COLORS else "blue"
        command(f'summon naturalist:dragonfly 7.8 102 7.5 {{NoAI:1b,NoGravity:1b,Tags:[froglass_fixture],Variant:"naturalist:{coat}"}}')
        if variant == "kill_control":
            command("kill @e[type=naturalist:dragonfly,tag=froglass_fixture]")
        else:
            command("summon minecraft:frog 6.5 102 7.5 {PersistenceRequired:1b,Tags:[froglass_fixture]}")
        for _ in range(6):
            wait(30)
            if not any("dragonfly-present" in row for row in command(
                "execute if entity @e[type=naturalist:dragonfly,tag=froglass_fixture] run say dragonfly-present"
            )):
                break
        else:
            raise AssertionError(f"Dragonfly stayed live in {variant}")
        drops = []
        for color in COLORS.values():
            item = f"naturalist:{color}_froglass"
            check = (f'execute if entity @e[type=minecraft:item,{AREA},nbt={{Item:{{id:"{item}",count:1}}}}] '
                     'run say froglass-dropped')
            if any("froglass-dropped" in row for row in command(check)):
                drops.append(item)
        result[variant] = drops
    return result


if __name__ == "__main__":
    import java_reference

    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = lambda command, _: capture_froglass(command, lambda ticks: time.sleep(ticks / 20))
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "froglass-parity-fixture"])
