"""Reopen a real Horizon world with an Ant carrying a live apple."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile
import time
from uuid import UUID


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-ant-save-", ignore_cleanup_errors=True) as temporary:
        root = Path(temporary)
        socket = root / "ant.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def data(entity, path):
            marker = "following entity data: "
            return next((row.split(marker, 1)[1] for row in cmd(f"data get entity {entity} {path}")
                         if marker in row), None)

        def launch():
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Carried Food Persistence", "--creative", "--test-terrain", "--render-distance", 4, "--wait")

        def quit_game():
            ctl("quit")
            deadline = time.monotonic() + 12
            while socket.exists() and time.monotonic() < deadline:
                time.sleep(0.1)

        def as_uuid(tag):
            values = [int(value) & 0xFFFFFFFF for value in re.findall(r"-?\d+", tag)]
            return str(UUID(bytes=b"".join(value.to_bytes(4, "big") for value in values)))

        def position(entity):
            return [float(value) for value in re.findall(r"-?\d+(?:\.\d+)?", data(entity, "Pos") or "")]

        def observe(ant, food):
            ant_at, food_at = position(ant), position(food)
            return {
                "carrier": data(food, "Ant") == data(ant, "UUID"),
                "food": data(food, "Item.id") == '"minecraft:apple"',
                "one": data(food, "Item.count") == "1",
                "offset": round(food_at[1] - ant_at[1], 4) if len(ant_at) == len(food_at) == 3 else None,
            }

        try:
            launch()
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:random_tick_speed 0")
            cmd("tp @s 8.5 103 14.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 105 15 minecraft:air")
            cmd("tp @s 8.5 100 14.5")
            for lane, x in enumerate((2.5, 13.5)):
                cmd(f"summon naturalist:ant {x} 100 3.5 {{Tags:[lane{lane}],HillCooldown:1200}}")
            for x in (2.5, 13.5):
                cmd(f'summon minecraft:item {x + 0.18} 100 3.5 {{Item:{{id:"minecraft:apple",count:2}},PickupDelay:0s,NoGravity:1b}}')

            carriers = []
            for _ in range(12):
                carriers = [(ant, tag) for lane in range(2)
                            if (ant := f"@e[tag=lane{lane},limit=1]")
                            and (tag := data(ant, "CarriedFood"))]
                if carriers:
                    break
                ctl("wait", 8)
            assert len(carriers) == 1, carriers
            ant, saved_food = carriers[0]
            food = as_uuid(saved_food)
            before = observe(ant, food)
            print(json.dumps({"before": before, "food_uuid": food}), flush=True)
            cmd("save-all")
            quit_game()

            launch()
            ctl("wait", 14)
            current_food = data(ant, "CarriedFood")
            after = observe(ant, food)
            after["same_food_uuid"] = current_food == saved_food
            print(json.dumps({"after": after}), flush=True)
            assert after.pop("same_food_uuid") and after == before
            assert before == {"carrier": True, "food": True, "one": True, "offset": 0.55}, before
        finally:
            if socket.exists():
                quit_game()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    capture(parser.parse_args().version)
