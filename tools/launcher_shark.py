"""Compare Pinned Java Great White Shark traits and actual gameplay headlessly."""
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import time

version = Path(sys.argv[1])
java = json.loads(Path(sys.argv[2]).read_text())

with tempfile.TemporaryDirectory(prefix="nat-shark-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "mobs.sock"

    def ctl(*args):
        proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str,args)],
                              check=True, capture_output=True, text=True, timeout=75)
        return json.loads(proc.stdout.splitlines()[-1])

    def cmd(command):
        return ctl("command", command)["chat"]

    def data(target, name):
        return next(row.split("following entity data: ", 1)[1] for row in
            cmd(f"data get entity {target} {name}") if "following entity data: " in row)

    def step(n):
        cmd(f"tick step {n}")
        ctl("wait", n + 2)

    def inspect(query="values"):
        marker = "Shark fixture "
        return next(json.loads(row.split(marker, 1)[1]) for row in
                    cmd(f"naturalist_fixture_shark {query}") if marker in row)

    def block(x, y, z, name):
        return any("Test passed" in row or "test passed" in row or "succeeded" in row
                   for row in cmd(f"execute if block {x} {y} {z} {name}"))

    child = None
    try:
        launcher = os.environ.get("NATURALIST_STOCK_LAUNCHER")
        if launcher:
            with Path(__file__).with_name("play.log").open("w") as log:
                child = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Great White Shark {home.name}", "--creative", "--arg=--control-socket",
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
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Great White Shark", "--creative", "--test-terrain", "--render-distance", 4, "--wait")

        kind = "naturalist:great_white_shark"
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("difficulty normal")
        cmd("tp @s 8.5 92 3.5")
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("fill 3 91 1 16 91 16 minecraft:stone")
        cmd("fill 3 92 1 16 97 16 minecraft:air")
        cmd("fill 3 92 7 16 97 7 minecraft:glass")
        cmd("fill 5 92 8 14 96 15 minecraft:water")
        cmd("tp @s 8.5 92 3.5")
        out = {}

        def sample():
            [data] = inspect()["mobs"]
            return data

        def kill_sharks():
            cmd(f"kill @e[type={kind}]")
            step(21)

        def summon(pos, fields):
            assert any("Summoned" in row for row in cmd(f"summon {kind} {pos} {fields}"))

        target = f"@e[type={kind},limit=1]"
        summon("8.5 92 5.5", "{NoAI:1b,NoGravity:1b,Rotation:[0.0f,0.0f]}")
        step(3)
        base = sample()
        out.update(pushable=base["pushable"], adult_dims=base["dims"],
                   part_width_height=base["part_sizes"], part_offsets_z=base["part_offsets_z"],
                   breeding_food=[bool(base["food"])] * 8,
                   variant=data(target, "Variant").strip('"'))
        for attribute, key in (
            ("max_health", "health"), ("movement_speed", "speed"), ("attack_damage", "attack"),
            ("follow_range", "range"), ("knockback_resistance", "knockback_resistance"),
            ("step_height", "step"),
        ):
            messages = cmd(f"attribute {target} minecraft:{attribute} base get")
            value = next((m[1] for row in messages if (m := re.search(r" is ([0-9.eE+-]+)", row))), None)
            assert value is not None, (attribute, messages)
            out[key] = round(float(value), 4)

        cmd("gamemode survival @s")
        foods = ("cod", "cooked_cod", "salmon", "cooked_salmon", "tropical_fish", "pufferfish", "beef", "tropical_fish_bucket")
        out["healing_amounts"] = []
        for food in foods:
            cmd(f"data merge entity {target} {{Health:60.0f}}")
            cmd(f"item replace entity @s weapon.mainhand with minecraft:{food} 2")
            ctl("wait", 3)
            look = ctl("look-at-entity", base["id"])
            assert look["target"].get("entity") and look["target"]["entity"]["id"] == base["id"], (look, ctl("state")["player"])
            ctl("mouse", "right", "click")
            ctl("wait", 4)
            observed = sample()
            held_count = ctl("state")["held"]["count"]
            out["healing_amounts"].append([round(observed["health"] - 60), held_count])
        cmd("gamemode creative @s")
        cmd(f"data merge entity {target} {{Age:-24000}}")
        out["baby_dims"] = sample()["dims"]

        kill_sharks()
        summon("8.5 92 5.5", "{NoAI:1b,NoGravity:1b,Air:0s}")
        step(19)
        dry = sample()
        out["dry_air_19"], out["dry_health_19"] = dry["air"], dry["health"]
        step(1)
        dry = sample()
        out["dry_air_20"], out["dry_health_20"] = dry["air"], dry["health"]
        cmd(f"tp {target} 8.5 93.5 10.5")
        step(2)
        out["wet_air"] = sample()["air"]
        obs = inspect("spawn")
        out["spawn_two_water"], out["spawn_dry"] = obs["wet"], obs["dry"]
        if "cluster" in obs:
            out["cluster"] = obs["cluster"]
        cmd("setblock 8 93 10 minecraft:air")
        out["spawn_one_water"] = inspect("spawn")["wet"]
        cmd("setblock 8 93 10 minecraft:water")

        kill_sharks()
        cmd("fill 2 92 3 16 97 18 minecraft:water")
        cmd("fill 2 92 2 16 97 2 minecraft:glass")
        cmd("tp @s 3.5 96 4.5")
        summon("8.5 94.5 9.5", "{InLove:600,NoGravity:1b}")
        summon("10.5 94.5 9.5", "{InLove:600,NoGravity:1b}")
        step(80)
        out["no_offspring"] = len(inspect()["mobs"]) == 2

        for sated in (True, False):
            kill_sharks()
            cmd("kill @e[type=minecraft:cod,tag=SharkPrey]")
            cmd("summon minecraft:cod 9.5 94.4 13.5 {NoAI:1b,NoGravity:1b,Health:1.0f,Tags:[SharkPrey]}")
            cooldown = 150 if sated else 0
            summon("9.5 94.4 6.5", f"{{Rotation:[0.0f,0.0f],HuntingCooldown:{cooldown}}}")
            if sated:
                step(28)
                out["sated_no_target"] = sample()["target"] == "None"
                continue
            saw_target = False
            for _ in range(180):
                step(1)
                row = sample()
                saw_target |= row["target"] != "None"
                if row["target"] != "None":
                    assert abs(row["speed"] - 1.4) < 1e-6, row
                if row["cooldown"] >= 2399:
                    break
            out["fish_target"] = saw_target
            out["prey_dead"] = not any(p["health"] > 0 for p in ctl("entities", 30, "minecraft:cod")["entities"])
            out["kill_cooldown"] = row["cooldown"]
            step(1)
            out["cooldown_tick"] = sample()["cooldown"]

        kill_sharks()
        cmd("fill 3 92 1 16 97 16 minecraft:air")
        summon("8.5 92 6.5", "{NoAI:1b,NoGravity:1b,Rotation:[0.0f,0.0f]}")
        cmd("tp @s 11.5 92 8.5")
        step(3)
        ctl("look-at", 8.5, 92.8, 8.5)
        dot = ctl("looking-at")
        print("Shark head ray", json.dumps(dot), flush=True)
        result = dot.get("target", dot)
        assert result.get("entity") and result["entity"]["id"] == sample()["id"], dot
        health = sample()["health"]
        ctl("mouse", "left", "click")
        step(1)
        assert sample()["health"] < health, sample()
        print("Shark head melee damaged the authoritative parent.", flush=True)
        cmd("kill @e[type=minecraft:item]")
        cmd(f"kill {target}")
        step(22)
        tooth = '@e[type=minecraft:item,nbt={Item:{id:"naturalist:tooth"}},limit=1]'
        drops = data(tooth, "Item")
        count = next((int(m[1]) for m in re.finditer(r'count: (\d+)', drops)), None)
        assert count is not None and 2 <= count <= 5, drops
        print(f"An actual Shark death dropped {count} Naturalist Teeth (Java range 2–5).", flush=True)
        print(json.dumps(out), flush=True)
        mismatch = {key: [expected, out.get(key)] for key, expected in java.items()
                    if key in out and out[key] != expected}
        assert not mismatch, mismatch
        print(f"Great White Shark matched {len(out)} measured Java readings.", flush=True)
    finally:
        if socket.exists():
            ctl("quit")
        if child is not None:
            try:
                child.wait(timeout=8)
            except subprocess.TimeoutExpired:
                child.terminate()
                child.wait(timeout=8)
