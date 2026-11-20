"""Check the appearance state actually delivered to a second headless client."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-lan-", ignore_cleanup_errors=True) as work:
        root = Path(work)

        def call(side, *args):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                check=True, capture_output=True, text=True, timeout=65)
            return json.loads(reply.stdout.splitlines()[-1])

        def launch(side, *options):
            home = root / side
            home.mkdir()
            call(side, "launch", "--headless", "--game", version / "horizon-client",
                 "--resources", version / ".local/horizon-client/26.3", "--home", home,
                 "--game-dir", home, "--render-distance", 4, *options)

        def command(text):
            return call("host", "command", text)["chat"]

        def snapshots(side):
            return call(side, "status")["naturalist_fixture"]

        try:
            launch("host", "--world", "Naturalist LAN Appearance", "--creative",
                   "--test-terrain", "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 101 5.5")
            call("host", "wait", 25)
            command("fill 0 100 0 15 100 15 minecraft:grass_block")
            command("fill 0 101 0 15 107 15 minecraft:air")
            duck = "@e[tag=fixture_duck,limit=1]"
            deer = "@e[tag=fixture_deer,limit=1]"
            lines = [
                'summon naturalist:duck 8.5 101 8.5 {NoAI:1b,Owner:[I;1,2,3,4],DyeColor:11,Sitting:1b,Tags:[fixture_duck]}',
                'summon naturalist:deer 9.5 101 8.5 {NoAI:1b,Age:-24000,Tags:[fixture_deer]}',
                f"attribute {deer} minecraft:scale base set 2",
            ]
            for line in lines:
                output = command(line)
                assert any("Summoned" in row or "set to 2.0" in row for row in output), (line, output)
            port = call("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            call("guest", "connect", f"127.0.0.1:{port}", "NaturalistGuest")
            call("guest", "wait-for", "loaded", "--timeout", 55)
            command("tp NaturalistGuest 7.5 101 5.5")

            def await_synced(dye, sitting, scale):
                for _ in range(12):
                    call("host", "wait", 10)
                    host = {row["type"].rsplit(":", 1)[-1]: row for row in snapshots("host")}
                    guest = {row["type"].rsplit(":", 1)[-1]: row for row in snapshots("guest")}
                    if host.keys() == guest.keys() == {"deer", "duck"}:
                        if host == guest and guest["duck"]["dye"] == dye and guest["duck"]["sits"] == sitting and guest["deer"]["scale"] == scale:
                            return guest
                raise AssertionError((host, guest, dye, sitting, scale))

            initial = await_synced(11, True, 2.0)
            assert initial["duck"]["variant"] == "naturalist:duck", initial
            assert initial["deer"]["baby"], initial
            command(f"data merge entity {duck} {{DyeColor:5,Sitting:0b}}")
            command(f"attribute {deer} minecraft:scale base set 3")
            updated = await_synced(5, False, 3.0)
            print(json.dumps({"initial": initial, "updated": updated}, indent=2), flush=True)
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    call(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    capture(parser.parse_args().version)
