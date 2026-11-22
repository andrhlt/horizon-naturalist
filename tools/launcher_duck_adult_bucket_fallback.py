"""Verify an adult Duck click falls through and the next bucket click still works."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

from prepare import JAR_SHA


def run(version: Path, jar: Path):
    if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
        raise ValueError("expected the pinned Naturalist 2.0.7 NeoForge 26.3 jar")
    with tempfile.TemporaryDirectory(prefix="nat-duck-adult-bucket-", ignore_cleanup_errors=True) as temp:
        root = Path(temp)

        def ctl(side, *args):
            home = root / side
            result = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                cwd=root,
                env={**os.environ, "HORIZON_HOME": str(home), "HORIZON_NO_AUDIO": "1"},
                capture_output=True,
                text=True,
                timeout=85,
            )
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(line):
            result = ctl("host", "command", line)
            assert result["ok"], (line, result)
            return result["chat"]

        def uuid(target):
            return next((row.split("following entity data: ", 1)[1]
                         for row in cmd(f"data get entity {target} UUID")
                         if "following entity data: " in row), None)

        def launch(side, *args):
            home = root / side
            (home / "jars").mkdir(parents=True, exist_ok=True)
            os.link(jar, home / "jars" / f"{JAR_SHA}.jar")
            (home / "horizon-mods.txt").write_text("mod.naturalist:true\n")
            ctl(side, "launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--render-distance", 4, *args)

        try:
            launch("host", "--world", "Adult Duck bucket fallback", "--test-terrain", "--creative", "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("kill @e[type=naturalist:duck]")
            cmd("tp @s 7.5 100 6.5")
            ctl("host", "wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 108 15 minecraft:air")
            port = ctl("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            ctl("guest", "connect", f"127.0.0.1:{port}", "DuckAdultGuest")
            ctl("guest", "wait-for", "loaded", "--timeout", 55)
            cmd("tp DuckAdultGuest 7.5 100 6.5")
            cmd("tp @s 12.5 100 6.5")
            cmd("gamemode survival DuckAdultGuest")
            ctl("host", "wait", 25)
            cmd("item replace entity DuckAdultGuest weapon.mainhand with minecraft:bucket")
            cmd("summon naturalist:duck 7.5 100 8.5 {Tags:[adult_duck],NoAI:1b,NoGravity:1b,Age:0}")
            target = "@e[tag=adult_duck,limit=1]"
            before = uuid(target)
            assert before and uuid(target)
            ducks = ctl("guest", "entities", 8, "naturalist:duck")["entities"]
            assert len(ducks) == 1 and not ducks[0]["baby"], ducks
            ctl("guest", "look-at", "entity", ducks[0]["id"])
            ctl("guest", "mouse", "right", "click")
            ctl("host", "wait", 5)
            after_first = ctl("guest", "inventory")
            assert uuid(target) == before, (before, uuid(target))
            assert after_first["counts"].get("minecraft:bucket") == 1, after_first
            assert after_first["counts"].get("minecraft:water_bucket", 0) == 0, after_first
            cmd("tp @e[tag=adult_duck,limit=1] 10 100 10")
            cmd("setblock 7 101 8 minecraft:water")
            cmd("setblock 7 101 6 minecraft:stone")
            cmd("tp DuckAdultGuest 7.5 102 6.5")
            ctl("guest", "wait", 2)
            ctl("guest", "look-at", 7.5, 101.5, 8.5)
            ctl("guest", "mouse", "right", "click")
            ctl("host", "wait", 6)
            after_block = ctl("guest", "inventory")
            assert after_block["counts"].get("minecraft:water_bucket") == 1, after_block
            assert after_block["counts"].get("minecraft:bucket", 0) == 0, after_block
            print(json.dumps({
                "adult_entity_click_did_not_pick_up_water": True,
                "adult_duck_remained": True,
                "next_intentional_block_use_worked": True,
            }), flush=True)
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    ctl(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("jar", type=Path)
    args = parser.parse_args()
    run(args.version, args.jar)
