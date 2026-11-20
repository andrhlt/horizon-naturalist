"""Verify Giraffe ownership, seat height and client-authored motion on a headless Horizon LAN world."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-giraffe-lan-", ignore_cleanup_errors=True) as temp:
        root = Path(temp)

        def ctl(side, *args):
            process = subprocess.run([version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                                     check=True, capture_output=True, text=True, timeout=75)
            return json.loads(process.stdout.splitlines()[-1])

        def launch(side, *args):
            home = root / side
            home.mkdir()
            ctl(side, "launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--render-distance", 4, *args)

        def command(text):
            return ctl("host", "command", text)["chat"]

        def data(target, field):
            return next((line.split("following entity data: ", 1)[1]
                         for line in command(f"data get entity {target} {field}")
                         if "following entity data: " in line), None)

        def position(mount):
            return tuple(float(number) for number in re.findall(r"(-?\d+(?:\.\d+)?(?:e[+-]?\d+)?)d", data(mount, "Pos")))

        def guest_click():
            ctl("guest", "look-at", 8.5, 101.75, 8.5)
            ctl("guest", "mouse", "right", "click")
            ctl("host", "wait", 8)

        try:
            launch("host", "--world", "Naturalist Giraffe LAN", "--creative", "--test-terrain", "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("forceload add 0 0 15 31")
            command("tp @s 4.5 101 8.5")
            ctl("host", "wait", 22)
            command("fill 0 100 0 15 100 31 minecraft:stone")
            command("fill 0 101 0 15 106 31 minecraft:air")
            command("tp @s 4.5 101 8.5")
            port = ctl("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            ctl("guest", "connect", f"127.0.0.1:{port}", "GiraffeGuest")
            ctl("guest", "wait-for", "loaded", "--timeout", 55)
            command("tp GiraffeGuest 8.5 101 5.5")
            command("gamemode survival GiraffeGuest")
            command("gamemode spectator @s")
            owner = data("GiraffeGuest", "UUID")
            assert owner
            giraffe = "@e[type=naturalist:giraffe,tag=lan_giraffe,limit=1]"
            rows = command(f"summon naturalist:giraffe 8.5 101 8.5 {{Owner:{owner},Tags:[lan_giraffe]}}")
            assert any("Summoned" in row for row in rows), rows
            ctl("guest", "wait", 12)
            guest_click()
            state = {"guest_mounted": ctl("guest", "status").get("riding") is not None}
            offset = position("GiraffeGuest")[1] - position(giraffe)[1]
            state["java_seat_height"] = abs(offset - 3.24) < 0.03

            ctl("guest", "look-at", 8.5, 104.1, 28.5)
            start = position(giraffe)
            ctl("guest", "key", "w", "down")
            ctl("guest", "wait", 20)
            ctl("guest", "key", "w", "up")
            ctl("host", "wait", 5)
            walked = position(giraffe)
            state["guest_steered"] = walked[2] - start[2] > 1.0

            ctl("guest", "key", "space", "down")
            ctl("guest", "wait", 10)
            ctl("guest", "key", "space", "up")
            heights = [position(giraffe)[1] for _ in range(8)]
            state["space_does_not_jump"] = max(heights) - walked[1] < 0.05
            ctl("guest", "key", "shift", "down")
            ctl("guest", "wait", 4)
            ctl("guest", "key", "shift", "up")
            state["guest_dismounted"] = ctl("guest", "status").get("riding") is None
            return state
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    ctl(side, "quit")


if __name__ == "__main__":
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("version", type=Path)
    args = cli.parse_args()
    actual = capture(args.version)
    print(json.dumps(actual, indent=2), flush=True)
    assert all(actual.values()), actual
