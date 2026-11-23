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
from headless_fixture import HeadlessCommands

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

    commands = None

    def cmd(line):
        return commands(line)

    def data_or(target, key, default=None):
        rows = cmd(f"data get entity {target} {key}")
        value = next((row.split("following entity data: ", 1)[1]
                      for row in rows if "following entity data: " in row), None)
        if value is None:
            if default is not None and (
                rows == [f"Found no elements matching {key}"]
                or any("No entity was found" in row for row in rows)
            ):
                return default
            raise RuntimeError(f"Could not read {key} from {target}; Horizon replied: {rows!r}")
        return value

    def data(target, key):
        return data_or(target, key)

    def inspect():
        rows = cmd("naturalist_fixture_mobs")
        marker = "Mob fixture "
        payload = next((row.split(marker, 1)[1] for row in rows if marker in row), None)
        if payload is None:
            raise RuntimeError(f"Anglerfish inspect fixture returned no payload: {rows!r}")
        return json.loads(payload)

    def step(ticks):
        cmd(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    child = None
    try:
        launcher = os.environ.get("NATURALIST_STOCK_LAUNCHER")
        if launcher:
            log = Path(sys.argv[2]).parent.parent / "anglerfish/play.log"
            with log.open("w") as handle:
                command = [launcher, "play", "--version", version.name]
                if os.environ.get("NATURALIST_ALLOW_JAR_DOWNLOADS") == "1":
                    command.append("--allow-jar-downloads")
                instance = os.environ.get("NATURALIST_INSTANCE")
                if instance:
                    command.extend(("--instance", instance))
                account = os.environ.get("NATURALIST_ACCOUNT")
                if account:
                    command.extend(("--account", account))
                command.extend(("--world", f"Anglerfish Combat {home.name}", "--arg=--control-socket",
                    f"--arg={socket}", "--arg=--headless", "--arg=--test-terrain", "--arg=--render-distance", "--arg=4"))
                child = subprocess.Popen(command,
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
                "--world", "Anglerfish Combat", "--test-terrain", "--render-distance", 4, "--wait")
        commands = HeadlessCommands(ctl)
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("difficulty normal")
        force_load = cmd("forceload add 0 0 47 47")
        assert any("Marked 9 chunks" in row for row in force_load), force_load
        ctl("tp", 8.5, 74, 7.0)
        ctl("wait", 2)
        print("initial player position", ctl("state").get("player", {}).get("position"), flush=True)
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("fill 4 91 3 14 91 14 minecraft:stone")
        cmd("fill 4 92 3 14 96 14 minecraft:air")
        cmd("fill 5 75 8 13 78 13 minecraft:water")
        cmd("fill 2 22 2 27 33 14 minecraft:stone")
        cmd("fill 5 25 5 22 31 11 minecraft:water")
        cmd("fill 14 23 3 14 32 13 minecraft:stone")
        cmd("setblock 21 27 8 minecraft:sea_lantern")
        cmd("fill 5 39 5 12 41 12 minecraft:water")
        cmd("fill 5 53 5 12 55 12 minecraft:water")
        step(20)
        cmd('summon naturalist:anglerfish 8.5 76 8.5 {NoGravity:1b,PersistenceRequired:1b,Tags:[fish]}')
        cmd('summon minecraft:cod 10.25 76 8.5 {NoAI:1b,NoGravity:1b,Health:2f,PersistenceRequired:1b,Tags:[attacker]}')
        fish = "@e[type=naturalist:anglerfish,tag=fish,limit=1]"
        attacker = "@e[tag=attacker,limit=1]"
        output = {}
        for attribute in ("max_health", "movement_speed", "attack_damage"):
            rows = cmd(f"attribute {fish} minecraft:{attribute} base get")
            match = next((m for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row))), None)
            if match is None:
                raise RuntimeError(f"Could not parse Horizon attribute response for {attribute}: {rows!r}")
            output[attribute] = float(match[1])
        output["default_variant"] = next((mob["variant"] for mob in inspect()["mobs"] if mob["health"] == 8.0), None)
        output["default_variant"] = (output["default_variant"] or '"naturalist:red"').strip('"')

        combat_event = cmd("naturalist_fixture_mobs attack anglerfish cod")
        assert any("landed=true" in line for line in combat_event), combat_event
        output["combat_damage_applied"] = True
        immediate_combat_state = inspect()["mobs"]
        fish_state = next((mob for mob in immediate_combat_state
                           if mob["kind"] == "naturalist:anglerfish"), None)
        combat_cooldown = 0 if fish_state is None else fish_state.get("hunting_cooldown") or 0
        output["cooldown_started_by_combat"] = combat_cooldown == 2400
        assert output["cooldown_started_by_combat"], immediate_combat_state
        step(2)
        cod_state = next((mob for mob in inspect()["mobs"] if mob["kind"] == "minecraft:cod"), None)
        output["attacker_health_after_combat"] = 0.0 if cod_state is None else cod_state["health"]
        cooldown = next((mob.get("hunting_cooldown") or 0 for mob in inspect()["mobs"]
                        if mob.get("alive") and mob["kind"] == "naturalist:anglerfish"), 0)
        output["cooldown_loaded"] = cooldown == 2399
        output["cooldown"] = cooldown
        assert output["cooldown_loaded"], output
        step(1)
        output["next_cooldown"] = next((mob.get("hunting_cooldown") or 0 for mob in inspect()["mobs"]
                                        if mob.get("alive") and mob["kind"] == "naturalist:anglerfish"), 0)
        output["cooldown_ticks_down"] = output["next_cooldown"] == 2398
        assert output["cooldown_ticks_down"], output
        cmd("kill @e[type=minecraft:cod]")
        print(json.dumps(output), flush=True)

        step(21)
        spawn_result = cmd('summon naturalist:anglerfish 8.5 76 8.5 {NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Health:6f,Variant:"naturalist:glow"}')
        assert any("Summoned" in row for row in spawn_result), spawn_result
        requested = next((mob for mob in inspect()["mobs"] if mob["health"] == 6.0), None)
        assert requested is not None, inspect()
        variant_before_capture = requested.get("variant")
        output["variant_before_capture"] = variant_before_capture
        assert variant_before_capture == "naturalist:glow", (requested, variant_before_capture)
        target_entity = requested
        ctl("tp", 8.5, 74, 6.5)
        ctl("wait", 2)
        survival_mode = cmd("gamemode survival @s")
        assert ctl("state").get("gamemode") == "survival", survival_mode
        cmd("item replace entity @s weapon.mainhand with minecraft:water_bucket")
        step(4)
        ctl("look-at-entity", target_entity["id"])
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        held = ctl("state").get("held") or {}
        assert held.get("item") == "naturalist:anglerfish_bucket", (held, ctl("state"))
        bucket_state = inspect()["held"]
        output["bucket_item"] = bucket_state["item"]
        output["bucket_variant"] = bucket_state.get("bucket_variant")
        output["custom_variant"] = bucket_state.get("custom_variant")
        assert "naturalist:anglerfish_bucket" in output["bucket_item"]
        assert output["bucket_variant"] == "naturalist:glow", bucket_state
        assert output["custom_variant"] == "naturalist:glow", bucket_state
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        after_release = inspect()
        output["runtime_creative_flag"] = after_release.get("creative")
        output["after_player_release_held"] = None if after_release["held"] is None else after_release["held"]["item"]
        released = next((mob for mob in after_release["mobs"] if mob["alive"]
                         and mob["kind"] == "naturalist:anglerfish"
                         and 0 <= mob["position"][0] < 20
                         and mob["health"] == 6.0), None)
        output["player_released_variant"] = None if released is None else released["variant"]
        assert released is not None and output["player_released_variant"] == "naturalist:glow", after_release
        output["player_bucket_conserved"] = output["after_player_release_held"] == "minecraft:bucket"
        print("player release", json.dumps(output), flush=True)
        cmd("kill @e[type=naturalist:anglerfish]")
        cmd("setblock 8 76 8 minecraft:air")
        print("dispenser", cmd('setblock 8 76 7 minecraft:dispenser[facing=south]'), flush=True)
        print("insert", cmd('item replace block 8 76 7 container.0 with '
            'naturalist:anglerfish_bucket[minecraft:bucket_entity_data={Variant:"naturalist:glow"}]'), flush=True)
        print("stored", cmd('data get block 8 76 7 Items'), flush=True)
        print("redstone", cmd('setblock 8 77 7 minecraft:redstone_block'), flush=True)
        step(10)
        dispenser_contents = cmd('data get block 8 76 7 Items')
        after_dispenser = inspect()
        dispensed = next((mob for mob in after_dispenser["mobs"] if mob["alive"]
                          and mob["kind"] == "naturalist:anglerfish"), None)
        output["dispenser_released_variant"] = None if dispensed is None else dispensed["variant"]
        output["dispenser_bucket_conserved"] = any('id: "minecraft:bucket"' in row for row in dispenser_contents)
        output["dispenser_item_entities"] = ctl("entities", 64, "minecraft:item").get("entities", [])
        output["dispenser_release_pass"] = output["dispenser_released_variant"] == "naturalist:glow"
        print("dispenser result", json.dumps({key: output[key] for key in (
            "dispenser_released_variant", "dispenser_bucket_conserved", "dispenser_item_entities",
            "dispenser_release_pass")}), flush=True)
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
