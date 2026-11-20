"""Replay the pinned Jellyfish placement, extended melee, hunt cooldown, and buckets."""
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
with tempfile.TemporaryDirectory(prefix="nat-anglerfish-", ignore_cleanup_errors=True) as work:
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
            log = Path(sys.argv[2]).parent.parent / "anglerfish/play.log"
            with log.open("w") as handle:
                child = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Anglerfish Combat {home.name}", "--creative", "--arg=--control-socket",
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
                "--world", "Anglerfish Combat", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("difficulty normal")
        cmd("tp @s 8.5 92 6.5")
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("fill 4 91 3 14 91 14 minecraft:stone")
        cmd("fill 4 92 3 14 96 14 minecraft:air")
        cmd("fill 5 92 8 13 95 13 minecraft:water")
        cmd("fill 2 22 2 27 33 14 minecraft:stone")
        cmd("fill 5 25 5 22 31 11 minecraft:water")
        cmd("fill 14 23 3 14 32 13 minecraft:stone")
        cmd("setblock 21 27 8 minecraft:sea_lantern")
        cmd("fill 5 39 5 12 41 12 minecraft:water")
        cmd("fill 5 53 5 12 55 12 minecraft:water")
        step(20)
        cmd('summon naturalist:anglerfish 8.5 93 8.5 {NoGravity:1b,PersistenceRequired:1b,Tags:[fish]}')
        cmd('summon minecraft:cod 10.25 93 8.5 {NoAI:1b,NoGravity:1b,Health:2f,PersistenceRequired:1b,Tags:[attacker]}')
        fish = "@e[type=naturalist:anglerfish,tag=fish,limit=1]"
        attacker = "@e[tag=attacker,limit=1]"
        output = {}
        for attribute in ("max_health", "movement_speed", "attack_damage"):
            rows = cmd(f"attribute {fish} minecraft:{attribute} base get")
            output[attribute] = float(next(m[1] for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row))))
        output["default_variant"] = data(fish, "Variant").strip('"')

        def fixture(tag):
            return next(row.split("Anglerfish fixture ", 1)[1] for row in
                        cmd("naturalist_fixture_anglerfish " + tag) if "Anglerfish fixture " in row)

        assert any("damage" in line.lower()
                   for line in cmd(f"damage {fish} 1 minecraft:mob_attack by {attacker}"))
        output["retaliated"] = fixture("fish") == "attacker"
        for counter in range(6):
            step(1)
            cooldown = int(data(fish, "HuntingCooldown"))
            if cooldown > 0:
                break
        output["extended_reach"] = cooldown > 0 and counter < 2
        output["cooldown"] = cooldown
        step(1)
        output["next_cooldown"] = int(data(fish, "HuntingCooldown"))

        placement = [entry == "true" for entry in fixture("placement").split()]
        output.update(zip(("dark_spawn", "bright_spawn", "middle_spawn", "surface_spawn"), placement))
        output["deep_variant"] = fixture("deep")
        print("Aquatic spawn tables", fixture("tables"), flush=True)
        print(json.dumps(output), flush=True)

        cmd("kill @e[type=naturalist:anglerfish]")
        step(21)
        cmd('summon naturalist:anglerfish 8.5 92.5 8.5 {NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[fish]}')
        cmd(f'data merge entity {fish} {{Variant:"naturalist:glow"}}')
        cmd(f"tp {fish} 8.5 92.5 8.5")
        cmd("tp @s 8.5 92 6.5")
        cmd("gamemode survival @s")
        cmd("item replace entity @s weapon.mainhand with minecraft:water_bucket")
        step(4)
        nearby = ctl("entities", 5, "naturalist:anglerfish")
        assert nearby.get("entities"), nearby
        ctl("look-at-entity", nearby["entities"][0]["id"])
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        inventory = data("@s", "Inventory")
        assert 'id: "naturalist:anglerfish_bucket"' in inventory, (inventory, ctl("state"))
        for component, key in (("bucket_entity_data", "bucket_variant"), ("custom_data", "custom_variant")):
            match = re.search(rf'"minecraft:{component}": \{{[^{{}}]*Variant: "([^"]+)"', inventory)
            assert match, inventory
            output[key] = match[1]
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        live = "@e[type=naturalist:anglerfish,nbt={FromBucket:1b},limit=1]"
        output["released_variant"] = data(live, "Variant").strip('"')
        assert 'id: "minecraft:bucket"' in data("@s", "Inventory")
        cmd(f"kill {live}")
        cmd('setblock 8 93 7 minecraft:dispenser[facing=south]')
        cmd('item replace block 8 93 7 container.0 with '
            'naturalist:anglerfish_bucket[minecraft:bucket_entity_data={Variant:-1}]')
        cmd('setblock 8 94 7 minecraft:redstone_block')
        step(10)
        output["legacy_variant"] = data('@e[type=naturalist:anglerfish,nbt={Variant:"naturalist:glow"},limit=1]', "Variant").strip('"')
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
