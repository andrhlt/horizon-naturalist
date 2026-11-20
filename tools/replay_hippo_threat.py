"""Exercise Hippo damage against real player hitboxes at a water's edge."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
java = json.loads(Path(sys.argv[2]).read_text()) if len(sys.argv) > 2 else None
kind = sys.argv[3] if len(sys.argv) > 3 else "hippo"
with tempfile.TemporaryDirectory(prefix="nat-hippo-threat-", ignore_cleanup_errors=True) as scratch:
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
            "--world", "Hippo water targeting", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
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
        states = [
            ("dry_adult", 9.15, False, False, False),
            ("water_adult", 7.75, False, False, False),
            ("edge_adult", 8.12, False, False, False),
            ("edge_baby", 8.12, True, False, False),
            ("edge_owner", 8.12, False, True, False),
            ("edge_creative", 8.12, False, False, True),
        ]
        if kind != "hippo":
            states = [case for case in states if not case[3]]
        uuid = next(row.split("following entity data: ", 1)[1] for row in
            cmd("data get entity @s UUID") if "following entity data: " in row)
        print("UUID read", bool(uuid), flush=True)
        actual = {}
        id = "@e[tag=threat,limit=1]"
        for label, x, baby, owner, creative in states:
            cmd("gamemode creative @s")
            cmd("kill @e[tag=threat]")
            cmd("effect give @s minecraft:instant_health 1 4 true")
            step(24)
            cmd("setblock 8 92 6 minecraft:oak_sign[waterlogged=false]")
            c = ("Age:-24000," if baby else "") + (f"Owner:{uuid}," if owner else "")
            cmd("setblock 7 92 6 minecraft:water")
            cmd(f"summon naturalist:{kind} 10.45 92 6.5 {{{c}Tags:[threat]}}")
            cmd("gamemode " + ("creative" if creative else "survival") + " @s")
            baseline = health()
            for _ in range(22):
                cmd(f"tp @s {x} 92 6.5")
                step(4)
                assert center_dry(), (label, "the dry sign cell changed")
                if health() < baseline:
                    break
            actual[label] = health() < baseline
            print(label, actual[label], baseline, health(), flush=True)
        if java:
            wanted = {key: item[1] for key, item in java.items()}
            assert actual == wanted, (wanted, actual)
        print(json.dumps(actual), flush=True)
    finally:
        if socket.exists():
            ctl("quit")
