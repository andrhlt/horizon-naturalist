"""Verify guest layer prediction and the host's final state in a headless LAN world."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

GOOP = "naturalist:glow_goop"
AT = (8, 100, 8)


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-goop-lan-", ignore_cleanup_errors=True) as work:
        root = Path(work)

        def ctl(side, *args):
            response = subprocess.run(
                [version / "horizon-ctl", "--socket", root / (side + ".sock"), *map(str, args)],
                capture_output=True, text=True, timeout=75,
            )
            assert response.returncode == 0, (side, args, response.stdout, response.stderr)
            return json.loads(response.stdout.splitlines()[-1])

        def cmd(command):
            return ctl("host", "command", command)["chat"]

        def launch(side, *options):
            home = root / side
            home.mkdir()
            ctl(side, "launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--render-distance", 4, *options)

        def states(pos):
            return [ctl(side, "block", *pos)["state"] for side in ("host", "guest")]

        try:
            launch("host", "--world", "Glow Goop LAN", "--creative", "--test-terrain", "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 100 6.5")
            ctl("host", "wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 107 15 minecraft:air")
            cmd("tp @s 6.5 100 6.5")
            port = ctl("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            ctl("guest", "connect", f"127.0.0.1:{port}", "GoopGuest")
            ctl("guest", "wait-for", "loaded", "--timeout", 55)
            cmd("gamemode survival GoopGuest")
            cmd("tp GoopGuest 8.5 100 6.5")
            cmd(f"item replace entity GoopGuest weapon.mainhand with {GOOP} 5")
            ctl("guest", "wait", 7)
            result = []
            for stage in range(4):
                ctl("guest", "look-at", 8.5, 100.0 if stage == 0 else 100.98, 8.5)
                ctl("guest", "mouse", "right", "click")
                ctl("guest", "wait", 6)
                current = f"{GOOP}[goop={min(stage + 1, 3)},waterlogged=false]"
                above = f"{GOOP}[goop=1,waterlogged=false]" if stage == 3 else "minecraft:air"
                checks = {"base": states(AT), "above": states((8, 101, 8))}
                result.append(checks)
                assert checks == {"base": [current, current], "above": [above, above]}, result
                inventory = ctl("guest", "status")["held"]
                assert inventory["count"] == 4 - stage, inventory
            return result
        finally:
            for side in ("guest", "host"):
                if (root / (side + ".sock")).exists():
                    ctl(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    print(json.dumps(capture(args.version), indent=2), flush=True)
