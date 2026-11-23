"""Replay Java's Whale health, feeding, and physical multipart gameplay headlessly."""
import json
import math
import os
from pathlib import Path
import re
import subprocess
import shutil
import sys
import tempfile
import time
from headless_fixture import HeadlessCommands


def replay(version, java):
    with tempfile.TemporaryDirectory(prefix="nat-whale-", ignore_cleanup_errors=True) as scratch:
        home = Path(scratch)
        socket = home / "whale.sock"
        jar = Path.home() / "Library/Application Support/horizon/jars/b1a220dc6bcb6101911226a57d75d62f2797461c3e529e8cf7110673b69632538c4a129b135f9572411cbc60336c096e056488fbe1528638196c247eef6492cd.jar"
        if jar.is_file():
            (home / "jars").mkdir()
            shutil.copyfile(jar, home / "jars" / jar.name)
            (home / "horizon-mods.txt").write_text("mod.naturalist:true\n")

        def ctl(*args):
            result = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                check=True, capture_output=True, text=True, timeout=75,
            )
            return json.loads(result.stdout.splitlines()[-1])

        command_channel = [None]

        def cmd(command):
            if command_channel[0] is None:
                command_channel[0] = HeadlessCommands(ctl)
            return command_channel[0](command)

        def data(target, name):
            if target.startswith("@e["):
                return command_channel[0].entity_data(target, name)
            marker = "following entity data: "
            return next(row.split(marker, 1)[1] for row in cmd(f"data get entity {target} {name}") if marker in row)

        def attribute(target, key, scope="base get"):
            rows = cmd(f"attribute {target} minecraft:{key} {scope}")
            value = next((m[1] for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row))), None)
            assert value is not None, (key, rows)
            return round(float(value), 4)

        def step(n):
            cmd(f"tick step {n}")
            ctl("wait", n + 2)

        def assure(command, marker):
            rows = cmd(command)
            assert any(marker in row for row in rows), (command, rows)

        def inspect(query):
            marker = "Whale fixture "
            rows = cmd(f"naturalist_fixture_whale {query}")
            value = next((json.loads(row.split(marker, 1)[1]) for row in rows if marker in row), None)
            assert value is not None, (query, rows)
            return value

        launcher = os.environ.get("NATURALIST_STOCK_LAUNCHER")
        child = None
        try:
            if launcher:
                output = Path(os.environ.get("NATURALIST_PLAY_LOG", home / "whale-play.log"))
                with output.open("w") as log:
                    child = subprocess.Popen([launcher, "play", "--version", version.name,
                        "--world", f"Whale {home.name}", "--creative", "--arg=--control-socket", f"--arg={socket}",
                        "--arg=--headless", "--arg=--test-terrain", "--arg=--render-distance", "--arg=4"],
                        stdout=log, stderr=subprocess.STDOUT)
                for _ in range(60):
                    if child.poll() is not None:
                        raise RuntimeError(f"Horizon Play exited with {child.returncode}")
                    if socket.exists() and ctl("state")["screen"] == "playing":
                        break
                    time.sleep(1)
                else:
                    raise RuntimeError("Horizon did not open a headless world")
            else:
                manifest = json.loads((version / "native-mods.json").read_text())
                resources = Path.home() / "Library/Application Support/horizon-launcher/versions" / manifest["base"] / ".local/horizon-client/26.3"
                ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                    resources, "--home", home, "--game-dir", home,
                    "--world", "Whale", "--creative", "--test-terrain", "--render-distance", 4, "--wait")

            kind = "naturalist:whale"
            target = f"@e[type={kind},limit=1]"
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("difficulty normal")
            cmd("tp @s 8.5 92 8.5")
            ctl("wait", 20)
            cmd("tick freeze")
            loaded = cmd("forceload add 0 0 47 47")
            assert any("Marked 9 chunks" in row for row in loaded), loaded
            assure("fill 1 91 1 18 91 32 minecraft:stone", "block(s)")
            cmd("fill 1 92 1 18 104 32 minecraft:air")
            cmd("fill 3 92 20 16 99 30 minecraft:water")
            assure(f"summon {kind} 8.5 92 8.5 {{NoAI:1b,NoGravity:1b,Air:0s,Rotation:[0.0f,0.0f]}}", "Summoned")
            observed = {"variant": data(target, "Variant").strip('"')}
            fixture = os.environ.get("NATURALIST_WHALE_FIXTURE") == "1"
            if os.environ.get("NATURALIST_WHALE_SPAWN_FIXTURE") == "1":
                observed.update(inspect("spawn"))
            if fixture:
                sample = inspect("sample")
                observed.update(adult_dims=sample["dims"], adult_part_dims=sample["part_dims"],
                                adult_part_offsets=sample["part_offsets"], pushable=sample["pushable"])
            for attr, key in (("max_health", "max_health"), ("movement_speed", "speed"),
                              ("follow_range", "range"), ("knockback_resistance", "knockback_resistance"),
                              ("step_height", "step")):
                observed[key] = attribute(target, attr)
            step(1)
            observed["dry_air"] = int(data(target, "Air").rstrip("s"))
            if fixture:
                observed["dry_spray"] = inspect("sample")["spray_cooldown"] > 0
            cmd("tp @s 4.25 92 8.5")
            cmd("gamemode survival @s")
            [whale] = ctl("entities", 16, kind)["entities"]
            foods = ("minecraft:cod", "minecraft:cooked_cod", "minecraft:salmon", "minecraft:cooked_salmon",
                     "naturalist:crab_meat", "naturalist:cooked_crab_meat", "minecraft:tropical_fish",
                     "minecraft:pufferfish", "minecraft:beef")
            observed["healing"] = []
            if fixture:
                observed["food"] = [candidate in sample["food"] for candidate in foods]
            for food in foods:
                cmd(f"data merge entity {target} {{Health:60.0f}}")
                cmd(f"item replace entity @s weapon.mainhand with {food} 2")
                ctl("wait", 3)
                assert ctl("look-at-entity", whale["id"])["target"]["entity"]["id"] == whale["id"]
                ctl("mouse", "right", "click")
                ctl("wait", 4)
                health = float(data(target, "Health").rstrip("f"))
                observed["healing"].append([round(health - 60), ctl("state")["held"]["count"]])
            cmd("gamemode creative @s")
            cmd(f"data merge entity {target} {{Age:-24000}}")
            step(1)
            observed["baby_max_health"] = attribute(target, "max_health", "get")
            if fixture:
                sample = inspect("sample")
                observed.update(baby_dims=sample["dims"], baby_part_dims=sample["part_dims"],
                                baby_part_offsets=sample["part_offsets"])
            cmd(f"data merge entity {target} {{Age:0}}")
            observed["regrowth_before_tick_max_health"] = attribute(target, "max_health", "get")
            step(1)
            observed["regrowth_after_tick_max_health"] = attribute(target, "max_health", "get")
            cmd(f"tp {target} 8.5 94.0 24.5")
            cmd(f"data merge entity {target} {{Air:0s}}")
            step(1)
            observed["wet_air"] = int(data(target, "Air").rstrip("s"))
            if fixture:
                observed["deep_spray"] = inspect("sample")["spray_cooldown"] > 0
                cmd(f"tp {target} 8.5 97.0 24.5")
                step(1)
                cooldown = inspect("sample")["spray_cooldown"]
                observed["surface_spray"] = 160 <= cooldown < 320

            cmd(f"tp {target} 8.5 92.0 8.5")
            cmd(f"data merge entity {target} {{Health:80.0f}}")
            cmd("tp @s 8.5 92 15.75")
            ctl("wait", 3)
            ctl("look-at", 8.5, 93.17, 12.5)
            hit = ctl("looking-at")
            assert hit.get("entity", {}).get("id") == whale["id"], hit
            ctl("mouse", "left", "click")
            step(2)
            assert float(data(target, "Health").rstrip("f")) < 80, "Detached head did not damage its parent"

            if fixture:
                cmd("fill 2 107 1 16 124 31 minecraft:air")
                cmd("fill 2 107 1 16 107 31 minecraft:stone")
                cmd("fill 2 108 1 16 123 31 minecraft:water")
                cmd("setblock 13 124 5 minecraft:stone")
                cmd("setblock 10 115 5 minecraft:stone")
                observed.update(inspect("goals"))
            cmd("kill @e[type=minecraft:item]")
            cmd(f"kill {target}")
            step(22)
            fat = '@e[type=minecraft:item,nbt={Item:{id:"naturalist:fat"}},limit=1]'
            drop = data(fat, "Item")
            count = next((int(m[1]) for m in re.finditer(r"count: (\d+)", drop)), None)
            assert count is not None and 2 <= count <= 5, drop
            for key, value in observed.items():
                expected = java[key]
                if isinstance(value, list) and value and isinstance(value[0], float):
                    assert all(math.isclose(x, y, abs_tol=1e-5) for x, y in zip(value, expected, strict=True)), (key, value, expected)
                elif isinstance(value, float):
                    assert math.isclose(value, expected, abs_tol=1e-5), (key, value, expected)
                else:
                    assert value == expected, (key, value, expected)
            print(json.dumps(observed), flush=True)
            print(f"Whale matched {len(observed)} pinned Java observations, a detached-head hit, and a {count}-Fat drop.", flush=True)
        finally:
            if socket.exists():
                ctl("quit")
            if child is not None:
                try:
                    child.wait(timeout=8)
                except subprocess.TimeoutExpired:
                    child.terminate()
                    child.wait(timeout=8)


if __name__ == "__main__":
    replay(Path(sys.argv[1]), json.loads(Path(sys.argv[2]).read_text()))
