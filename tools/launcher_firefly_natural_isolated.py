"""Check natural nighttime Firefly spawning in an open swamp in the headless Horizon launcher build."""
import argparse
import json
import re
import time
import subprocess
import tempfile
from pathlib import Path


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-bird-flap-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "firefly-surface.sock"
        config = directory / "config/naturalist-server.toml"
        config.parent.mkdir()
        config.write_text("[disable_mobs.bugs]\nbutterfly_removed=true\ndragonfly_removed=true\n")

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        def game_time():
            [line] = command("time query gametime")
            return int(re.findall(r"\d+", line)[-1])

        def sprint(ticks):
            starting_tick = game_time()
            command(f"tick sprint {ticks}")
            deadline = time.monotonic() + 45.0
            while game_time() - starting_tick < ticks:
                assert time.monotonic() < deadline, "natural spawn tick sprint did not finish"
                time.sleep(0.15)
            ctl("wait", 3)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Ambient Spawn Fixture", "--creative", "--test-terrain", "--render-distance", 8, "--wait")
            def report():
                return {name: ctl("entities", 90, f"naturalist:{name}")["entities"]
                        for name in ("butterfly", "dragonfly", "firefly")}

            command("gamerule minecraft:spawn_mobs false")
            command("difficulty peaceful")
            command("gamerule minecraft:advance_time false")
            command("gamerule minecraft:spawn_wandering_traders false")
            command("tp @s 8.5 100 8.5")
            ctl("wait", 80)
            for x in range(-64, 80, 16):
                for z in range(-64, 80, 16):
                    command(f"fillbiome {x} 48 {z} {x + 15} 110 {z + 15} minecraft:swamp")
                    command(f"fill {x} 99 {z} {x + 15} 99 {z + 15} minecraft:grass_block")
            command("time set noon")
            command("gamerule minecraft:spawn_mobs true")
            ctl("wait", 25)
            phases = {}
            observed_sites = {}
            for phase in ("noon", "midnight"):
                command(f"time set {phase}")
                command("kill @e[type=naturalist:butterfly]")
                command("kill @e[type=naturalist:dragonfly]")
                command("kill @e[type=naturalist:firefly]")
                sites = {name: {} for name in ("butterfly", "dragonfly", "firefly")}
                for _ in range(4):
                    command("kill @e[type=minecraft:bat]")
                    sprint(350)
                    for name, rows in report().items():
                        for row in rows:
                            x, y, z = row["pos"]
                            if row["health"] > 0 and -64 <= x < 80 and -64 <= z < 80 and y >= 100.0:
                                sites[name].setdefault(row["id"], [x, y, z])
                phases[phase] = {name: len(present) for name, present in sites.items()}
                observed_sites[phase] = list(sites["firefly"].values())
            print(json.dumps({"counts": phases, "firefly_sites": observed_sites}, indent=2), flush=True)
            assert all(all(rows[s] == 0 for s in ("butterfly", "dragonfly")) for rows in phases.values()), phases
            assert phases["noon"]["firefly"] == 0 and phases["midnight"]["firefly"] > 0, phases
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    capture(args.version)
