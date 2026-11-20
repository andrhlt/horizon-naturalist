"""Observe a Capture Net swing on both connected headless clients."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


SWING_SOUND = "minecraft:entity.player.attack.sweep"


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-net-lan-", ignore_cleanup_errors=True) as work:
        root = Path(work)

        def control(side, *args):
            result = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                capture_output=True, check=True, text=True, timeout=65)
            return json.loads(result.stdout.splitlines()[-1])

        def launch(side, *args):
            home = root / side
            home.mkdir()
            control(side, "launch", "--headless", "--game", version / "horizon-client",
                    "--resources", version / ".local/horizon-client/26.3", "--home", home,
                    "--game-dir", home, "--render-distance", 4, *args)

        def command(text):
            return control("host", "command", text)["chat"]

        def counters(side):
            status = control(side, "status")
            return (status["heard"].get(SWING_SOUND, 0),
                    status["particles"].get("smoke", 0))

        try:
            launch("host", "--world", "Naturalist Capture Net LAN", "--creative",
                   "--test-terrain", "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 101 6.5")
            control("host", "wait", 25)
            command("fill 0 100 0 15 100 15 minecraft:stone")
            command("fill 0 101 0 15 114 15 minecraft:air")
            command("tp @s 8.5 101 6.5")
            port = control("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            control("guest", "connect", f"127.0.0.1:{port}", "NetGuest")
            control("guest", "wait-for", "loaded", "--timeout", 55)
            command("tp NetGuest 8.5 101 8.5")
            command("gamemode survival @a")

            result = {}
            players = {"host": "@s", "guest": "NetGuest"}
            for thrower, target in players.items():
                command(f"item replace entity {target} weapon.mainhand with naturalist:capture_net")
                control("host", "wait", 5)
                control(thrower, "look-at", 8.5, 115, 9.5)
                baseline = {side: counters(side) for side in players}
                control(thrower, "mouse", "right", "click")
                control("host", "wait", 10)
                result[thrower] = {}
                for side in players:
                    sound, particles = counters(side)
                    result[thrower][side] = {
                        "sounds": sound - baseline[side][0],
                        "particles": particles - baseline[side][1],
                    }
            return result
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    control(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    actual = capture(args.version)
    print(json.dumps(actual, indent=2))
    assert all(result == {"sounds": 1, "particles": 1}
               for peers in actual.values() for result in peers.values()), actual
