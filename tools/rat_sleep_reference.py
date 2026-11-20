"""Measure Rat sleep in the actual pinned Java world."""
import argparse
import json
from pathlib import Path
import time


AREA = "rat_sleep_fixture"


def sample(command, wait, inspect):
    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:advance_time false")
    command("gamerule minecraft:random_tick_speed 0")
    command("weather clear")
    command("tick freeze")
    command("kill @e[type=!minecraft:player]")
    wait(25)
    command("fill 2 91 2 15 91 14 minecraft:stone")
    command("fill 2 92 2 15 98 14 minecraft:air")
    command(f"kill @e[tag={AREA}]")
    command("time set 1000")
    command(f"summon naturalist:rat 8.5 92 5.5 {{Tags:[{AREA}],PersistenceRequired:1b}}")
    wait(12)
    result = {"day": inspect()}
    command("time set 18000")
    wait(12)
    result["night"] = inspect()
    command(f"summon naturalist:rat 10.5 92 5.5 {{NoAI:1b,Tags:[{AREA}_other]}}")
    wait(12)
    result["other_rat"] = inspect()
    command(f"summon minecraft:cow 10.5 92 6.5 {{NoAI:1b,Tags:[{AREA}_cow]}}")
    wait(12)
    result["cow"] = inspect()
    command(f"kill @e[tag={AREA}_cow]")
    wait(42)
    for _ in range(8):
        if inspect()[1]:
            break
        wait(24)
    result["cow_gone"] = inspect()
    command("time set 1000")
    wait(12)
    result["morning"] = inspect()
    command(f"kill @e[tag={AREA}_other]")
    return result


def capture(command, _):
    command("forceload add 0 0 15 15")

    def wait(ticks):
        command(f"tick sprint {ticks}")
        time.sleep(.45)

    def inspect():
        for row in command(f"rat_sleep_probe @e[tag={AREA},limit=1]"):
            if "rat_sleep_probe [" in row:
                return json.loads(row.split("rat_sleep_probe ", 1)[1])
        raise RuntimeError("Java Rat sleep probe returned no value")

    return sample(command, wait, inspect)


if __name__ == "__main__":
    import java_reference

    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    with java_reference.deer_behavior.reference_probe(args.server, args.java_home, "RatSleepProbe.java"):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "rat-sleep-parity-fixture"])
