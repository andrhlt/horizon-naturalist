"""Compare Naturalist Animal love-state NBT with the pinned Java inheritance fixture."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, reference):
    with tempfile.TemporaryDirectory(prefix="nat-love-save-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "love-save.sock"

        def ctl(*args):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                check=True, capture_output=True, text=True, timeout=75,
            )
            return json.loads(reply.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        def read(target, key):
            marker = "following entity data: "
            return next((row.split(marker, 1)[1] for row in command(f"data get entity {target} {key}")
                         if marker in row), None)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", directory,
                "--game-dir", directory, "--world", "Naturalist Animal Love Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("forceload add 0 0")
            command("tp @s 7.5 100 6.5")
            ctl("wait", 20)
            command("tick freeze")
            result = {}
            for index, (species, inherits) in enumerate(reference.items()):
                tag = f"animal_love_{index}"
                target = f"@e[tag={tag},limit=1]"
                nbt = "NoAI:1b,NoGravity:1b"
                if inherits:
                    nbt += ",InLove:600,LoveCause:[I;1,2,3,4]"
                spawned = command(f"summon {species} 7.5 100 8.5 {{{nbt},Tags:[{tag}]}}")
                assert any("Summoned" in row for row in spawned), (species, spawned)
                before = [read(target, key) for key in ("InLove", "LoveCause")]
                command(f"data merge entity {target} {{NoAI:0b}}")
                after = [read(target, key) for key in ("InLove", "LoveCause")]
                result[species] = {"before": before, "after": after}
                command(f"kill {target}")
            return result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("inheritance", type=Path)
    parser.add_argument("java_no_ai_reference", type=Path)
    args = parser.parse_args()
    reference = json.loads(args.inheritance.read_text())
    expected = json.loads(args.java_no_ai_reference.read_text())
    observed = capture(args.version, reference)
    errors = []
    for species, inherits in reference.items():
        goal = expected if inherits else {"before": [None, None], "after": [None, None]}
        if observed[species] != goal:
            errors.append((species, observed[species], goal))
    print(json.dumps({"tested": len(observed), "mismatches": errors, "observed": observed}, indent=2), flush=True)
    assert not errors
