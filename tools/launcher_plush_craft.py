"""Craft Java's original Plush Bear pattern and break the placed block headlessly."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile
import time


def verify(version):
    with tempfile.TemporaryDirectory(prefix="nat-plush-craft-", ignore_cleanup_errors=True) as directory:
        home = Path(directory)
        socket = home / "craft.sock"
        script = home / "clicks.json"
        env = {**os.environ, "HORIZON_TICK_SCRIPT": str(script)}

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, check=True, timeout=75, env=env)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def craft(material, should_match):
            cmd("clear @s")
            for slot in (1, 3, 4, 5, 7):
                cmd(f"item replace entity @s hotbar.{slot} with {material if slot == 4 else 'minecraft:string'}")
            ctl("interact", 8, 102, 8, "north")
            clicks = [click for slot in (1, 3, 4, 5, 7) for click in (slot, 100 + slot)]
            clicks.extend((200, 0))
            steps = {tick: [f"click-slot {click}"] for tick, click in
                     zip(range(3, 3 + 3 * len(clicks), 3), clicks, strict=True)}
            close = max(steps) + 3
            steps[close] = ["close"]
            done = script.with_suffix(".done")
            done.unlink(missing_ok=True)
            script.write_text(json.dumps({"name": material, "ticks": close + 3, "steps": steps}))
            script.with_suffix(".go").touch()
            deadline = time.monotonic() + 12
            while time.monotonic() < deadline and not done.exists():
                time.sleep(0.05)
            assert done.exists() and done.read_text() == material, material
            rows = cmd("data get entity @s Inventory")
            crafted = any('id: "naturalist:plush_bear"' in row for row in rows)
            assert crafted == should_match, (material, rows)
            print(material, crafted, flush=True)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Naturalist Plush Craft", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 102 6.5")
            ctl("wait", 20)
            cmd("fill 2 101 2 15 101 15 minecraft:stone")
            cmd("fill 2 102 2 15 109 15 minecraft:air")
            cmd("setblock 8 102 8 minecraft:crafting_table")
            cmd("tp @s 8.5 102 6.5")
            cmd("gamemode survival @s")
            craft("minecraft:white_carpet", False)
            craft("minecraft:white_wool", True)
            craft("minecraft:blue_wool", True)
            cmd("setblock 8 102 8 minecraft:air")
            ctl("select", "naturalist:plush_bear")
            ctl("look-at", 8.5, 101.99, 8.5)
            ctl("mouse", "right", "click")
            ctl("wait", 3)
            assert ctl("block", 8, 102, 8)["block"] == "naturalist:plush_bear"
            assert not any('id: "naturalist:plush_bear"' in row for row in cmd("data get entity @s Inventory"))
            ctl("look-at", 8.5, 102.55, 8.5)
            ctl("mouse", "left", "down")
            ctl("wait", 40)
            ctl("mouse", "left", "up")
            rows = cmd('execute if entity @e[type=minecraft:item,nbt={Item:{id:"naturalist:plush_bear",count:1}}] run say plush-dropped')
            assert any("plush-dropped" in line for line in rows), (rows, ctl("block", 8, 102, 8))
            print("Crafted Plush Bear was placed, broken bare-handed and dropped one of itself", flush=True)
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    verify(parser.parse_args().version)
