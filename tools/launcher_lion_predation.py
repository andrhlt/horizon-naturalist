"""Check that real Lion targets match night, age, cooldown and prey eligibility."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

native = Path(sys.argv[1])
with tempfile.TemporaryDirectory(prefix="nat-lion-prey-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "control.sock"

    def ctl(*args):
        result = subprocess.run([native / "horizon-ctl", "--socket", socket, *map(str, args)],
                                check=True, capture_output=True, text=True, timeout=75)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(command):
        return ctl("command", command)["chat"]

    def step(ticks):
        cmd(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    def state(query):
        marker = "Lion fixture "
        for row in cmd(f"naturalist_fixture_lion {query}"):
            if marker in row:
                return json.loads(row.split(marker, 1)[1])
        raise AssertionError(query)

    def spawn(kind, x, tag, nbt):
        summon = cmd(f"summon {kind} {x} 92 7.5 {{Tags:[{tag}]{',' if nbt else ''}{nbt}}}")
        assert any("Summoned" in row for row in summon), summon

    try:
        ctl("launch", "--headless", "--game", native / "horizon-client", "--resources",
            native / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Lion prey controls", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:random_tick_speed 0")
        cmd("gamerule minecraft:advance_time false")
        cmd("tp @s 8.5 92 3.5")
        ctl("wait", 24)
        cmd("tick freeze")
        cmd("fill 2 91 2 30 91 14 minecraft:stone")
        cmd("fill 2 92 2 30 98 14 minecraft:air")
        cmd("tp @s 8.5 92 3.5")
        cmd("gamemode spectator @s")
        step(4)

        scenarios = {
            "at_night": (18000, "", "minecraft:horse", "NoAI:1b", False),
            "daylight": (1000, "", "minecraft:horse", "NoAI:1b", False),
            "sated": (18000, "HuntingCooldown:2400", "minecraft:horse", "NoAI:1b", False),
            "juvenile_prey": (18000, "", "minecraft:horse", "NoAI:1b,Age:-24000", False),
            "juvenile_lion": (18000, "Age:-24000", "minecraft:horse", "NoAI:1b", False),
            "unlisted_prey": (18000, "", "minecraft:cow", "NoAI:1b", False),
            "blocked": (18000, "", "minecraft:horse", "NoAI:1b", True),
        }
        observed = {}
        for key, (clock, lion_nbt, prey, prey_nbt, blocked) in scenarios.items():
            cmd(f"time set {clock}")
            if blocked:
                cmd("fill 11 92 2 11 97 14 minecraft:stone")
            spawn("naturalist:lion", 6.5, "hunter", lion_nbt)
            spawn(prey, 17.5, "prey", prey_nbt)
            acquired = None
            for _ in range(20):
                step(6)
                probe = state("cub" if key == "juvenile_lion" else "adult")
                acquired = probe["target"]
                if acquired:
                    break
            observed[key] = acquired
            cmd("kill @e[tag=hunter]")
            cmd("kill @e[tag=prey]")
            step(25)
        print(json.dumps(observed), flush=True)
        assert observed == {name: "minecraft:horse" if name == "at_night" else None for name in scenarios}, observed
    finally:
        if socket.exists():
            ctl("quit")
