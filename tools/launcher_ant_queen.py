"""Verify queen placement and owner links in a headless world."""
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
            cmd("setblock 8 101 8 minecraft:stone")
            give("naturalist:queen_ant")
            click_block(8, 99, 8)
            blocked = data("entity @s SelectedItem.id") == '"naturalist:queen_ant"'
            cmd("setblock 8 101 8 minecraft:air")
            click_block(8, 99, 8)
            consumed = data("entity @s SelectedItem.id") is None
            positions = [(x, y, z) for x in range(7, 10) for y in range(100, 103)
                         for z in range(7, 10) if block(x, y, z)]
            entrances = [pos for pos in positions if block(*pos, "[open=true]")]
            queen = [pos for pos in positions if block(*pos, "[has_queen=true,open=true,workers=0]")]
            links = [data("block " + " ".join(map(str, position)) + " Owner") for position in positions]
            linked = links.count(owner)
            if linked != len(positions):
                print(json.dumps({"owner": owner, "links": links, "block": cmd("data get block 8 100 8")}), flush=True)
            main = {"blocked": blocked, "consumed": consumed, "blocks": len(positions),
                    "open_in_range": 4 <= len(entrances) <= 6, "queens": len(queen), "owner_links": linked}
            print(json.dumps({"queen": main}, indent=2), flush=True)

            assert main == {"blocked": True, "consumed": True, "blocks": 10,
                            "open_in_range": True, "queens": 1, "owner_links": 10}, main
            cmd("fill 7 100 7 9 102 9 minecraft:air")
            cmd("gamemode creative @s")
            give("naturalist:queen_ant")
            click_block(8, 99, 8)
            creative_kept = data("entity @s SelectedItem.id") == '"naturalist:queen_ant"'
            assert creative_kept and block(8, 100, 8), "creative placement or item retention failed"

            cmd("fill 7 100 7 9 102 9 minecraft:air")
            cmd("gamemode survival @s")
            cmd("item replace entity @s weapon.mainhand with minecraft:air")
            cmd("item replace entity @s weapon.offhand with naturalist:queen_ant")
            click_block(8, 99, 8)
            offhand_consumed = data("entity @s Inventory[{Slot:-106b}].id") is None
            assert offhand_consumed and block(8, 100, 8), "offhand placement or consumption failed"
            print(json.dumps({"creative_kept": creative_kept, "offhand_consumed": offhand_consumed}))
            return main

        finally:
            if socket.exists():
                ctl("quit")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("jar", type=Path)
    args = parser.parse_args()
    capture(args.version, args.jar)
