"""Replay Starfish spawning, capture, loot, and placement on headless Horizon."""
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
with tempfile.TemporaryDirectory(prefix="nat-starfish-", ignore_cleanup_errors=True) as work:
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
            log = Path(sys.argv[2]).parent.parent / "starfish/play.log"
            with log.open("w") as handle:
                child = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Starfish Multiface {home.name}", "--creative", "--arg=--control-socket",
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
                "--world", "Starfish Multiface", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("tp @s 8.5 92 6.5")
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("fill 4 91 3 15 91 15 minecraft:stone")
        cmd("fill 4 92 3 15 98 15 minecraft:air")
        cmd("fill 5 92 8 14 95 15 minecraft:water")
        cmd("fill 5 29 5 12 29 12 minecraft:stone")
        cmd("fill 5 30 5 12 31 12 minecraft:water")
        cmd("setblock 8 62 8 minecraft:stone")
        cmd("setblock 8 63 8 minecraft:water")
        cmd('summon naturalist:starfish 8.5 92.65 8.5 {NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[fish]}')
        fish = "@e[type=naturalist:starfish,tag=fish,limit=1]"
        output = {}
        for attribute, key in (("max_health", "max_health"), ("movement_speed", "speed")):
            rows = cmd(f"attribute {fish} minecraft:{attribute} base get")
            output[key] = float(next(m[1] for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row))))
        output["variant"] = data(fish, "Variant").strip('"')
        for query in ("mob", "spawn"):
            marker = "Starfish fixture "
            output.update(next(json.loads(row.split(marker, 1)[1])
                for row in cmd("naturalist_fixture_starfish " + query) if marker in row))

        cmd(f'data merge entity {fish} {{Variant:"naturalist:purple"}}')
        cmd("gamemode survival @s")
        cmd("tp @s 8.5 92 6.5")
        cmd("item replace entity @s weapon.mainhand with minecraft:water_bucket")
        step(4)
        nearby = ctl("entities", 5, "naturalist:starfish")["entities"]
        assert len(nearby) == 1, nearby
        ctl("look-at-entity", nearby[0]["id"])
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        inventory = data("@s", "Inventory")
        assert 'id: "naturalist:starfish_bucket"' in inventory, (inventory, ctl("state"))
        for component, key in (("bucket_entity_data", "bucket_variant"), ("custom_data", "custom_variant")):
            match = re.search(rf'"minecraft:{component}": \{{[^{{}}]*Variant: "([^"]+)"', inventory)
            assert match, inventory
            output[key] = match[1]

        ctl("look-at-block", 8, 91, 8)
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        live = "@e[type=naturalist:starfish,nbt={FromBucket:1b},limit=1]"
        output["released_variant"] = data(live, "Variant").strip('"')
        assert 'id: "minecraft:bucket"' in data("@s", "Inventory")

        def drop_at(color):
            return next((e["item"]["item"] for e in ctl("entities", 10, "minecraft:item")["entities"]
                if e["item"]["item"] == "naturalist:" + color + "_starfish"), "")

        cmd(f"kill {live}")
        ctl("wait", 6)
        output["death_color"] = drop_at("purple")
        cmd('summon naturalist:starfish 13.5 92 8.5 {Variant:"other:fantasy",Tags:[unknown]}')
        cmd("kill @e[tag=unknown,limit=1]")
        ctl("wait", 6)
        output["fallback_death_color"] = drop_at("orange")

        cmd('setblock 8 93 7 minecraft:dispenser[facing=south]')
        cmd('item replace block 8 93 7 container.0 with '
            'naturalist:starfish_bucket[minecraft:bucket_entity_data={Variant:-1}]')
        cmd('setblock 7 93 7 minecraft:redstone_block')
        step(8)
        output["legacy_variant"] = data('@e[type=naturalist:starfish,nbt={Variant:"naturalist:red"},limit=1]', "Variant").strip('"')
        print(json.dumps(output), flush=True)

        cmd("fill 6 92 4 8 94 7 minecraft:air")
        cmd("tp @s 7.5 92 6.5")
        cmd("item replace entity @s weapon.mainhand with naturalist:blue_starfish")
        ctl("look-at-block", 7, 91, 8)
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        block = ctl("block", 7, 92, 8)
        print("Placed", block, flush=True)
        serial = block.get("state", "")
        output["placed_block"] = serial.split("[", 1)[0]
        output["waterlogged"] = "waterlogged=true" in serial
        output["attached_down"] = "down=true" in serial
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
