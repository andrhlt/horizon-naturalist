"""Exercise Zombie navigation and intentional egg raids in a headless world."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

native = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-egg-raids-", ignore_cleanup_errors=True) as work:
    home = Path(work)
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

    try:
        ctl("launch", "--headless", "--game", native / "horizon-client",
            "--resources", native / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Naturalist egg raids", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("tp @s 7.5 92 3.5")
        ctl("wait", 25)
        cmd("gamemode spectator @s")
        cmd("difficulty normal")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:random_tick_speed 0")
        cmd("time set 18000")
        cmd("fill 1 91 1 15 91 14 minecraft:stone")
        cmd("fill 1 92 1 15 97 14 minecraft:air")
        cmd("tick freeze")
        result = {}
        for egg, allowed in (("alligator", True), ("tortoise", True), ("tortoise", False)):
            key = f"{egg}_{allowed}"
            if key not in reference:
                continue
            cmd("gamerule minecraft:mob_griefing " + str(allowed).lower())
            cmd("setblock 8 92 8 minecraft:air")
            placed = cmd(f"setblock 8 92 8 naturalist:{egg}_egg[eggs=4]")
            assert any("changed" in row.lower() or "set block" in row.lower() for row in placed), (key, placed)
            cmd("summon minecraft:zombie 5.5 92 8.5 {PersistenceRequired:1b,Tags:[raider]}")
            step(222)
            rows = cmd("execute if block 8 92 8 minecraft:air")
            result[key] = any("test passed" in row.lower() or "succeeded" in row.lower() for row in rows)
            cmd("kill @e[tag=raider]")
            step(24)
        print(json.dumps(result), flush=True)
        assert result == reference, (reference, result)
        print(f"Zombie egg raids matched all {len(result)} Java cases.")
    finally:
        if socket.exists():
            ctl("quit")
