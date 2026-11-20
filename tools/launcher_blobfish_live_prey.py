"""Compare live Blobfish targeting behavior with Java Crab and vanilla Cod."""
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
with tempfile.TemporaryDirectory(prefix="nat-blob-prey-", ignore_cleanup_errors=True) as work:
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
                    "--world", f"Blobfish Prey {home.name}", "--creative", "--arg=--control-socket",
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
                "--world", "Blobfish Prey", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("tp @s 12.5 31 6.5")
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("fill 4 28 3 15 28 15 minecraft:stone")
        cmd("fill 4 29 3 15 36 15 minecraft:water")
        cmd("tp @s 14.5 36 13.5")
        cmd('summon naturalist:blobfish 8.5 29.3 8.5 {NoGravity:1b,PersistenceRequired:1b,Tags:[hunter]}')
        cmd('summon minecraft:cod 8.5 29.3 9.5 {NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[control]}')

        def target():
            marker = "Crab fixture "
            rows = cmd("naturalist_fixture_crab prey")
            return next(json.loads(row.split(marker, 1)[1])["target"] for row in rows if marker in row)

        cod = False
        for _ in range(20):
            step(8)
            cod = cod or target() == "minecraft:cod"

        cmd('tp @e[tag=hunter,limit=1] 8.5 29.3 8.5')
        cmd('data merge entity @e[tag=hunter,limit=1] {Motion:[0d,0d,0d]}')
        cmd('summon naturalist:crab 8.9 29.3 8.5 {NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[prey]}')
        prey = "@e[tag=prey,limit=1]"
        hunted = attacked = False
        for _ in range(20):
            step(8)
            hunted = hunted or target() == "naturalist:crab"
            health = float(data(prey, "Health").rstrip("fF"))
            attacked = attacked or health < 8.0
            if hunted and attacked:
                break
        output = {"vanilla_cod_target": cod, "naturalist_crab_target": hunted, "naturalist_crab_hit": attacked}
        print(json.dumps(output), flush=True)
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
