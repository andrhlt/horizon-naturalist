"""Observe fresh raw and cooked death drops from Java's original animals."""
import argparse
from pathlib import Path
import time

import deer_behavior
from java_reference import run_server


MEAT = (
    ("boar", "minecraft:porkchop", "minecraft:cooked_porkchop"),
    ("turkey", "naturalist:drumstick", "naturalist:cooked_drumstick"),
    ("capybara", "naturalist:bushmeat", "naturalist:cooked_bushmeat"),
)


def capture_loot(command, _):
    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:mob_drops true")
    command("forceload add 0 0")
    time.sleep(2.0)
    command("fill 1 100 1 15 100 15 minecraft:stone")
    command("fill 1 101 1 15 105 15 minecraft:air")
    command("tick freeze")

    def found(item):
        rows = command(f'execute if entity @e[type=minecraft:item,nbt={{Item:{{id:"{item}"}}}},limit=1] run say meat-found')
        return any("meat-found" in row for row in rows)

    try:
        result = {}
        for species, raw, cooked in MEAT:
            result[species] = {}
            for fire in (0, 120):
                command("kill @e[tag=meat_fixture]")
                command("kill @e[type=minecraft:item]")
                command("tick step 25")
                time.sleep(1.5)
                command("kill @e[type=minecraft:item]")
                # Java's capybara table rolls zero or one bushmeat per death.
                for _ in range(12 if species == "capybara" else 1):
                    rows = command(f"summon naturalist:{species} 8.5 101 8.5 "
                                   f"{{Tags:[meat_fixture],NoAI:1b,PersistenceRequired:1b,Fire:{fire}s}}")
                    assert any("Summoned" in row for row in rows), rows
                rows = command("execute as @e[tag=meat_fixture] run damage @s 99 minecraft:generic")
                assert any("damage" in row for row in rows), rows
                command("tick step 23")
                time.sleep(1.4)
                result[species]["burning" if fire else "normal"] = {"raw": found(raw), "cooked": found(cooked)}
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
    deer_behavior.capture = capture_loot
    run_server(options, ["sh", "run.sh", "nogui", "--world", "land-loot-fixture"])
