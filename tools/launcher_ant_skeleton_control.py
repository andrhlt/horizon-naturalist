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
            cmd("time set midnight")
            cmd("summon naturalist:ant 3.5 100 3.5 {Tags:[ant],HillCooldown:1200}")
            cmd("summon minecraft:skeleton 4.5 100 3.5 {Tags:[prey],NoAI:1b,PersistenceRequired:1b}")
            ant="@e[tag=ant,limit=1]"; sheep="@e[tag=prey,limit=1]"
            before=data(sheep,"Health");assert before
            for step in range(30):
                ctl("wait",20)
                health=data(sheep,"Health")
                if health != before:
                    print(json.dumps({"before":before,"after":health,"ticks_min":20*(step+1)}),flush=True)
                    raise AssertionError("Naturalist Ant attacked a unprovoked skeleton")
            print(json.dumps({"before":before,"after":health,"ticks_min":600}),flush=True)
        finally:
            if socket.exists():
                quit_game()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--jar", type=Path, required=True)
    args = parser.parse_args()
    capture(args.version, args.jar)
