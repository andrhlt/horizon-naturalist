"""Capture vertical motion of Boar, Turkey and Capybara in Java or Horizon."""
import argparse
from pathlib import Path
import re
import time

import deer_behavior
from java_reference import run_server


def descent(command, settle):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    settle(1.0)
    command("tick freeze")
    try:
        command("kill @e[tag=gravity_fixture]")
        command("fill 1 100 1 15 100 15 minecraft:stone")
        command("fill 1 101 1 15 155 15 minecraft:air")
        animals = ("boar", "turkey", "capybara")
        for slot, animal in enumerate(animals):
            x = 3.5 + 4 * slot
            command(f"summon naturalist:{animal} {x} 150 8.5 "
                    f"{{Tags:[gravity_fixture,fall_{slot}],PersistenceRequired:1b}}")
        result = {animal: [] for animal in animals}
        for steps in (0, 1, 3, 4, 8, 8, 8):
            if steps:
                command(f"tick step {steps}")
                settle(steps / 20 + 0.2)
            for slot, animal in enumerate(animals):
                rows = command(f"data get entity @e[tag=fall_{slot},limit=1] Pos[1]")
                value = next((float(match[1]) for row in rows if (match := re.search(
                    r"following entity data: (-?\d+(?:\.\d+)?(?:[Ee][+-]?\d+)?)d\b", row))), None)
                assert value is not None, (animal, steps, rows)
                result[animal].append(value)
        return result
    finally:
        command("tick unfreeze")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("server", type=Path)
    parser.add_argument("java_home", type=Path)
    parser.add_argument("output", type=Path)
    options = parser.parse_args()
    options.variants = False
    options.grazing = False
    options.behavior = True
    deer_behavior.capture = lambda command, _: descent(command, time.sleep)
    run_server(options, ["sh", "run.sh", "nogui", "--world", "gravity-fixture"])
