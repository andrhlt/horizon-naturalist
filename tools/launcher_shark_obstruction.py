"""Compare block collisions on Shark parts with actual native movement."""
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
        cmd("tp @s 4.5 92 2.5")
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("fill 2 91 1 31 91 16 minecraft:stone")
        cmd("fill 2 92 1 31 97 16 minecraft:air")
        out = {}
        for index, (label, block_z) in enumerate((("open", None), ("head", 8), ("tail", 3))):
            cmd("kill @e[tag=SharkPartObstacle]")
            step(21)
            x = 8 + 8 * index
            if block_z is not None:
                cmd(f"setblock {x} 92 {block_z} minecraft:stone")
            cmd(f"summon {kind} {x + 0.5} 92 6.5 {{NoAI:1b,NoGravity:1b,Rotation:[0f,0f],Tags:[SharkPartObstacle]}}")
            step(1)
            position = inspect()["mobs"][0]["position"]
            out[label] = [round(a - b, 8) for a, b in zip(position, (x + 0.5, 92.0, 6.5))]
        print(json.dumps(out), flush=True)
        assert out == java, (out, java)
        if len(sys.argv) > 3:
            wanted = json.loads(Path(sys.argv[3]).read_text())
            cmd("kill @e[tag=SharkPartObstacle]")
            step(21)
            cmd("fill 2 92 1 31 97 16 minecraft:air")
            for x, name, mob_type in ((8, "shark", kind), (20, "control", "minecraft:cow")):
                for dx, level in ((-1, 0), (0, 3), (1, 6)):
                    cmd(f"setblock {x + dx} 92 6 minecraft:water[level={level}]")
                cmd(f"summon {mob_type} {x + 0.5} 92.01 6.5 {{NoAI:1b,NoGravity:1b,Rotation:[0f,0f],Tags:[SharkCurrent{name}]}}")
            step(1)
            measured = {name: [round(float(v), 6) for v in re.findall(r"(-?[0-9.eE+-]+)d",
                data(f"@e[tag=SharkCurrent{name},limit=1]", "Motion"))] for name in ("shark", "control")}
            print("Flowing water", json.dumps(measured), flush=True)
            assert measured == wanted, (measured, wanted)
    finally:
        if socket.exists():
            ctl("quit")
        if child is not None:
            try:
                child.wait(timeout=8)
            except subprocess.TimeoutExpired:
                child.terminate()
                child.wait(timeout=8)
