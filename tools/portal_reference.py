"""Capture original Naturalist portal transit in a headless Java world."""
import argparse
from pathlib import Path
import time

import deer_behavior
from java_reference import run_server


ANIMALS = (
    "minecraft:pig", "naturalist:boar", "naturalist:turkey", "naturalist:capybara",
    "naturalist:desert_scorpion", "naturalist:jungle_scorpion", "naturalist:duck",
    "naturalist:butterfly", "naturalist:caterpillar", "naturalist:bass",
)


def capture_portals(command, wait):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 31 31")
    wait(40)
    command("kill @e[tag=portal_fixture]")
    command("fill 1 100 1 29 100 29 minecraft:stone")
    command("fill 1 101 1 29 107 29 minecraft:air")
    for slot, animal in enumerate(ANIMALS):
        x, z = 3 + slot % 5 * 4, 8 + slot // 5 * 5
        tag = f"portal_sample_{slot}"
        command(f"fill {x - 1} 100 {z} {x + 2} 104 {z} minecraft:obsidian outline")
        rows = command(f"summon {animal} {x + 0.5} 101 {z + 0.5} "
                       f"{{Tags:[portal_fixture,{tag}],NoAI:1b,PersistenceRequired:1b}}")
        assert any("Summoned " in row for row in rows), rows
        command(f"fill {x} 101 {z} {x + 1} 103 {z} minecraft:nether_portal[axis=x]")
    wait(100)
    results = {}
    for slot, animal in enumerate(ANIMALS):
        target = f"@e[tag=portal_sample_{slot},distance=..500,limit=1]"
        dimensions = {}
        for dimension in ("overworld", "the_nether"):
            rows = command(f"execute in minecraft:{dimension} if entity {target} run say portal-found")
            dimensions[dimension] = any("portal-found" in row for row in rows)
        assert sum(dimensions.values()) == 1, (animal, dimensions)
        results[animal] = dimensions["the_nether"]
    assert results["minecraft:pig"], "The vanilla control did not cross the portal"
    return results


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("server", type=Path)
    parser.add_argument("java_home", type=Path)
    parser.add_argument("output", type=Path)
    options = parser.parse_args()
    options.variants = False
    options.grazing = False
    options.behavior = True
    deer_behavior.capture = lambda command, _: capture_portals(command, lambda ticks: time.sleep(ticks / 20))
    run_server(options, ["sh", "run.sh", "nogui", "--world", "portal-fixture"])
