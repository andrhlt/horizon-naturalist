"""Confirm the new follow-range modifier after a real player uses a Deer egg."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, guest=False):
    with tempfile.TemporaryDirectory(prefix="nat-deer-entry-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        def socket_for(side):
            return root / f"{side}.sock"

        def ctl(*args, side="host"):
            proc = subprocess.run([version / "horizon-ctl", "--socket", socket_for(side), *map(str, args)],
                                  check=True, capture_output=True, text=True, timeout=65)
            return json.loads(proc.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        def launch(side, *options):
            home = root / side
            home.mkdir()
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--render-distance", 4, *options, side=side)

        try:
            launch("host", "--world", "Naturalist Deer Spawns", "--creative", "--test-terrain", "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 101 6.5")
            ctl("wait", 35)
            command("fill 2 100 2 15 100 15 minecraft:grass_block")
            command("fill 4 101 4 14 105 14 minecraft:air")
            command("tp @s 8.5 101 6.5")
            command("setblock 7 101 8 minecraft:stone")
            if guest:
                port = ctl("lan", "open", 0, "commands")["port"]
                launch("guest")
                ctl("connect", f"127.0.0.1:{port}", "DeerEggGuest", side="guest")
                ctl("wait-for", "loaded", "--timeout", 55, side="guest")
                command("gamemode creative DeerEggGuest")
                command("tp DeerEggGuest 8.5 101 6.5")
                command("tp @s 11.5 101 6.5")
            actor = "DeerEggGuest" if guest else "@s"
            side = "guest" if guest else "host"
            command(f"item replace entity {actor} weapon.mainhand with naturalist:deer_spawn_egg")
            ctl("look-at", "block", 7, 101, 8, "north", side=side)
            ctl("wait", 6, side=side)
            ctl("mouse", "right", "click", side=side)
            ctl("wait", 10, side=side)
            selector = "@e[type=naturalist:deer,distance=..8,limit=1]"
            variant = command(f"data get entity {selector} Variant")
            assert any('naturalist:deer' in row for row in variant), variant
            attrs = command(f"data get entity {selector} attributes")
            assert any('minecraft:random_spawn_bonus' in row for row in attrs), attrs
            return {"variant": variant, "attributes": attrs}
        finally:
            for side in ("guest", "host"):
                if socket_for(side).exists():
                    ctl("quit", side=side)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--guest", action="store_true")
    args = parser.parse_args()
    print(json.dumps(capture(args.version, args.guest), indent=2))
