"""Compare Giant Isopod cover, damage, knockback, and buckets with pinned Java."""
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import time
from headless_fixture import HeadlessCommands

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-isopod-", ignore_cleanup_errors=True) as work:
    home = Path(work)
    socket = home / "fish.sock"

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                capture_output=True, check=True, text=True, timeout=75)
        return json.loads(result.stdout.splitlines()[-1])

    command_channel = [None]

    def cmd(line):
        if command_channel[0] is None:
            command_channel[0] = HeadlessCommands(ctl)
        return command_channel[0](line)

    def data(target, key):
        if target.startswith("@e["):
            return command_channel[0].entity_data(target, key)
        return next(row.split("following entity data: ", 1)[1]
                    for row in cmd(f"data get entity {target} {key}") if "following entity data: " in row)

    def step(ticks):
        cmd(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    child = None
    try:
        launcher = os.environ.get("NATURALIST_STOCK_LAUNCHER")
        if launcher:
            log = Path(sys.argv[2]).parent.parent / "giant-isopod/play.log"
            with log.open("w") as handle:
                child = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Giant Isopod {home.name}", "--creative", "--arg=--control-socket",
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
                "--world", "Giant Isopod", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("forceload add 0 0 47 47")
        cmd("tp @s 8.5 92 6.5")
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("fill 4 91 3 18 91 16 minecraft:stone")
        cmd("fill 4 92 3 18 98 16 minecraft:air")
        cmd("fill 5 92 8 14 95 15 minecraft:water")
        cmd("fill 5 29 5 12 29 12 minecraft:stone")
        cmd("fill 5 30 5 12 31 12 minecraft:water")
        cmd("setblock 8 62 8 minecraft:stone")
        cmd("setblock 8 63 8 minecraft:water")
        cmd('summon naturalist:giant_isopod 8.5 92 8.5 {NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[adult]}')
        cmd('summon naturalist:giant_isopod 9.5 92 8.5 {Age:-24000,NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[baby]}')
        adult = "@e[type=naturalist:giant_isopod,tag=adult,limit=1]"
        output = {}

        def probe(query):
            marker = "Isopod fixture "
            rows = cmd("naturalist_fixture_giant_isopod " + query)
            output.update(next(json.loads(row.split(marker, 1)[1]) for row in rows if marker in row))

        for attribute, key in (("max_health", "max_health"), ("movement_speed", "speed")):
            rows = cmd(f"attribute {adult} minecraft:{attribute} base get")
            value = next((m[1] for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row))), None)
            assert value is not None, (attribute, rows)
            output[key] = float(value)
        output["variant"] = data(adult, "Variant").strip('"')
        for query in ("food", "spawn"):
            probe(query)
        assert output.pop("supported_water") is True, output
        assert output.pop("open_water") is False, output
        assert output.pop("sea_surface") is False, output
        cmd("tp @s 8.5 92 6.0")
        step(5)
        output["breathes_water"] = int(data(adult, "Air").rstrip("sS")) == 300
        probe("creative")
        cmd("gamemode survival @s")
        cmd("effect give @s minecraft:invisibility 20 0 true")
        step(2)
        probe("invisible")
        cmd("effect clear @s minecraft:invisibility")
        cmd("fill 8 92 7 8 93 7 minecraft:stone")
        step(2)
        probe("wall")
        cmd("fill 8 92 7 8 93 7 minecraft:air")
        cmd("tp @s 8.5 92 6.5")
        step(2)
        for query in ("near", "baby", "knock_near"):
            probe(query)
        cmd(f"damage {adult} 4 minecraft:generic")
        output["near_health"] = float(data(adult, "Health").rstrip("fF"))
        cmd("tp @s 16.5 92 6.5")
        step(9)
        probe("after_9")
        step(1)
        probe("after_10")
        probe("outside")
        probe("knock_outside")
        step(12)
        cmd(f"data merge entity {adult} {{Health:12.0f}}")
        cmd(f"damage {adult} 4 minecraft:generic")
        output["outside_health"] = float(data(adult, "Health").rstrip("fF"))

        cmd("fill 4 92 3 18 96 16 minecraft:air")
        cmd("kill @e[tag=baby,limit=1]")
        step(22)
        cmd(f"tp {adult} 8.5 92.65 8.5")
        cmd(f'data merge entity {adult} {{Variant:"naturalist:blue",Motion:[0d,0d,0d]}}')
        cmd("tp @s 8.5 92 7.3")
        cmd("item replace entity @s weapon.mainhand with minecraft:bucket")
        step(3)
        nearby = ctl("entities", 5, "naturalist:giant_isopod")["entities"]
        assert len(nearby) == 1, nearby
        look = ctl("look-at-entity", nearby[0]["id"])
        ctl("wait", 2)
        assert ctl("looking-at")["kind"] == "entity", look
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        assert "milk_bucket" not in data("@s", "Inventory")
        cmd("item replace entity @s weapon.mainhand with minecraft:water_bucket")
        look = ctl("look-at-entity", nearby[0]["id"])
        ctl("wait", 2)
        aimed = ctl("looking-at")
        assert aimed["kind"] == "entity", (look, aimed)
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        inventory = data("@s", "Inventory")
        for component, key in (("bucket_entity_data", "bucket_variant"), ("custom_data", "custom_variant")):
            match = re.search(rf'"minecraft:{component}": \{{[^{{}}]*Variant: "([^"]+)"', inventory)
            assert match, (inventory, aimed, nearby)
            output[key] = match[1]
        cmd("tp @s 8.5 92 7.3")
        cmd("setblock 8 92 8 minecraft:water")
        ctl("wait", 2)
        look = ctl("look-at-block", 8, 91, 8)
        assert ctl("looking-at")["kind"] == "block", look
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        live = "@e[type=naturalist:giant_isopod,nbt={FromBucket:1b},limit=1]"
        assert 'id: "minecraft:bucket"' in data("@s", "Inventory")
        output["released_variant"] = data(live, "Variant").strip('"')

        cmd('setblock 8 93 7 minecraft:dispenser[facing=south]')
        cmd('item replace block 8 93 7 container.0 with '
            'naturalist:giant_isopod_bucket[minecraft:bucket_entity_data={Variant:-2}]')
        cmd('setblock 7 93 7 minecraft:redstone_block')
        step(8)
        output["legacy_variant"] = data('@e[type=naturalist:giant_isopod,nbt={Variant:"naturalist:brown",FromBucket:1b},limit=1]', "Variant").strip('"')
        print(json.dumps(output), flush=True)
        diff = {key: (expected, output.get(key)) for key, expected in reference.items() if output.get(key) != expected}
        assert not diff, diff
        print(f"Matched all {len(reference)} Giant Isopod Java behavior fields")
    finally:
        if socket.exists():
            ctl("quit")
        if child is not None:
            try:
                child.wait(timeout=8)
            except subprocess.TimeoutExpired:
                child.terminate()
                child.wait(timeout=8)
