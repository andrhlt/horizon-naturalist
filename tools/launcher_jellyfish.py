"""Replay the pinned Jellyfish contact damage, pulse, and bucket variants."""
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
with tempfile.TemporaryDirectory(prefix="nat-jellyfish-", ignore_cleanup_errors=True) as work:
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
            log = Path(sys.argv[2]).parent.parent / "jellyfish/play.log"
            with log.open("w") as handle:
                child = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Jellyfish Contact {home.name}", "--creative", "--arg=--control-socket",
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
                "--world", "Jellyfish Contact", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("difficulty normal")
        cmd("tp @s 8.5 92 6.5")
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("fill 4 91 3 14 91 14 minecraft:stone")
        cmd("fill 4 92 3 14 96 14 minecraft:air")
        cmd("fill 5 92 8 13 95 13 minecraft:water")
        cmd('summon naturalist:jellyfish 6.5 92 8.5 {NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[jellyfish_fixture]}')
        cmd('summon naturalist:jellyfish 8.7 92 8.5 {NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[peer]}')
        cmd('summon minecraft:cod 8.7 92 8.7 {NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[touching]}')
        cmd('summon minecraft:cod 10.5 92 8.5 {NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[outside]}')
        fish = "@e[type=naturalist:jellyfish,tag=jellyfish_fixture,limit=1]"
        output = {}
        for attribute in ("max_health", "movement_speed"):
            rows = cmd(f"attribute {fish} minecraft:{attribute} base get")
            output[attribute] = float(next(m[1] for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row))))
        output["default_variant"] = data(fish, "Variant").strip('"')

        def fixture(command):
            return next(row.split("Jellyfish fixture ", 1)[1] for row in
                        cmd("naturalist_fixture_jellyfish " + command) if "Jellyfish fixture " in row)

        def pulse():
            assert fixture("reset") == "ready"
            step(1)
            out = fixture("pulse")
            print(out, flush=True)
            return out.startswith("true ")

        def health(tag):
            return float(data(f"@e[tag={tag},limit=1]", "Health").rstrip("f"))

        output["first_pulse"] = pulse()
        output["touching_cod"] = health("touching")
        output["outside_cod"] = health("outside")
        output["nearby_jellyfish"] = health("peer")
        early_pulses = [pulse() for _ in range(19)]
        output["no_early_pulse"] = not any(early_pulses)
        output["second_pulse"] = pulse()
        print(json.dumps(output), flush=True)

        cmd("kill @e[type=naturalist:jellyfish]")
        cmd("kill @e[type=minecraft:cod]")
        step(21)
        cmd("fill 7 92 4 9 95 7 minecraft:air")
        cmd('summon naturalist:jellyfish 8.5 92.5 8.5 {NoAI:1b,NoGravity:1b,Variant:"naturalist:green",Tags:[jellyfish_fixture]}')
        cmd("tp @s 8.5 92 6.5")
        cmd("gamemode survival @s")
        cmd("item replace entity @s weapon.mainhand with minecraft:water_bucket")
        step(4)
        nearby = ctl("entities", 5, "naturalist:jellyfish")
        assert len(nearby["entities"]) == 1, nearby
        target_id = nearby["entities"][0]["id"]
        ctl("look-at-entity", target_id)
        sight = ctl("looking-at")
        assert (sight.get("entity") or {}).get("id") == target_id, (nearby, sight)
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        inventory = data("@s", "Inventory")
        assert 'id: "naturalist:jellyfish_bucket"' in inventory, (inventory, ctl("state"))
        for component, key in (("bucket_entity_data", "bucket_variant"), ("custom_data", "custom_variant")):
            match = re.search(rf'"minecraft:{component}": \{{[^{{}}]*Variant: "([^"]+)"', inventory)
            assert match, inventory
            output[key] = match[1]
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        live = "@e[type=naturalist:jellyfish,nbt={FromBucket:1b},limit=1]"
        output["released_variant"] = data(live, "Variant").strip('"')
        assert 'id: "minecraft:bucket"' in data("@s", "Inventory")
        cmd(f"kill {live}")
        cmd('setblock 8 93 7 minecraft:dispenser[facing=south]')
        cmd('item replace block 8 93 7 container.0 with '
            'naturalist:jellyfish_bucket[minecraft:bucket_entity_data={Variant:-2}]')
        cmd('setblock 8 94 7 minecraft:redstone_block')
        step(10)
        output["legacy_variant"] = data('@e[type=naturalist:jellyfish,nbt={Variant:"naturalist:blue"},limit=1]', "Variant").strip('"')
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
