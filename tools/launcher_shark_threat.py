"""Compare Shark’s night/day player acquisition at a stable water edge."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
java = json.loads(Path(sys.argv[2]).read_text()) if len(sys.argv) > 2 else None
kind = "great_white_shark"
with tempfile.TemporaryDirectory(prefix="nat-shark-threat-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "mobs.sock"

    def ctl(*args):
        proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                              check=True, capture_output=True, text=True, timeout=75)
        return json.loads(proc.stdout.splitlines()[-1])

    def cmd(command):
        return ctl("command", command)["chat"]

    def step(n):
        cmd(f"tick step {n}")
        ctl("wait", n + 2)

    def health():
        return ctl("state")["player"]["health"]

    def center_dry():
        return any("Test passed" in s for s in cmd("execute if block 8 92 6 minecraft:oak_sign[waterlogged=false]"))

    try:
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Shark water targeting", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:natural_regeneration false")
        cmd("difficulty normal")
        cmd("time set 6000")
        cmd("weather clear")
        cmd("tp @s 8.5 92 6.5")
        ctl("wait", 20)
        cmd("tick freeze")
        cmd("fill 4 91 3 15 91 12 minecraft:stone")
        cmd("fill 4 92 3 15 96 12 minecraft:air")
        for where in ("6 92 6", "7 92 5", "7 92 7"):
            cmd(f"setblock {where} minecraft:glass")

        tests = (
            ("dry_adult", 9.15, False, False),
            ("water_adult", 7.75, False, False),
            ("edge_adult", 8.12, False, False),
            ("edge_baby", 8.12, True, False),
            ("edge_creative", 8.12, False, True),
        )
        actual = {}
        for light, tick in (("day", 6000), ("night", 18000)):
            cmd(f"time set {tick}")
            step(4)
            actual[light] = {}
            for label, x, baby, creative in tests:
                cmd("gamemode creative @s")
                cmd("kill @e[tag=threat]")
                cmd("effect give @s minecraft:instant_health 1 4 true")
                step(22)
                cmd("setblock 8 92 6 minecraft:oak_sign[waterlogged=false]")
                cmd("setblock 7 92 6 minecraft:water")
                age = "Age:-24000," if baby else ""
                cmd(f"summon naturalist:{kind} 11.0 92 6.5 {{{age}NoGravity:1b,Tags:[threat]}}")
                cmd("gamemode " + ("creative" if creative else "survival") + " @s")
                aimed = False
                for _ in range(16):
                    cmd(f"tp @s {x} 92 6.5")
                    step(4)
                    status = next(json.loads(row.split("Shark fixture ", 1)[1])
                        for row in cmd("naturalist_fixture_shark values") if "Shark fixture " in row)["mobs"]
                    assert center_dry(), (light, label)
                    if any(row["target"] == "Some(Player)" for row in status):
                        aimed = True
                        break
                actual[light][label] = aimed
                print(light, label, aimed, flush=True)

        wanted = {light: {case: item[1] for case, item in cases.items() if case != "light"}
                  for light, cases in java.items()}
        assert actual == wanted, (wanted, actual)
        print(json.dumps(actual), flush=True)
    finally:
        if socket.exists():
            ctl("quit")
