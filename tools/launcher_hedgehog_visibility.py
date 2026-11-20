"""Compare a live headless LAN Hedgehog's response to two survival players."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-hedgehog-lan-", ignore_cleanup_errors=True) as work:
        root = Path(work)

        def call(side, *args):
            result = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                capture_output=True, text=True, timeout=75,
            )
            assert result.returncode == 0, (side, args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def launch(side, *args):
            home = root / side
            home.mkdir()
            call(side, "launch", "--headless", "--game", version / "horizon-client",
                 "--resources", version / ".local/horizon-client/26.3",
                 "--home", home, "--game-dir", home, "--render-distance", 4, *args)

        def cmd(text):
            return call("host", "command", text)["chat"]

        def heard_hide():
            return call("host", "status")["heard"].get("naturalist:entity.hedgehog.hide", 0)

        try:
            launch("host", "--world", "Hedgehog Visibility", "--creative", "--test-terrain", "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 100 6.5")
            call("host", "wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 107 15 minecraft:air")
            cmd("fill 9 100 8 9 101 8 minecraft:stone")
            cmd("tp @s 8.5 100 6.5")
            cmd("gamemode survival @s")
            cmd("item replace entity @s weapon.mainhand with minecraft:sweet_berries")
            port = call("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            call("guest", "connect", f"127.0.0.1:{port}", "HedgehogGuest")
            call("guest", "wait-for", "loaded", "--timeout", 55)
            cmd("tp HedgehogGuest 10.5 100 8.5")
            cmd("gamemode survival HedgehogGuest")
            call("host", "wait", 10)
            before = heard_hide()
            reply = cmd("summon naturalist:hedgehog 8.5 100 8.5 {Tags:[nearby]}")
            assert any("Summoned" in row for row in reply), reply
            call("host", "wait", 12)
            hidden_behind_wall = heard_hide() == before
            cmd("tp HedgehogGuest 8.5 100 5.5")
            call("host", "wait", 14)
            host = call("host", "status")
            guest = call("guest", "status")
            print(json.dumps({"host_actor": host["player"]["position"],
                              "guest_actor": guest["player"]["position"],
                              "host_heard": host["heard"], "guest_heard": guest["heard"]}), flush=True)
            return {
                "nearer_food_holder_other_visible": heard_hide() > before,
                "other_player_occluded": hidden_behind_wall,
            }
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    call(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    result = capture(args.version)
    print(json.dumps(result, indent=2), flush=True)
    expected = json.loads(args.reference.read_text())
    assert all(expected[key] == value for key, value in result.items()), (result, expected)
