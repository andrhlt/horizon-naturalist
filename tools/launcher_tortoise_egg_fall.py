"""Compare who can crush Tortoise eggs while falling onto a physical nest."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

native = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-egg-landings-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "control.sock"

    def ctl(*args):
        result = subprocess.run([native / "horizon-ctl", "--socket", socket, *map(str, args)],
                                check=True, capture_output=True, text=True, timeout=75)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(command):
        return ctl("command", command)["chat"]

    def step(n):
        cmd(f"tick step {n}")
        ctl("wait", n + 2)

    def unchanged():
        rows = cmd("execute if block 8 92 8 naturalist:tortoise_egg[eggs=4,variant=2]")
        return any("test passed" in row.lower() or "succeeded" in row.lower() for row in rows)

    try:
        ctl("launch", "--headless", "--game", native / "horizon-client",
            "--resources", native / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Naturalist egg landings", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:random_tick_speed 0")
        cmd("time set 18000")
        cmd("tp @s 7.5 92 3.5")
        ctl("wait", 25)
        cmd("fill 1 91 1 15 91 14 minecraft:stone")
        cmd("fill 1 92 1 15 97 14 minecraft:air")
        cmd("tick freeze")
        result = {}
        cases = (("tortoise", "naturalist:tortoise", True), ("zombie", "minecraft:zombie", True),
                 ("bat", "minecraft:bat", True), ("cow", "minecraft:cow", True),
                 ("griefing_off", "minecraft:cow", False), ("player", None, False))
        for key, kind, allow in cases:
            if os.environ.get("NATURALIST_FALL_ONLY", key) != key:
                continue
            cmd("gamerule minecraft:mob_griefing " + str(allow).lower())
            cmd("setblock 8 92 8 naturalist:tortoise_egg[eggs=4,variant=2]")
            assert unchanged(), key
            subject = "@s" if kind is None else "@e[tag=walker,limit=1]"
            if kind:
                cmd(f"summon {kind} 8.5 94.6 8.5 {{PersistenceRequired:1b,Tags:[walker]}}")
            else:
                cmd("gamemode survival @s")
            broken = False
            for _ in range(20):
                cmd(f"tp {subject} 8.5 94.6 8.5")
                step(14)
                if not unchanged():
                    broken = True
                    break
            result[f"stomp_{key}"] = broken
            print(key, cmd(f"data get entity {subject} Pos"), broken, flush=True)
            if kind:
                cmd("kill @e[tag=walker]")
                step(24)
            else:
                cmd("gamemode creative @s")
                cmd("tp @s 7.5 92 3.5")
        print(json.dumps(result), flush=True)
        wanted = {key: reference[key] for key in result if key in reference}
        assert all(result[key] == value for key, value in wanted.items()), (wanted, result)
        print("Tortoise egg mob trample matched the Java controls. Player case recorded separately.")
    finally:
        if socket.exists():
            ctl("quit")
