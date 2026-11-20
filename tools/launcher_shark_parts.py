"""Replay actual arrow impacts at the separate Shark parts against Java."""
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
        cmd("tp @s 8.5 92 2.5")
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("fill 3 91 1 47 91 16 minecraft:stone")
        cmd("fill 3 92 1 47 97 16 minecraft:air")
        readings = {}
        for index, (name, z) in enumerate((("body", 6.5), ("head", 8.5), ("inner_tail", 5.0), ("tail", 3.6), ("outside", 2.5))):
            x = 8.5 + index * 8
            cmd("kill @e[tag=SharkPartFixture]")
            step(21)
            cmd(f"summon {kind} {x} 92 6.5 {{NoAI:1b,NoGravity:1b,Rotation:[0f,0f],Tags:[SharkPartFixture]}}")
            step(2)
            assert inspect()["mobs"][0]["health"] == 80.0
            cmd(f"summon minecraft:arrow {x + 1.3} 92.8 {z} {{NoGravity:1b,Motion:[-1.4d,0d,0d],damage:3.0d,Tags:[SharkPartFixture]}}")
            step(4)
            readings[name] = inspect()["mobs"][0]["health"]
            print(name, readings[name], flush=True)
        assert readings == java, (java, readings)
        print("All Shark body/part arrow impact readings matched Java.", flush=True)
    finally:
        if socket.exists():
            ctl("quit")
        if child is not None:
            try:
                child.wait(timeout=8)
            except subprocess.TimeoutExpired:
                child.terminate()
                child.wait(timeout=8)
