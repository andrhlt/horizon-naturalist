"""Check Chrysalis block placement through ordinary player interaction in the headless Horizon launcher build."""
import argparse
import json
import subprocess
import tempfile
from pathlib import Path


def capture(version, expected):
    with tempfile.TemporaryDirectory(prefix="nat-bird-flap-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "bird-flap.sock"

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        def step(ticks):
            command(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Chrysalis Placement Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 100 8.5")
            ctl("wait", 20)
            command("fill 0 99 0 15 99 15 minecraft:stone")
            command("fill 0 100 0 15 109 15 minecraft:air")
            command("tp @s 8.5 100 8.5")
            command("time set midnight")
            command("tick freeze")

            command("gamemode survival")
            command("tp @s 8.5 100 6.5")
            command("setblock 7 101 7 minecraft:stone")
            command("setblock 11 101 7 minecraft:oak_log")
            command("item replace entity @s hotbar.0 with naturalist:chrysalis")
            ctl("interact", 7, 101, 7, "east")
            step(3)
            bad_state = ctl("block", 8, 101, 7)["state"]
            [inventory] = command("clear @s naturalist:chrysalis 0")
            held = ctl("status")["held"]
            retained = "Found 1 matching item(s)" in inventory
            client_retained = held is not None and held["item"] == "naturalist:chrysalis"

            command("item replace entity @s hotbar.0 with naturalist:chrysalis")
            ctl("interact", 11, 101, 7, "west")
            step(3)
            good_state = ctl("block", 10, 101, 7)["state"]
            actual = {
                "refuses_stone": bad_state == "minecraft:air" and retained,
                "east_on_log": good_state.startswith("naturalist:chrysalis[")
                    and "facing=east" in good_state and "age=0" in good_state,
            }
            print(json.dumps({"actual": actual, "stone": bad_state, "oak": good_state, "client_retained": client_retained}, indent=2), flush=True)
            assert actual == expected and client_retained, (actual, client_retained)
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.fixture.read_text()))
