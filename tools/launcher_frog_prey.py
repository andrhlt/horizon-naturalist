"""Replay real frog tongue attacks in a headless Horizon build."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-frog-prey-", ignore_cleanup_errors=True) as temp:
        directory = Path(temp)
        socket = directory / "frog.sock"

        def control(*args):
            proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                  check=True, capture_output=True, text=True, timeout=45)
            return json.loads(proc.stdout.splitlines()[-1])

        def command(line):
            return control("command", line)["chat"]

        def data(tag, field):
            rows = command(f"data get entity @e[tag={tag},limit=1] {field}")
            return next((row.split("following entity data: ", 1)[1]
                         for row in rows if "following entity data: " in row), None)

        def step(ticks):
            command(f"tick step {ticks}")
            control("wait", ticks + 2)

        try:
            control("launch", "--headless", "--game", version / "horizon-client",
                    "--resources", version / ".local/horizon-client/26.3",
                    "--home", directory, "--game-dir", directory, "--world", "Naturalist Frog Prey",
                    "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 106 8.5")
            control("wait", 25)
            command("fill 0 100 0 15 100 15 minecraft:stone")
            command("fill 0 101 0 15 107 15 minecraft:air")
            command("time set midnight")
            command("tick freeze")
            output = {}
            for species in ("dragonfly", "firefly"):
                command("kill @e[tag=frog_fixture]")
                step(22)
                command("summon minecraft:frog 8.5 101 7.5 "
                        "{OnGround:1b,PersistenceRequired:1b,Tags:[frog_fixture,frog_subject]}")
                command(f"summon naturalist:{species} 8.5 101 8.15 "
                        "{NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[frog_fixture,frog_food]}")
                assert data("frog_subject", "Health") and data("frog_food", "Health")
                glowing = False
                for _ in range(30):
                    step(4)
                    glowing |= "minecraft:glowing" in (data("frog_subject", "active_effects") or "")
                    health = data("frog_food", "Health")
                    if health is None or float(health.rstrip("f")) == 0:
                        break
                output[f"naturalist:{species}"] = {
                    "eaten": health is None or float(health.rstrip("f")) == 0,
                    "glowing": glowing,
                }
            return output
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.reference.read_text())
    actual = capture(args.version)
    print(json.dumps(actual, indent=2), flush=True)
    for species, row in expected.items():
        assert actual[species] == {"eaten": row["edible"] and row["hit"],
                                    "glowing": row["glowing_ticks"] > 0}, (species, actual, row)
