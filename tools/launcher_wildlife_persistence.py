"""Verify six wildlife UUIDs across disk save and same-world headless reopen."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
resources = Path.home() / "Library/Application Support/horizon-launcher/versions/nightly-2026-11-20-0a875da/.local/horizon-client/26.3"
shared_jars = Path.home() / "Library/Application Support/horizon/jars"
species = ("alligator", "capybara", "catfish", "clam", "hippo", "piranha")

with tempfile.TemporaryDirectory(prefix="nat-wildlife-persist-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    shutil.copytree(shared_jars, home / "jars")
    socket = home / "persistence.sock"
    env = {**os.environ, "HORIZON_HOME": str(home)}

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                capture_output=True, check=True, text=True, timeout=100, env=env)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def launch():
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources", resources,
            "--home", home, "--game-dir", home, "--world", "APP2129 persistence", "--creative",
            "--test-terrain", "--render-distance", 4, "--wait")

    def uuid_for(name):
        rows = cmd(f"execute in minecraft:overworld run data get entity @e[type=naturalist:{name},tag=app2129_{name},limit=1] UUID")
        return next(row.split("following entity data: ", 1)[1] for row in rows if "following entity data: " in row)

    def value_for(name, key):
        rows = cmd(f"execute in minecraft:overworld run data get entity @e[type=naturalist:{name},tag=app2129_{name},limit=1] {key}")
        return next((row.split("following entity data: ", 1)[1] for row in rows
                     if "following entity data: " in row), None)

    def state_for(name):
        specific = {
            "alligator": ["Age"],
            "capybara": ["Age", "Owner", "Tame", "DyeColor", "FollowingOwner", "Sitting"],
            "catfish": ["HuntingCooldown"],
            "clam": ["NaturalistClamOpen", "NaturalistClamTimer", "NaturalistClamCooldown"],
            "hippo": ["Age"],
            "piranha": [],
        }
        return {key: value_for(name, key) for key in ["UUID", "Health", *specific[name]]}

    try:
        launch()
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:advance_time false")
        cmd("forceload add 0 0 31 0")
        cmd("tick freeze")
        cmd("fill 2 101 2 30 101 14 minecraft:stone")
        cmd("fill 2 102 2 30 108 14 minecraft:air")
        cmd("fill 20 102 6 24 105 10 minecraft:water")
        owner = next(row.split("following entity data: ", 1)[1] for row in cmd("data get entity @s UUID") if "following entity data: " in row)
        positions = ((8.5, 102, 5.5), (10.5, 102, 5.5), (12.5, 102, 5.5),
                     (14.5, 102, 5.5), (16.5, 102, 5.5), (24.5, 102, 5.5))
        health = {"alligator": 20, "capybara": 6, "catfish": 4, "clam": 20, "hippo": 50, "piranha": 4}
        for name, (x, y, z) in zip(species, positions):
            state = f"Health:{health[name]}f,NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[app2129_{name}]"
            if name in ("alligator", "capybara", "hippo"):
                state += ",Age:-6000"
            if name == "capybara":
                state += f",Tame:1b,Owner:{owner},DyeColor:14,FollowingOwner:1b,Sitting:0b"
            if name == "catfish":
                state += ",HuntingCooldown:17"
            if name == "clam":
                state += ",NaturalistClamOpen:1b,NaturalistClamTimer:127,NaturalistClamCooldown:27"
            cmd(f"summon naturalist:{name} {x} {y} {z} {{{state}}}")
        cmd("save-all flush")
        before = {name: state_for(name) for name in species}
        ctl("quit")
        socket = home / "reopen.sock"
        launch()
        cmd("tick freeze")
        cmd("forceload add 0 0 31 0")
        after = {name: state_for(name) for name in species}
        uuid_matches = {name: after[name]["UUID"] == before[name]["UUID"] for name in species}
        elapsed_fields = {"Age", "HuntingCooldown", "NaturalistClamTimer", "NaturalistClamCooldown"}
        def persisted(name, key, initial, current):
            if current is None:
                return False
            if key not in elapsed_fields:
                return initial == current
            try:
                initial_value, current_value = int(initial.rstrip("fF")), int(current.rstrip("fF"))
            except (AttributeError, ValueError):
                return False
            if key == "Age" and initial_value < 0:
                return initial_value <= current_value <= 0
            return 0 <= current_value <= initial_value
        state_matches = {name: {key: persisted(name, key, value, after[name].get(key))
                                for key, value in before[name].items()} for name in species}
        cmd("tp @s 10.5 102 4.5")
        cmd("item replace entity @s weapon.mainhand with minecraft:air")
        capybara = ctl("entities", 5, "naturalist:capybara")["entities"][0]
        ctl("look-at-entity", capybara["id"])
        ctl("mouse", "right", "click")
        ctl("wait", 3)
        capybara_interacted_after_reopen = value_for("capybara", "Sitting") == "1b"
        result = {
            "artifact": version.name,
            "horizon": "0a875da91dbb0fdcb7ba8c42c11341aa229af51a",
            "world": "APP2129 persistence",
            "dimension": "minecraft:overworld",
            "disk_record_parse_attempted": False,
            "disk_record_note": "Skipped because same-world runtime observations compare stable UUIDs and meaningful entity state before and after reopen.",
            "force_loaded_chunks": [[0, 0], [1, 0]],
            "pre_reopen_state": before,
            "post_reopen_state": after,
            "uuid_matches": uuid_matches,
            "all_six_post_reopen_uuid_matches": all(uuid_matches.values()),
            "state_matches": state_matches,
            "all_saved_runtime_fields_match": all(all(fields.values()) for fields in state_matches.values()),
            "capybara_owner_interaction_after_reopen": capybara_interacted_after_reopen,
            "owner_player_uuid": owner,
            "query": "execute in minecraft:overworld run data get entity @e[type=naturalist:<species>,tag=app2129_<species>,limit=1] UUID",
        }
        print(json.dumps(result, indent=2))
        if len(sys.argv) > 3:
            Path(sys.argv[3]).write_text(json.dumps(result, indent=2) + "\n")
    finally:
        try:
            ctl("quit")
        except Exception:
            pass
