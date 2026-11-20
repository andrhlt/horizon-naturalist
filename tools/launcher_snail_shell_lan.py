"""Verify Snail Shell state, live block-entity updates, and render geometry over LAN."""
import argparse
import json
from pathlib import Path
from socket import AF_UNIX, SOCK_STREAM, socket
import subprocess
import tempfile
import time


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-shell-lan-", ignore_cleanup_errors=True) as work:
        root = Path(work)

        def call(side, *args):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                check=True, capture_output=True, text=True, timeout=65,
            )
            return json.loads(reply.stdout.splitlines()[-1])

        def launch(side, *options):
            home = root / side
            home.mkdir()
            call(side, "launch", "--headless", "--game", version / "horizon-client",
                 "--resources", version / ".local/horizon-client/26.3", "--home", home,
                 "--game-dir", home, "--render-distance", 4, *options)

        def command(text):
            return call("host", "command", text)["chat"]

        def sample(side):
            with socket(AF_UNIX, SOCK_STREAM) as connection:
                connection.settimeout(10)
                connection.connect(str(root / f"{side}.sock"))
                connection.sendall(b"naturalist_snail_shell_lan\n")
                with connection.makefile() as reply:
                    return json.loads(reply.readline())

        def await_shell(side, flower):
            deadline = time.monotonic() + 10
            while time.monotonic() < deadline:
                snapshot = sample(side)
                ready = (snapshot.get("block") == snapshot.get("entity") == "naturalist:snail_shell"
                         and snapshot["color"] == "purple" and snapshot["rotation"] == "2"
                         and snapshot["flower"] == flower and snapshot["vertices"] > 0)
                if ready:
                    assert snapshot["remote"] == (side != "host"), snapshot
                    return snapshot
                call("host", "wait", 5)
            raise AssertionError((side, flower, snapshot))

        def connect(side, name):
            launch(side)
            call(side, "connect", f"127.0.0.1:{port}", name)
            call(side, "wait-for", "loaded", "--timeout", 55)
            command(f"tp {name} 10.5 100 7.5")

        try:
            launch("host", "--world", "Naturalist Shell LAN", "--creative",
                   "--test-terrain", "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 10.5 100 6.5")
            call("host", "wait", 20)
            command("fill 0 99 0 15 99 15 minecraft:stone")
            command("fill 0 100 0 15 107 15 minecraft:air")
            command("setblock 8 100 8 naturalist:snail_shell[color=purple,rotation=2]")
            port = call("host", "lan", "open", 0, "commands")["port"]
            connect("guest", "ShellGuest")
            empty = await_shell("guest", None)
            empty_host = await_shell("host", None)

            command("gamemode survival @s")
            command("item replace entity @s weapon.mainhand with minecraft:dandelion 2")
            call("host", "look-at-block", 8, 100, 8)
            call("host", "mouse", "right", "click")
            flower = ["minecraft:dandelion", 1]
            filled_host = await_shell("host", flower)
            updated = await_shell("guest", flower)
            connect("late", "ShellLate")
            late = await_shell("late", flower)
            for before, after in [(empty_host, filled_host), (empty, updated)]:
                assert after["vertices"] > before["vertices"], (before, after)
                assert after["max_y"] > before["max_y"], (before, after)
            assert empty["vertices"] == empty_host["vertices"], (empty, empty_host)
            assert late["vertices"] == updated["vertices"] == filled_host["vertices"], (late, updated, filled_host)
            return {"empty_host": empty_host, "empty_guest": empty, "filled_host": filled_host,
                    "updated_guest": updated, "late_guest": late}
        finally:
            for side in ("late", "guest", "host"):
                if (root / f"{side}.sock").exists():
                    call(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    print(json.dumps(capture(parser.parse_args().version), indent=2), flush=True)
