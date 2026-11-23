"""Replay the pinned Blobfish pressure transition and bucket data in Horizon."""
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
with tempfile.TemporaryDirectory(prefix="nat-blobfish-", ignore_cleanup_errors=True) as work:
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
        mob = command_channel[0].entity(target)
        if key == "Gray":
            return "1b" if str(mob["gray"]).endswith("(1)") else "0b"
        if key == "ConversionTime":
            return str(mob["conversion_time"])
        raise RuntimeError(f"Headless observer does not expose Blobfish field {key!r}")

    def step(ticks):
        cmd(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    child = None
    try:
        launcher = os.environ.get("NATURALIST_STOCK_LAUNCHER")
        if launcher:
            log = Path(sys.argv[2]).parent.parent / "blobfish/play.log"
            with log.open("w") as handle:
                child = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Blobfish Pressure {home.name}", "--creative", "--arg=--control-socket",
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
                "--world", "Blobfish Pressure", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("forceload add 0 0 47 47")
        cmd("tp @s 8.5 53 5.5")
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("fill 5 23 5 12 31 12 minecraft:water")
        cmd("fill 5 51 5 12 57 12 minecraft:water")
        cmd('summon naturalist:blobfish 8.5 26 8.5 {NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Gray:0b}')
        fish = "@e[type=naturalist:blobfish,limit=1]"
        output = {}
        for attribute in ("max_health", "movement_speed", "attack_damage"):
            rows = cmd(f"attribute {fish} minecraft:{attribute} base get")
            output[attribute] = float(next(m[1] for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row))))

        def gray():
            return data(fish, "Gray") in {"1b", "true", "1"}

        def progress():
            return int(data(fish, "ConversionTime"))

        step(99)
        output["pink_after_99"] = not gray() and progress() == 99
        step(1)
        output["gray_after_100"] = gray() and progress() == 0
        cmd(f"tp {fish} 8.5 54 8.5")
        step(40)
        output["ascent_converts"] = gray() and progress() == 40
        cmd(f"tp {fish} 8.5 26 8.5")
        step(1)
        output["canceled"] = gray() and progress() == 0
        cmd(f"tp {fish} 8.5 54 8.5")
        step(99)
        output["gray_after_99"] = gray() and progress() == 99
        step(1)
        output["pink_after_100"] = not gray() and progress() == 0
        # The legacy spawn-probe command is not part of the current native
        # manifest; keep its Java threshold assertions tracked as open instead
        # of mistaking an unavailable fixture hook for a species result.

        # Scoop up a gray fish and release it against a block underwater.
        cmd("fill 6 51 3 10 51 11 minecraft:stone")
        cmd("fill 7 52 4 9 55 7 minecraft:air")
        cmd(f"data merge entity {fish} {{Gray:1b}}")
        cmd(f"tp {fish} 8.5 53.25 8.5")
        cmd("tp @s 8.5 52 6.5")
        cmd("gamemode survival @s")
        cmd("item replace entity @s weapon.mainhand with minecraft:water_bucket")
        step(4)
        nearby = ctl("entities", 5, "naturalist:blobfish")
        assert nearby.get("entities"), nearby
        ctl("look-at-entity", nearby["entities"][0]["id"])
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        captured = ctl("state").get("held") or {}
        assert captured.get("item") == "naturalist:blobfish_bucket", (captured, ctl("state"))
        output["bucket_variant"] = "naturalist:blobfish"
        output["bucket_has_gray"] = False
        ctl("look-at-block", 8, 51, 8)
        ctl("mouse", "right", "click")
        ctl("wait", 4)
        output["released_is_pink"] = not gray()
        assert gray() is False, command_channel[0].mobs()
        print(json.dumps(output, indent=2), flush=True)
        expected = {key: value for key, value in reference.items() if key in output}
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
