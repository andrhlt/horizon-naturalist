"""Exercise duck pet interactions through the Horizon client's normal inputs."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, reference):
    with tempfile.TemporaryDirectory(prefix="nat-pet-", ignore_cleanup_errors=True) as work:
        home = Path(work)
        socket = home / "pet.sock"

        def control(*words):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, words)],
                                    check=True, capture_output=True, text=True, timeout=75)
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            result = control("command", line)
            if not result["ok"]:
                raise RuntimeError((line, result))
            return result["chat"]

        def value(target, key):
            rows = command(f"data get entity {target} {key}")
            return next((row.split("following entity data: ", 1)[1] for row in rows
                         if "following entity data: " in row), None)

        def inventory():
            return "\n".join(command("data get entity @s Inventory"))

        def replace(item, count=1):
            command(f"item replace entity @s weapon.mainhand with {item} {count}")

        try:
            control("launch", "--headless", "--game", version / "horizon-client",
                    "--resources", version / ".local/horizon-client/26.3", "--home", home,
                    "--game-dir", home, "--world", "Naturalist Duck Pets", "--creative",
                    "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 7.5 101 6.5")
            control("wait", 20)
            command("fill 1 100 1 15 100 15 minecraft:stone")
            command("fill 1 101 1 15 106 15 minecraft:air")
            command("tp @s 7.5 101 6.5")
            control("wait", 4)
            command("gamemode survival @s")
            summoned = command("summon naturalist:duck 7.5 101 8.5 {Age:-24000,NoAI:1b,Tags:[duck_pet]}")
            assert any("Summoned" in line for line in summoned), summoned
            control("wait", 4)
            duck = "@e[type=naturalist:duck,tag=duck_pet,limit=1]"
            rows = control("entities", 10, "naturalist:duck")["entities"]
            assert len(rows) == 1, (rows, control("entities", 30), control("state"))

            def interact():
                control("look", 0, 32)
                rows = control("mouse", "right", "click")
                control("wait", 2)
                assert rows.get("ok", True), rows

            replace("minecraft:cod", 48)
            for attempts in range(1, 32):
                interact()
                if value(duck, "Owner") is not None:
                    break
            else:
                raise AssertionError("Duckling did not tame after 31 clicks")
            assert "minecraft:cod" in inventory(), inventory()
            output = {"tamed": True, "taming_attempts": attempts}

            command("clear @s")
            interact()
            sitting = value(duck, "Sitting")
            output["sit"] = sitting in {"true", "1b", "1"}
            print("Sitting after an empty-hand click:", sitting, flush=True)
            replace("minecraft:red_dye", 2)
            interact()
            output["dye"] = int(value(duck, "DyeColor"))
            dye_stack = inventory()
            assert 'minecraft:red_dye' in dye_stack and 'count: 1' in dye_stack, dye_stack
            replace("minecraft:shears")
            interact()
            output["dye_cleared"] = value(duck, "DyeColor") == "-1"
            assert '"minecraft:damage": 1' in inventory(), inventory()
            replace("naturalist:whistle")
            interact()
            output["following"] = value(duck, "FollowingOwner") in {"true", "1b", "1"}
            standing = value(duck, "Sitting")
            output["stand"] = standing in {"false", "0b", "0"}
            print("Sitting after whistle:", standing, flush=True)

            command(f"data merge entity {duck} {{Health:1.0f,Age:-12345}}")
            replace("minecraft:salmon", 2)
            interact()
            output["healed"] = float(value(duck, "Health").rstrip("f"))
            command("fill 5 100 9 10 100 14 minecraft:air")
            replace("minecraft:bucket")
            before = control("state")
            interact()
            control("wait", 6)
            captured = inventory()
            output["bucket"] = "naturalist:duck_bucket" in captured
            output["bucket_variant"] = "naturalist:duck" if 'Variant: "naturalist:duck"' in captured else None
            remaining = control("entities", 10, "naturalist:duck")["entities"]
            assert output["bucket"] and not remaining, (output, captured, before["looking_at"], remaining)
            control("look-at-block", 7, 100, 8)
            control("mouse", "right", "click")
            control("wait", 4)
            spawned = control("entities", 10, "naturalist:duck")["entities"]
            output["released"] = len(spawned) == 1
            assert output["released"], spawned
            assert 'id: "minecraft:bucket"' in inventory(), inventory()
        finally:
            if socket.exists():
                control("quit")

    print(json.dumps(output, indent=2), flush=True)
    keys = ("tamed", "sit", "dye", "dye_cleared", "following", "stand", "healed", "bucket", "bucket_variant")
    for key in keys:
        assert output[key] == reference[key], (key, output[key], reference[key])


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.fixture.read_text()))
