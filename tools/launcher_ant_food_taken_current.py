"""Reopen a real Horizon world with an Ant carrying a live apple."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile
import time
import shutil
import hashlib
from prepare import JAR_SHA
from uuid import UUID


def capture(version, jar):
    with tempfile.TemporaryDirectory(prefix="nat-ant-save-", ignore_cleanup_errors=True) as temporary:
        root = Path(temporary)
        socket = root / "ant.sock"
        (root / "jars").mkdir()
        if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
            raise ValueError("expected pinned Naturalist Java jar")
        shutil.copy2(jar, root / "jars" / f"{JAR_SHA}.jar")
        (root / "horizon-mods.txt").write_text("mod.naturalist:true\n")

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
            cmd("gamerule minecraft:mob_drops false")
            owner = data("@s", "UUID")
            cmd("gamemode survival @s")
            for tame in (False, True):
                cmd("kill @e[type=naturalist:ant]")
                cmd("kill @e[type=minecraft:item]")
                cmd("clear @s")
                cmd("tp @s 8.5 100 14.5")
                ctl("wait", 60)
                owned = f",Owner:{owner}" if tame else ""
                ant_id = 1 if not tame else 11
                food_id = 5 if not tame else 15
                ant_uuid = f"[I;{ant_id},2,3,4]"
                food_uuid = f"[I;{food_id},6,7,8]"
                cmd(f'summon minecraft:item 2.5 100.55 3.5 {{UUID:{food_uuid},Ant:{ant_uuid},Item:{{id:"minecraft:apple",count:1}},PickupDelay:0s,NoGravity:1b}}')
                cmd(f"summon naturalist:ant 2.5 100 3.5 {{UUID:{ant_uuid},CarriedFood:{food_uuid},NoAI:1b,Tags:[worker{int(tame)}],HillCooldown:1200{owned}}}")
                ant = f"@e[tag=worker{int(tame)},limit=1]"
                tag = data(ant, "CarriedFood")
                assert tag, ("missing linked food",tame)
                food = as_uuid(tag)
                cmd(f"data merge entity {ant} {{NoAI:1b}}")
                where = position(ant)
                cmd(f"tp @s {where[0]} {where[1]} {where[2]}")
                for _ in range(20):
                    ctl("wait",4)
                    if data(ant,"CarriedFood") is None: break
                print(json.dumps({"carried":data(ant,"CarriedFood"),"food":data(food,"Item"),"food_pos":data(food,"Pos"),"ant_pos":data(ant,"Pos"),"player":data("@s","Pos"),"inventory":data("@s","Inventory")}),flush=True)
                assert data(ant,"CarriedFood") is None, ("player did not take food",tame)
                anger = data(ant,"anger_end_time")
                target = data(ant,"angry_at")
                print(json.dumps({"tame":tame,"anger_end_time":anger,"angry_at":target,"player_uuid":owner}),flush=True)
                if tame:
                    assert target is None, target
                else:
                    assert target == owner, (target, owner)
            print("PASS",flush=True)
        finally:
            if socket.exists():
                quit_game()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--jar", type=Path, required=True)
    args = parser.parse_args()
    capture(args.version, args.jar)
