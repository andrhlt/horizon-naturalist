"""Compare Komodo diet, breeding, and delayed aggression with Java."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version, reference):
    with tempfile.TemporaryDirectory(prefix="nat-komodo-", ignore_cleanup_errors=True) as path:
        home = Path(path)
        socket = home / "komodo.sock"

        def ctl(*args):
            process = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                     check=True, capture_output=True, text=True, timeout=75)
            return json.loads(process.stdout.splitlines()[-1])

        def cmd(line):
            result = ctl("command", line)
            if not result["ok"]:
                raise RuntimeError((line, result))
            return result["chat"]

        def read(who, key):
            return next((row.split("following entity data: ", 1)[1]
                         for row in cmd(f"data get entity {who} {key}")
                         if "following entity data: " in row), None)

        def summon(nbt, x=8.5):
            rows = cmd(f"summon naturalist:komodo_dragon {x} 101 8.5 {nbt}")
            assert any("Summoned" in row for row in rows), rows

        def interact(food):
            rows = cmd(f"item replace entity @s weapon.mainhand with {food} 2")
            assert any("Replaced" in row for row in rows), (food, rows)
            ctl("look-at", 8.5, 101.5, 8.5)
            ctl("mouse", "right", "click")
            ctl("wait", 2)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--world", "Naturalist Komodo Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("forceload add 0 0")
            cmd("fill 4 100 4 13 100 13 minecraft:stone")
            cmd("fill 4 101 4 13 104 13 minecraft:stone hollow")
            cmd("fill 5 101 5 12 104 12 minecraft:air")
            cmd("tp @s 8.5 101 6.5")
            ctl("wait", 10)
            cmd("gamemode survival @s")
            cmd("time set noon")
            summon("{NoAI:1b,Tags:[dragon]}")
            dragon = "@e[type=naturalist:komodo_dragon,tag=dragon,limit=1]"
            result = {"variant": json.loads(read(dragon, "Variant") or "null")}
            for attribute in ("max_health", "movement_speed", "attack_damage", "step_height"):
                rows = cmd(f"attribute {dragon} minecraft:{attribute} base get")
                result[attribute] = next((float(match[1]) for row in rows
                                          if (match := re.search(r" is ([0-9.eE+-]+)", row))), None)
            accepted = []
            items = sorted(set(reference["food"]) | {"minecraft:cod", "minecraft:rotten_flesh",
                                                    "minecraft:melon_slice", "minecraft:spider_eye"})
            for food in items:
                cmd(f"data merge entity {dragon} {{Health:30f,Age:0,InLove:0}}")
                interact(food)
                if int(read(dragon, "InLove") or 0) > 0:
                    accepted.append(food)
            result["food"] = accepted
            result["fed_adult"] = bool(accepted)
            cmd(f"data merge entity {dragon} {{Health:20f,InLove:0}}")
            interact("minecraft:beef")
            result["healed_health"] = float((read(dragon, "Health") or "0").rstrip("fd"))
            result["heal_consumed"] = bool(re.search(r"count:\s*1\b", read("@s", "SelectedItem") or ""))
            cmd(f'data merge entity {dragon} {{Health:30f,Age:0,InLove:600,NoAI:0b,Variant:"fixture:first_parent"}}')
            summon('{InLove:600,Tags:[mate],Variant:"fixture:second_parent"}', x=9.9)
            ctl("wait", 105)
            nearby = ctl("entities", 20, "naturalist:komodo_dragon")["entities"]
            result["same_species_mate"] = any(mob["baby"] for mob in nearby)
            if result["same_species_mate"]:
                baby = "@e[type=naturalist:komodo_dragon,tag=!dragon,tag=!mate,limit=1]"
                variant = json.loads(read(baby, "Variant") or "null")
                assert variant in {"fixture:first_parent", "fixture:second_parent"}, variant

            cmd("kill @e[type=naturalist:komodo_dragon]")
            cmd("clear @s")
            cmd("tp @s 8.5 101 6.5")
            cmd("time set midnight")
            cmd("effect give @s minecraft:resistance infinite 1 true")
            summon("{Tags:[hunter]}")
            ctl("wait", 275)
            before = ctl("state")["player"]
            assert before["health"] == 20.0 and all(e["id"] != "poison" for e in before["effects"]), before
            for _ in range(65):
                ctl("wait", 4)
                after = ctl("state")["player"]
                poison = next((e for e in after["effects"] if e["id"] == "poison"), None)
                if poison:
                    break
            result["attack_landed"] = poison is not None and after["health"] < before["health"]
            result["poison_duration"] = poison["duration_ticks"] if poison else 0
            result["poison_amplifier"] = poison["amplifier"] if poison else None
            return result
        finally:
            if socket.exists():
                ctl("quit")


def compare(output, reference):
    for key, actual in output.items():
        target = reference[key]
        if key == "poison_duration":
            assert 91 <= actual <= target, (key, actual, target)
        elif type(target) is float:
            assert actual is not None and abs(actual - target) < 1e-6, (key, actual, target)
        else:
            assert actual == target, (key, actual, target)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    options = parser.parse_args()
    reference = json.loads(options.fixture.read_text())
    output = capture(options.version, reference)
    print(json.dumps(output, indent=2), flush=True)
    compare(output, reference)
