"""Exercise a Naturalist bass bucket through real headless player inputs."""
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
with tempfile.TemporaryDirectory(prefix="nat-bass-bucket-", ignore_cleanup_errors=True) as work:
    home = Path(work)
    socket = home / "bucket.sock"

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                check=True, capture_output=True, text=True, timeout=45)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        result = ctl("command", line)
        assert result["ok"], (line, result)
        return result["chat"]

    def data(target, key):
        rows = cmd(f"data get entity {target} {key}")
        return next(row.split("following entity data: ", 1)[1]
                    for row in rows if "following entity data: " in row)

    def click(yaw, pitch):
        ctl("look", yaw, pitch)
        ctl("mouse", "right", "click")
        ctl("wait", 4)

    process = None
    try:
        launcher = os.environ.get("NATURALIST_STOCK_LAUNCHER")
        if launcher:
            with (version.parent.parent / "../play.log").open("w") as log:
                process = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Bass Bucket Fixture {home.name}", "--creative", "--arg=--control-socket",
                    f"--arg={socket}", "--arg=--headless", "--arg=--test-terrain", "--arg=--render-distance", "--arg=4"],
                    stdout=log, stderr=subprocess.STDOUT)
            for _ in range(50):
                if process.poll() is not None:
                    raise RuntimeError(f"Horizon Play exited with code {process.returncode}")
                if socket.exists() and ctl("state")["screen"] == "playing":
                    break
                time.sleep(1)
            else:
                raise RuntimeError("Timed out waiting for Horizon Play to load the fixture world")
        else:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Bass Bucket Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("tp @s 7.5 101 6.5")
        ctl("wait", 20)
        cmd("fill 2 100 2 15 100 15 minecraft:stone")
        cmd("fill 2 101 2 15 107 15 minecraft:air")
        cmd("tick freeze")
        cmd("fill 4 101 8 12 104 14 minecraft:water")
        cmd("tp @s 7.5 101 6.5")
        cmd("gamemode survival @s")
        cmd("item replace entity @s weapon.mainhand with minecraft:water_bucket")
        cmd('summon naturalist:bass 7.5 102.25 8.5 {Variant:"naturalist:bass_large",'
            'Health:2f,CustomName:"Big Bass",NoAI:1b,PersistenceRequired:1b}')
        click(0, 6)
        inventory = data("@s", "Inventory")
        assert 'id: "naturalist:bass_bucket"' in inventory, (inventory, ctl("state"))
        assert not ctl("entities", 20, "naturalist:bass")["entities"], ctl("state")
        output = {}
        for component, key in (("bucket_entity_data", "bucket_variant"), ("custom_data", "custom_variant")):
            match = re.search(rf'"minecraft:{component}": \{{[^{{}}]*Variant: "([^"]+)"', inventory)
            assert match, inventory
            output[key] = match[1]
        assert '"minecraft:custom_name": "Big Bass"' in inventory, inventory
        output["bucket_name"] = "Big Bass"
        click(0, 6)
        fish = "@e[type=naturalist:bass,limit=1]"
        output["released_variant"] = data(fish, "Variant").strip('"')
        output["released_health"] = float(data(fish, "Health").rstrip("f"))
        assert 'id: "minecraft:bucket"' in data("@s", "Inventory")
        assert data(fish, "FromBucket") in {"1b", "true", "1"}
        cmd(f"kill {fish}")
        cmd('setblock 8 102 7 minecraft:dispenser[facing=south]')
        cmd('item replace block 8 102 7 container.0 with '
            'naturalist:bass_bucket[minecraft:bucket_entity_data={Variant:-2}]')
        cmd('setblock 8 103 7 minecraft:redstone_block')
        cmd("tick step 10")
        ctl("wait", 12)
        output["legacy_variant"] = data(fish, "Variant").strip('"')
        print(json.dumps(output, indent=2), flush=True)
        assert output == reference, (output, reference)
    finally:
        if socket.exists():
            ctl("quit")
        if process is not None:
            try:
                process.wait(timeout=8)
            except subprocess.TimeoutExpired:
                process.terminate()
                process.wait(timeout=8)
