"""Compare Clam launch, shell damage, and inventory with pinned Java."""
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
with tempfile.TemporaryDirectory(prefix="nat-clam-", ignore_cleanup_errors=True) as work:
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
            log = Path(__file__).with_name("play.log")
            with log.open("w") as handle:
                child = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Clam {home.name}", "--creative", "--arg=--control-socket",
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
                "--world", "Clam", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("tp @s 8.5 92 5.5")
        ctl("wait", 18)
        cmd("tick freeze")
        cmd("fill 4 91 3 18 91 16 minecraft:stone")
        cmd("fill 4 92 3 18 98 16 minecraft:air")
        cmd("fill 5 29 5 12 29 12 minecraft:stone")
        cmd("fill 5 30 5 12 31 12 minecraft:water")
        cmd("setblock 8 62 8 minecraft:stone")
        cmd("setblock 8 63 8 minecraft:water")
        cmd('summon naturalist:clam 8.5 92 8.5 {NoGravity:1b,PersistenceRequired:1b,Tags:[shell]}')
        cmd('summon naturalist:clam 12.5 92 8.5 {NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[drown]}')
        shell = "@e[type=naturalist:clam,tag=shell,limit=1]"
        drown = "@e[type=naturalist:clam,tag=drown,limit=1]"
        output = {}
        for attribute, key in (("max_health", "health"), ("knockback_resistance", "knockback_resistance"),
                               ("movement_speed", "speed")):
            rows = cmd(f"attribute {shell} minecraft:{attribute} base get")
            value = next((m[1] for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row))), None)
            assert value is not None, (attribute, rows)
            output[key] = float(value)
        output["variant"] = data(shell, "Variant").strip('"')

        def probe(query="status"):
            marker = "Clam fixture "
            rows = cmd("naturalist_fixture_clam " + query)
            return next(json.loads(row.split(marker, 1)[1]) for row in rows if marker in row)

        def hp(target):
            return float(data(target, "Health").rstrip("fF"))

        def click():
            nearby = ctl("entities", 5, "naturalist:clam")["entities"]
            target = next(entry for entry in nearby if abs(entry["pos"][0] - 8.5) < 1)
            looked = ctl("look-at-entity", target["id"])
            assert looked["target"]["kind"] == "entity", (looked, target)
            ctl("mouse", "right", "click")
            ctl("wait", 4)

        spawn = probe("spawn")
        output["cluster"] = spawn["cluster"]
        assert spawn["supported_water"] and not spawn["open_water"] and not spawn["sea_surface"], spawn
        status = probe()
        output["pushable"] = status["pushable"]
        output["initial_closed"] = not status["open"]
        output["initial_no_treasure"] = not status["treasure"]
        cmd(f"damage {shell} 4 minecraft:generic")
        output["closed_rejects_hit"] = hp(shell) == 30.0
        cmd(f"damage {drown} 2 minecraft:drown")
        output["closed_accepts_drowning"] = hp(drown) == 28.0

        cmd("gamemode survival @s")
        cmd("tp @s 8.5 92 7.5")
        ctl("key", "shift", "down")
        ctl("wait", 8)
        assert ctl("state")["player"]["sneaking"], ctl("state")["player"]
        step(2)
        output["sneak_stays_closed"] = not probe()["open"]
        ctl("key", "shift", "up")
        ctl("wait", 2)
        step(1)
        status = probe()
        output["near_opens"] = status["open"]
        output["launch_delay"] = status["delay"]
        output["launch_cooldown"] = status["cooldown"]
        output["opens_timer_range"] = 100 <= status["timer"] <= 200
        initial = ctl("state")["player"]
        step(3)
        before = ctl("state")["player"]
        output["before_launch"] = before["position"][1] - initial["position"][1]
        step(1)
        after = ctl("state")["player"]
        assert after["position"][1] > before["position"][1] + 1.0, after
        cmd(f"damage {shell} 4 minecraft:generic")
        output["open_accepts_hit"] = hp(shell) == 26.0

        cmd("tp @s 8.5 92 7.5")
        cmd("item replace entity @s weapon.mainhand with minecraft:ender_pearl 3")
        ctl("wait", 3)
        click()
        status = probe()
        output["inserted"] = (status["treasure"] and 'id: "minecraft:ender_pearl"' in data(shell, "equipment.mainhand")
            and ctl("inventory")["counts"].get("minecraft:ender_pearl") == 2)
        output["stored_persistent"] = status["stored_persistent"]
        click()
        status = probe()
        output["returned"] = not status["treasure"] and ctl("inventory")["counts"].get("minecraft:ender_pearl") == 3
        output["withdraw_closes"] = not status["open"]
        output["closed_timer_range"] = 200 <= status["timer"] <= 400

        cmd("gamemode creative @s")
        cmd("item replace entity @s weapon.mainhand with minecraft:debug_stick")
        ctl("wait", 3)
        click()
        status = probe()
        output["creative_debug_opens"] = status["open"]
        output["creative_debug_timer"] = status["timer"]
        output["creative_debug_cooldown"] = status["cooldown"]

        cmd("item replace entity @s weapon.mainhand with minecraft:ender_pearl")
        ctl("wait", 3)
        click()
        cmd("gamemode survival @s")
        cmd("tp @s 8.5 92 5.5")
        probe("reset-snap")
        step(1)
        status = probe()
        output["approach_slams_treasure"] = not status["open"] and status["treasure"]
        output["slam_no_launch"] = status["delay"] == 0

        print(json.dumps(output), flush=True)
        # The immediate velocity increment is measured on the wire in launcher_clam_lan.py.
        expected = {key: value for key, value in reference.items() if key != "launch_vertical"}
        assert output == expected, {key: (value, output.get(key)) for key, value in expected.items()
                                    if output.get(key) != value}
        print(f"Matched {len(expected)} Clam Java fields and observed a live player launch")
    finally:
        if socket.exists():
            ctl("quit")
        if child is not None:
            try:
                child.wait(timeout=8)
            except subprocess.TimeoutExpired:
                child.terminate()
                child.wait(timeout=8)
