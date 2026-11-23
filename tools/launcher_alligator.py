"""Compare Java Alligator attributes, egg lifecycle and live targeting headlessly."""
import json
import shutil
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import time

version = Path(sys.argv[1])
java = json.loads(Path(sys.argv[2]).read_text())
live_java = json.loads(Path(sys.argv[3]).read_text()) if len(sys.argv) > 3 else None

with tempfile.TemporaryDirectory(prefix="nat-gator-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "mobs.sock"
    shared_jars = Path.home() / "Library/Application Support/horizon/jars"
    if shared_jars.exists():
        shutil.copytree(shared_jars, home / "jars", dirs_exist_ok=True)
    env = {**os.environ, "HORIZON_HOME": str(home)}

    def ctl(*args):
        proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str,args)],
                              check=True, capture_output=True, text=True, timeout=75, env=env)
        return json.loads(proc.stdout.splitlines()[-1])

    def cmd(command):
        return ctl("command", command)["chat"]

    def data(target, name):
        return next(row.split("following entity data: ", 1)[1] for row in
            cmd(f"data get entity {target} {name}") if "following entity data: " in row)

    def step(n):
        cmd(f"tick step {n}")
        ctl("wait", n + 2)

    def inspect(query):
        marker = "Alligator fixture "
        return next(json.loads(row.split(marker, 1)[1]) for row in
                    cmd(f"naturalist_fixture_alligator {query}") if marker in row)

    def block(x, y, z, name):
        return any("Test passed" in row or "test passed" in row or "succeeded" in row
                   for row in cmd(f"execute if block {x} {y} {z} {name}"))

    child = None
    try:
        launcher = os.environ.get("NATURALIST_STOCK_LAUNCHER")
        if launcher:
            with Path(__file__).with_name("play.log").open("w") as log:
                child = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Alligator {home.name}", "--creative", "--arg=--control-socket",
                    f"--arg={socket}", "--arg=--headless", "--arg=--test-terrain",
                    "--arg=--render-distance", "--arg=4"], stdout=log, stderr=subprocess.STDOUT)
            for _ in range(60):
                if child.poll() is not None:
                    raise RuntimeError(f"Horizon Play exited with {child.returncode}")
                if socket.exists() and ctl("state")["screen"] == "playing":
                    break
                time.sleep(1)
            else:
                raise RuntimeError("Horizon did not open a headless world")
        else:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                Path.home() / "Library/Application Support/horizon-launcher/versions/nightly-2026-11-20-0a875da/.local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Alligator", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:random_tick_speed 0")
        cmd("gamerule minecraft:mob_griefing true")
        cmd("time set 21600")
        cmd("tp @s 5.5 92 4.5")
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("fill 4 91 2 16 91 13 minecraft:stone")
        cmd("fill 4 92 2 16 97 13 minecraft:air")
        cmd('summon naturalist:alligator 6.5 92 5.5 {NoAI:1b,NoGravity:1b,HuntingCooldown:10,Tags:[adult]}')
        cmd('summon naturalist:alligator 8.5 92 5.5 {NoAI:1b,NoGravity:1b,Age:-24000,Tags:[baby]}')
        adult = "@e[type=naturalist:alligator,tag=adult,limit=1]"
        a, b = inspect("adult"), inspect("baby")
        actual = {key: a[key] for key in ("food", "head_yaw", "cluster")}
        actual.update(adult_dims=a["dims"], baby_dims=b["dims"],
                      knockback=[a["knockback"], b["knockback"]],
                      variant=data(adult, "Variant").strip('"'))
        for name, key in (("max_health", "health"), ("movement_speed", "speed"), ("attack_damage", "attack"),
                          ("knockback_resistance", "knockback_resistance"), ("step_height", "step")):
            rows = cmd(f"attribute {adult} minecraft:{name} base get")
            value = next((m[1] for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row))), None)
            assert value is not None, (name, rows)
            actual[key] = float(value)
        step(1)
        actual["tick_cooldown"] = inspect("adult")["hunting"]
        cmd('summon minecraft:frog 6.5 92 6.5 {NoAI:1b,Tags:[frog]}')
        actual["kill_cooldown"] = inspect("kill")["hunting"]
        cmd("kill @e[tag=frog]")

        cmd("setblock 11 92 5 naturalist:alligator_egg[eggs=3]")
        phases = [inspect("hatch") for _ in range(3)]
        actual["hatch_stages_stone"] = [phase["stage"] for phase in phases]
        for key in ("hatch_baby_ages", "hatch_offsets"):
            actual[key] = phases[-1][key]

        cmd("kill @e[type=naturalist:alligator]")
        step(25)
        cmd("gamerule minecraft:random_tick_speed 0")
        cmd("time set 6000")
        cmd("setblock 6 91 5 minecraft:mud")
        cmd('summon naturalist:alligator 6.5 92 5.5 {NoGravity:1b,HasEgg:1b,Tags:[adult]}')
        laying = None
        for _ in range(42):
            step(4)
            current = inspect("adult")
            if current["laying"]:
                laying = current
                break
        assert laying, "Alligator did not find the sole mud nest"
        assert 1 <= laying["counter"] <= 5, laying
        step(61 - laying["counter"])
        edge = inspect("adult")
        assert edge["has_egg"], edge
        step(1)
        after = inspect("adult")
        assert not after["laying"] and not after["has_egg"], after
        actual["lay_goal_calls"] = 61
        actual["lay_block"] = block(6, 92, 5, "naturalist:alligator_egg")
        actual["lay_fresh"] = block(6, 92, 5, "naturalist:alligator_egg[hatch=0]")
        actual["lay_eggs_in_1_4"] = any(block(6, 92, 5, f"naturalist:alligator_egg[eggs={count}]") for count in range(1,5))
        actual["lay_post_love_after_ai"] = after["in_love"]
        actual["lay_post_flags"] = [after["has_egg"], after["laying"]]
        cmd("kill @e[type=naturalist:alligator]")
        step(24)
        cmd("setblock 6 92 5 minecraft:air")
        cmd("setblock 6 91 5 minecraft:stone")
        cmd("tp @s 6.5 92 3.5")
        cmd("gamemode survival @s")
        cmd("item replace entity @s weapon.mainhand with minecraft:beef 5")
        cmd('summon naturalist:alligator 6.5 92 5.5 {NoAI:1b,NoGravity:1b,Tags:[court]}')
        step(2)

        def feed_closest(x=6.5):
            entities = ctl("entities", 5, "naturalist:alligator")["entities"]
            assert entities
            picked = min(entities, key=lambda entity: abs(entity["pos"][0] - x))
            aimed = ctl("look-at-entity", picked["id"])
            assert aimed["target"]["kind"] == "entity", aimed
            ctl("mouse", "right", "click")
            ctl("wait", 3)

        court = "@e[tag=court,limit=1]"
        feed_closest()
        actual["can_love"] = int(data(court, "InLove")) > 0
        cmd(f"data merge entity {court} {{InLove:0,HasEgg:1b}}")
        feed_closest()
        actual["egg_stops_love"] = int(data(court, "InLove")) == 0 and data(court, "HasEgg") == "1b"

        cmd(f"data merge entity {court} {{NoAI:0b,HasEgg:0b}}")
        cmd('summon naturalist:alligator 8.5 92 5.5 {NoGravity:1b,Tags:[mate]}')
        step(2)
        feed_closest(6.5)
        feed_closest(8.5)
        cmd("gamemode creative @s")
        cmd('summon naturalist:alligator 12 92 5.5 {NoAI:1b,NoGravity:1b,Age:-24000,Tags:[control_baby]}')
        mate = "@e[tag=mate,limit=1]"
        assert all(int(data(parent, "InLove")) > 0 for parent in (court, mate))
        actual["bred"] = False
        for _ in range(14):
            step(6)
            has_egg = [data(parent, "HasEgg") == "1b" for parent in (court, mate)]
            if sum(has_egg) == 1:
                actual["bred"] = all(int(data(parent, "InLove")) == 0 for parent in (court, mate))
                break
        population = ctl("entities", 16, "naturalist:alligator")["entities"]
        assert len(population) >= 2, population
        actual["child_count_after_breed"] = len(population) - 2
        print(json.dumps(actual), flush=True)
        mismatch = {key: [java[key], value] for key, value in actual.items() if java[key] != value}
        assert not mismatch, mismatch
        print(f"Alligator matched {len(actual)} Java readings through the production hooks.", flush=True)

        if os.environ.get("NATURALIST_CHECK_SAVED_LOVE"):
            target = "@e[tag=saved_love,limit=1]"
            cmd('summon naturalist:alligator 10.5 92 9.5 {NoAI:1b,NoGravity:1b,InLove:300,Tags:[saved_love]}')
            assert data(target, "InLove") == "300"
            cmd(f"data merge entity {target} {{InLove:250}}")
            assert data(target, "InLove") == "250"
            print("Alligator retains InLove on summon and live NBT reload.", flush=True)

        if live_java:
            present = {"prey": "minecraft:frog", "sated": "minecraft:frog",
                       "nest": "minecraft:cow", "nest_three_away": "minecraft:cow",
                       "baby": "minecraft:frog"}
            cmd("gamerule minecraft:natural_regeneration false")
            cmd("fill 3 115 2 13 115 13 minecraft:stone")
            cmd("fill 3 116 2 13 121 13 minecraft:air")
            cmd("fill 7 114 7 12 114 12 minecraft:stone")
            cmd("fill 7 115 7 12 115 12 minecraft:water")
            result = {}
            for phase, expected in live_java.items():
                cmd("kill @e[type=naturalist:alligator]")
                cmd("kill @e[tag=live_cand]")
                cmd("gamemode creative @s")
                cmd("tp @s 5.5 116 3.5")
                cmd("setblock 9 116 5 minecraft:air")
                step(22)
                cmd("time set " + ("18000" if phase == "dark_player" else "6000"))
                fields = ["NoGravity:1b", "PersistenceRequired:1b", "Tags:[adult]"]
                if phase == "sated":
                    fields.append("HuntingCooldown:2400")
                if phase == "pregnant_player":
                    fields.append("HasEgg:1b")
                if phase == "baby":
                    fields.append("Age:-24000")
                at = "7.2 116 6.2" if phase == "water_player" else "6.5 116 5.5"
                cmd("summon naturalist:alligator " + at + " {" + ",".join(fields) + "}")
                candidate = present.get(phase, "player")
                if candidate == "player":
                    cmd("gamemode survival @s")
                    cmd("effect give @s minecraft:instant_health 1 10 true")
                    position = "7.5 115.2 7.15" if phase == "water_player" else "6.5 116 7.15"
                    cmd(f"tp @s {position}")
                    actor = "@s"
                else:
                    x = 6.5 if phase == "nest_three_away" else 7.5
                    cmd("summon " + candidate + f" {x} 116 6.6 " +
                        "{NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[live_cand]}")
                    actor = "@e[tag=live_cand,limit=1]"
                    if phase.startswith("nest"):
                        cmd("setblock 9 116 5 naturalist:alligator_egg")
                start = float(data(actor, "Health").rstrip("fF"))
                compared = {"targeted": False, "injured": False}
                for _ in range(12):
                    step(8)
                    observed = inspect("baby" if phase == "baby" else "adult")
                    compared["targeted"] |= observed["target"] == candidate
                    compared["injured"] |= float(data(actor, "Health").rstrip("fF")) < start
                    if compared["targeted"] and (compared["injured"] or not expected["injured"]):
                        break
                result[phase] = compared
            print("Live targeting", json.dumps(result), flush=True)
            target_diff = {phase: [expected, result[phase]] for phase, expected in live_java.items()
                if result[phase] != {key: expected[key] for key in ("targeted", "injured")}}
            assert not target_diff, target_diff
            print(f"Alligator matched {len(result)} live Java target cases.", flush=True)
    finally:
        if socket.exists():
            ctl("quit")
        if child is not None:
            try:
                child.wait(timeout=8)
            except subprocess.TimeoutExpired:
                child.terminate()
                child.wait(timeout=8)
