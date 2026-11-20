"""Replay the pinned Jellyfish retaliation, poison, and bucket variants."""
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import time

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-ray-", ignore_cleanup_errors=True) as work:
    home = Path(work)
    socket = home / "fish.sock"

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                capture_output=True, check=True, text=True, timeout=75)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def data(target, key):
        return next(row.split("following entity data: ", 1)[1]
                    for row in cmd(f"data get entity {target} {key}") if "following entity data: " in row)

    def step(ticks):
        cmd(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    child = None
    try:
        launcher = os.environ.get("NATURALIST_STOCK_LAUNCHER")
        if launcher:
            log = Path(sys.argv[2]).parent.parent / "midwater-buckets/play.log"
            with log.open("w") as handle:
                child = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Midwater Buckets {home.name}", "--creative", "--arg=--control-socket",
                    f"--arg={socket}", "--arg=--headless", "--arg=--test-terrain", "--arg=--render-distance", "--arg=4"],
                    stdout=handle, stderr=subprocess.STDOUT)
            for _ in range(50):
                if child.poll() is not None:
                    raise RuntimeError(f"Horizon Play exited with code {child.returncode}")
                if socket.exists() and ctl("state")["screen"] == "playing":
                    break
                time.sleep(1)
            else:
                raise RuntimeError("Timed out waiting for Horizon Play to load the fixture world")
        else:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Midwater Buckets", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamemode survival @s")
        cmd("tp @s 8.5 92 6.5")
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("fill 4 91 3 14 91 15 minecraft:stone")
        cmd("fill 4 92 3 14 98 15 minecraft:air")
        cmd("fill 5 92 8 14 95 15 minecraft:water")
        cmd("tp @s 8.5 92 6.5")
        ctl("look", 0, 0)
        output = {}
        for bucket, species in reference.items():
            cmd("fill 4 92 3 14 98 7 minecraft:air")
            cmd("tp @s 8.5 92 6.5")
            ctl("look", 0, 0)
            cmd(f"item replace entity @s weapon.mainhand with {bucket}")
            assert ctl("state")["looking_at"]["kind"] == "none", ctl("state")
            ctl("mouse", "right", "click")
            ctl("wait", 4)
            fish = ctl("entities", 6, species).get("entities", [])
            inventory = data("@s", "Inventory")
            print(bucket, ctl("state")["player"]["position"], fish, inventory, flush=True)
            if fish and 'id: "minecraft:bucket"' in inventory:
                assert data(f"@e[type={species},limit=1]", "FromBucket") == "1b"
                output[bucket] = species
                cmd(f"kill @e[type={species}]")
                step(21)
        print(json.dumps(output, indent=2), flush=True)
        assert output == reference, (output, reference)
    finally:
        if socket.exists():
            ctl("quit")
        if child is not None:
            try:
                child.wait(timeout=8)
            except subprocess.TimeoutExpired:
                child.terminate()
                child.wait(timeout=8)
