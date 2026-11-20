"""Check a real world reopen preserves the Ostrich's health, tame state, and tracked eggs."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import time


def capture(version, java):
    with tempfile.TemporaryDirectory(prefix="nat-ostrich-save-", ignore_cleanup_errors=True) as temp:
        root = Path(temp)
        socket = root / "ostrich.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def data(entity, path):
            return next((row.split("following entity data: ", 1)[1]
                         for row in cmd(f"data get entity {entity} {path}") if "following entity data: " in row), None)

        def launch():
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Ostrich Persistence", "--creative", "--test-terrain", "--render-distance", 4, "--wait")

        def quit_game():
            ctl("quit")
            deadline = time.monotonic() + 12
            while socket.exists() and time.monotonic() < deadline:
                time.sleep(0.1)

        pet = "@e[type=naturalist:ostrich,tag=saved,limit=1]"

        def observed():
            health_path = 'attributes[{id:"minecraft:max_health"}].base'
            saved_base = data(pet, health_path)
            rows = cmd(f"attribute {pet} minecraft:max_health base get")
            base = next(float(row.rsplit(" is ", 1)[1].split()[0]) for row in rows if " is " in row)
            bits = {key: data(pet, key) for key in ("UUID", "Owner", "Sitting", "DyeColor", "Health", "Saddled", "HasEgg", "OwnedEggs", "FollowingOwner")}
            return {"live_base": base, "saved_base": None if saved_base is None else float(saved_base.rstrip("d")), **bits}

        try:
            launch()
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 103 10.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 105 15 minecraft:air")
            cmd("tp @s 8.5 100 10.5")
            owner = data("@s", "UUID")
            packed_egg = (8 << 38) | (8 << 12) | 100
            cmd("gamerule minecraft:random_tick_speed 0")
            cmd("setblock 8 100 8 naturalist:ostrich_egg")
            reply = cmd(f'summon naturalist:ostrich 6.5 100 8.5 {{Tags:[saved],Owner:{owner},'
                        f'Sitting:1b,DyeColor:10,Saddled:1b,HasEgg:1b,FollowingOwner:0b,'
                        f'OwnedEggs:[L;{packed_egg}L],Health:7f,NoAI:1b}}')
            assert any("Summoned" in row for row in reply), reply
            ctl("wait", 8)
            before = observed()
            print(json.dumps({"before": before}), flush=True)
            cmd("save-all")
            quit_game()

            launch()
            ctl("wait", 15)
            after = observed()
            print(json.dumps({"after": after}), flush=True)
            assert before == after, (before, after)
            assert before["live_base"] == java["attributes"][0], before
            assert before["saved_base"] in (None, before["live_base"]), before
            assert before["Owner"] == owner, before
            expected = {"Sitting": "1b", "DyeColor": "10", "Health": "7.0f", "Saddled": "1b",
                        "HasEgg": "1b", "FollowingOwner": "0b", "OwnedEggs": f"[L; {packed_egg}L]"}
            for key, value in expected.items():
                assert before[key].replace(" ", "") == value.replace(" ", ""), (key, before[key], value)
        finally:
            if socket.exists():
                quit_game()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("java_reference", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.java_reference.read_text()))
