"""Verify an ant spawn egg preserves its explicit health override."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile
import time


def capture(version, jar):
    with tempfile.TemporaryDirectory(prefix="nat-ant-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        (root / "jars").mkdir()
        import hashlib
        import shutil
        from prepare import JAR_SHA
        if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
            raise ValueError("expected the pinned Naturalist jar")
        shutil.copyfile(jar, root / "jars" / f"{JAR_SHA}.jar")
        socket = root / "ant.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def data(path):
            return next((row.split(" data: ", 1)[1] for row in cmd("data get " + path)
                         if " data: " in row), None)

        def block(x, y, z, properties=""):
            return any("Test passed" in row for row in
                       cmd(f"execute if block {x} {y} {z} naturalist:ant_hill{properties}"))

        def give(item):
            cmd("item replace entity @s weapon.mainhand with " + item)
            ctl("wait", 4)

        def click_block(x, y, z):
            ctl("look-at-block", x, y, z, "up")
            ctl("mouse", "right", "click")
            ctl("wait", 6)

        def wait_until(check, ticks=600):
            for _ in range((ticks + 19) // 20):
                if result := check():
                    return result
                ctl("wait", 20)
            return None

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Ant Hill Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:random_tick_speed 0")
            cmd("tp @s 10.5 103 6.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 107 15 minecraft:air")
            cmd("tp @s 10.5 100 6.5")
            cmd("gamemode survival @s")
            owner = data("entity @s UUID")
            assert owner and owner.startswith("[I;"), owner
            give('naturalist:ant_spawn_egg[minecraft:entity_data={id:"naturalist:ant",Tags:[eggtest],attributes:[{id:"minecraft:max_health",base:17d}]}]')
            click_block(8, 99, 8)
            result = cmd("attribute @e[tag=eggtest,limit=1] minecraft:max_health base get")
            assert any(line.endswith("17.0") for line in result), result
            print(json.dumps({"egg_max_health": result}, indent=2))
        finally:
            if socket.exists():
                ctl("quit")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("jar", type=Path)
    args = parser.parse_args()
    capture(args.version, args.jar)
