"""Check the Whale’s aquatic damage and current handling headlessly."""
import json
import re
from pathlib import Path
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
original = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-whale-natural-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "whale.sock"

    def ctl(*args):
        proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                              check=True, capture_output=True, text=True, timeout=75)
        return json.loads(proc.stdout.splitlines()[-1])

    def cmd(command):
        return ctl("command", command)["chat"]

    try:
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Whale environmental controls", "--seed", 17, "--creative",
            "--test-terrain", "--render-distance", 5, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("difficulty normal")
        cmd("tp @s 6.5 92 2.5")
        ctl("wait-ready")
        cmd("tick freeze")
        cmd("fill 1 91 1 31 91 18 minecraft:stone")
        cmd("fill 1 92 1 31 100 18 minecraft:air")
        cmd("fill 4 92 3 15 99 15 minecraft:water")
        kind = "naturalist:whale"

        def step(ticks):
            cmd(f"tick step {ticks}")
            ctl("wait", ticks + 3)

        def summon(species, position, fields):
            rows = cmd(f"summon {species} {position} {fields}")
            assert any("Summoned" in row for row in rows), rows

        def data(target, name):
            marker = "following entity data: "
            return next(row.split(marker, 1)[1] for row in cmd(f"data get entity {target} {name}") if marker in row)

        def health(target):
            return float(data(target, "Health").rstrip("f"))

        out = {}
        whale = "@e[tag=env_whale,limit=1]"
        summon(kind, "8.5 94 6.5", "{NoAI:1b,NoGravity:1b,Tags:[env_whale]}")
        summon("minecraft:cod", "8.5 94 13.5", "{NoAI:1b,NoGravity:1b,Tags:[env_prey]}")
        start = health(whale)
        cause = "@e[tag=env_prey,limit=1]"
        print("Cod damage", cmd(f"damage {whale} 4 minecraft:mob_attack by {cause}"), flush=True)
        out["cod_hit"] = health(whale) < start
        out["cod_health"] = health(whale)
        cmd(f"data merge entity {whale} {{Health:80.0f}}")
        cmd(f"kill {cause}")
        step(21)
        for species, key in (("naturalist:jellyfish", "jellyfish_immune"), ("minecraft:pufferfish", "pufferfish_immune")):
            summon(species, "8.75 94 6.5", "{NoAI:1b,NoGravity:1b,PuffState:2,Tags:[env_prey]}")
            verify = health(whale)
            step(3)
            out[key] = health(whale) == verify
            cmd(f"kill {cause}")
            step(21)
        cmd(f"kill {whale}")
        step(21)
        for species in ("naturalist:jellyfish", "minecraft:pufferfish"):
            cow = "@e[tag=env_control,limit=1]"
            summon("minecraft:cow", "8.5 94 6.5", "{NoAI:1b,NoGravity:1b,Tags:[env_control]}")
            summon(species, "8.75 94 6.5", "{NoAI:1b,NoGravity:1b,PuffState:2,Tags:[env_prey]}")
            step(3)
            assert health(cow) < 10.0, (species, "contact did not reach the positive control")
            cmd(f"kill {cow}")
            cmd(f"kill {cause}")
            step(21)
        cmd("fill 2 92 1 31 99 18 minecraft:air")
        for x, label, species in ((8, "whale", kind), (20, "control", "minecraft:cow")):
            for dx, level in ((-1, 0), (0, 3), (1, 6)):
                cmd(f"setblock {x + dx} 92 6 minecraft:water[level={level}]")
            summon(species, f"{x + 0.5} 92.01 6.5", f"{{NoAI:1b,NoGravity:1b,Tags:[current_{label}]}}")
        step(1)
        velocity = {label: [round(float(v), 6) for v in re.findall(r"(-?[0-9.eE+-]+)d", data(
            f"@e[tag=current_{label},limit=1]", "Motion"))] for label in ("whale", "control")}
        assert velocity["control"] == [0.014, 0.0, 0.0], velocity
        # The measured stream runs entirely east. The Whale can still resolve
        # body contact along another axis.
        out["current_push"] = abs(velocity["whale"][0]) > 1e-6
        print(json.dumps({"readings": out, "flow_positive_control": velocity}), flush=True)
        expected = {key: original[key] for key in out}
        assert out == expected, (out, expected)
    finally:
        if socket.exists():
            ctl("quit")
