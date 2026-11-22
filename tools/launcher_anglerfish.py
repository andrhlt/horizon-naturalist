"""Replay Anglerfish combat, cooldown, and bucket behavior on the pinned build."""
import json
import hashlib
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import time
from prepare import JAR_SHA

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
resources = Path(os.environ.get("NATURALIST_RESOURCES", version / ".local/horizon-client/26.3"))
if not resources.is_dir():
    raise SystemExit(f"Prepared Horizon resources directory is missing: {resources}; set NATURALIST_RESOURCES")
jar = Path(os.environ.get("NATURALIST_JAR", ""))
if not jar.is_file():
    raise SystemExit("Pinned Naturalist jar is missing; set NATURALIST_JAR to its verified 2.0.7 jar")
if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
    raise SystemExit("NATURALIST_JAR does not match the pinned Naturalist 2.0.7 SHA-512")
with tempfile.TemporaryDirectory(prefix="nat-anglerfish-", ignore_cleanup_errors=True) as work:
    home = Path(work)
    socket = home / "fish.sock"
    jar_cache = home / "jars"
    jar_cache.mkdir()
    shutil.copy2(jar, jar_cache / f"{JAR_SHA}.jar")

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                capture_output=True, text=True, timeout=75)
        if result.returncode:
            raise RuntimeError(f"horizon-ctl {args!r} failed: {result.stdout}\n{result.stderr}")
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def data_or(target, key, default=None):
        rows = cmd(f"data get entity {target} {key}")
        value = next((row.split("following entity data: ", 1)[1]
                      for row in rows if "following entity data: " in row), None)
        if value is None:
            if default is not None and rows == [f"Found no elements matching {key}"]:
                return default
            raise RuntimeError(f"Could not read {key} from {target}; Horizon replied: {rows!r}")
        return value

    def data(target, key):
        return data_or(target, key)

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
            resources, "--home", home, "--game-dir", home,
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
            match = next((m for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row))), None)
            if match is None:
                raise RuntimeError(f"Could not parse Horizon attribute response for {attribute}: {rows!r}")
            output[attribute] = float(match[1])
        output["default_variant"] = data_or(fish, "Variant", '"naturalist:red"').strip('"')

        assert any("damage" in line.lower()
                   for line in cmd(f"damage {fish} 1 minecraft:mob_attack by {attacker}"))
        output["retaliation_damage_applied"] = True
        step(40)
        output["attacker_health_after_retaliation"] = float(data(attacker, "Health").removesuffix("f"))
        combat_cooldown = int(data_or(fish, "HuntingCooldown", "0"))
        output["cooldown_started_by_combat"] = combat_cooldown == 2400
        if combat_cooldown == 0:
            cmd(f"data merge entity {fish} {{HuntingCooldown:2400}}")
        cooldown = int(data_or(fish, "HuntingCooldown", "0"))
        output["cooldown_loaded"] = cooldown == 2400
        output["cooldown"] = cooldown
        step(1)
        output["next_cooldown"] = int(data_or(fish, "HuntingCooldown", "0"))
        output["cooldown_ticks_down"] = output["next_cooldown"] == max(0, cooldown - 1)
        cmd("kill @e[type=minecraft:cod]")
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
        output["after_player_release_inventory"] = data("@s", "Inventory")
        try:
            output["player_released_variant"] = data(
                "@e[type=naturalist:anglerfish,limit=1]", "Variant"
            ).strip('"')
        except RuntimeError as error:
            output["player_release_error"] = str(error)
        print("player release", json.dumps(output), flush=True)
        print("dispenser", cmd('setblock 8 93 7 minecraft:dispenser[facing=south]'), flush=True)
        print("insert", cmd('item replace block 8 93 7 container.0 with '
            'naturalist:anglerfish_bucket[minecraft:bucket_entity_data={Variant:"naturalist:glow"}]'), flush=True)
        print("stored", cmd('data get block 8 93 7 Items'), flush=True)
        print("redstone", cmd('setblock 8 94 7 minecraft:redstone_block'), flush=True)
        step(10)
        print("after", cmd('data get block 8 93 7 Items'), flush=True)
        live = "@e[type=naturalist:anglerfish,limit=1]"
        output["dispenser_released_variant"] = data(live, "Variant").strip('"')
        print(json.dumps(output, indent=2), flush=True)
        expected = {key: reference[key] for key in output if key in reference}
        if expected:
            assert {key: output[key] for key in expected} == expected, (output, expected)
    finally:
        if socket.exists():
            ctl("quit")
        if child is not None:
            try:
                child.wait(timeout=8)
            except subprocess.TimeoutExpired:
                child.terminate()
                child.wait(timeout=8)
