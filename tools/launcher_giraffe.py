"""Compare Giraffe feeding, taming, raw baby age, and ridden motion in headless Play."""
import argparse
import json
import math
from pathlib import Path
import subprocess
import tempfile
from launcher_managed import launch_managed, wait_managed


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-giraffe-", ignore_cleanup_errors=True) as work:
        home = Path(work)
        socket = home / "ctl.sock"

        def call(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, check=True, timeout=60)
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            return call("command", line)["chat"]

        def field(entity, key):
            marker = "following entity data: "
            rows = command(f"data get entity {entity} {key}")
            return next((row.split(marker, 1)[1] for row in rows if marker in row), None)

        def item_count():
            return int(field("@s", "SelectedItem.count") or "0")

        def number(entity, key):
            value = field(entity, key)
            return float(value.rstrip("fdbls")) if value else 0.0

        def aim_at_nearest_giraffe():
            entries = call("entities", 12, "naturalist:giraffe")["entities"]
            assert entries, "No Giraffe available for interaction"
            target = min(entries, key=lambda entry: entry["distance"])
            aimed = call("look-at", "entity", target["id"])["target"]
            assert aimed.get("kind") == "entity" and aimed["entity"].get("id") == target["id"], (target, aimed)

        def click(item=None, count=1, pitch=0):
            if item is not None:
                command(f"item replace entity @s weapon.mainhand with minecraft:{item} {count}")
            else:
                command("item replace entity @s weapon.mainhand with minecraft:air")
            call("wait", 3)
            aim_at_nearest_giraffe()
            call("mouse", "right", "click")
            call("wait", 5)

        def position(kind):
            return [number(kind, f"Pos[{i}]") for i in range(3)]

        adult = "@e[tag=adult,limit=1]"
        baby = "@e[tag=baby,limit=1]"
        play = None
        try:
            play = launch_managed(version, socket, home, "Naturalist Giraffe",
                                  creative=True, test_terrain=True)
            command("gamerule minecraft:spawn_mobs false")
            call("wait", 12)
            command("fill 0 99 0 15 99 15 minecraft:stone")
            command("fill 0 100 0 15 109 15 minecraft:air")
            command("tp @s 8.5 100 6.5")
            command("gamemode survival @s")
            owner = field("@s", "UUID")
            command('summon naturalist:giraffe 8.5 100 9.0 {NoAI:1b,Tags:[adult]}')
            variant = field(adult, "variant")
            assert variant == '"minecraft:temperate"', ("summoned Giraffe variant", variant)
            click("apple", 3)
            results = {
                "default_adult_food_remaining": item_count(),
                "adult_love": number(adult, "InLove") > 575,
                "adult_tames_on_food": field(adult, "Owner") is not None,
            }
            command("tp @s 12.5 100 6.5")
            command('summon naturalist:giraffe 12.5 100 9.0 {Age:-2000,NoAI:1b,Tags:[baby]}')
            command("item replace entity @s weapon.mainhand with minecraft:apple 32")
            tries = 0
            while field(baby, "Owner") is None and tries < 32:
                aim_at_nearest_giraffe()
                call("mouse", "right", "click")
                call("wait", 5)
                tries += 1
            results["baby_taming"] = field(baby, "Owner") == owner
            results["baby_food_cost_per_attempt"] = item_count() == 32 - tries
            results["baby_keeps_young_age"] = -2000 <= number(baby, "Age") < -1600
            click(pitch=5)
            results["baby_empty_hand_mount"] = call("status").get("riding") is not None or position("@s")[1] > 102
            command(f"data merge entity {adult} {{Health:20f,Tame:1b,Owner:{owner},NoAI:1b}}")
            command("tp @s 8.5 100 6.5")
            print("owner fixture", {key: field(adult, key) for key in ("Tame", "Owner", "Age", "Health")},
                  "player", field("@s", "UUID"), flush=True)
            click("hay_block", 3)
            results["owned_food_remaining"] = item_count()
            results["owned_heal"] = number(adult, "Health")
            before = position(adult)
            command("tp @s 8.5 100 6.0")
            click()
            after = position("@s")
            results["owner_mounts"] = math.isclose(after[1] - before[1], 3.24, abs_tol=0.06)
            command(f"data merge entity {adult} {{NoAI:0b}}")
            call("hold", "w", 0.6)
            call("wait", 12)
            results["mounted_moves"] = math.dist(before, position(adult)) > 1.0
            return results
        finally:
            if socket.exists():
                call("quit")
            wait_managed(play)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.reference.read_text())
    actual = capture(args.version)
    print(json.dumps(actual, indent=2), flush=True)
    expected["adult_love"] = expected["adult_love"] == 600
    expected["baby_keeps_young_age"] = True
    expected["mounted_moves"] = True
    differences = {key: [expected[key], value] for key, value in actual.items() if value != expected[key]}
    assert not differences, differences
