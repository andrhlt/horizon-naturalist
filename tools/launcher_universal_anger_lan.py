"""Check whether retaliation reaches an unrelated player on a headless LAN world."""
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

        def health():
            rows = command("data get entity @s Health")
            return next(float(row.rsplit(": ", 1)[1].rstrip("Ff"))
                        for row in rows if "following entity data: " in row)

        try:
            launch("host", "--world", "Naturalist LAN Universal Anger", "--creative",
                   "--test-terrain", "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("gamerule minecraft:advance_time false")
            command("time set 2000")
            command("tp @s 8.5 101 5.5")
            command("forceload add 0 0")
            call("host", "wait", 25)
            command("fill 0 100 0 15 100 15 minecraft:grass_block")
            command("fill 0 101 0 15 107 15 minecraft:air")
            port = call("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            call("guest", "connect", f"127.0.0.1:{port}", "AngryGuest")
            call("guest", "wait-for", "loaded", "--timeout", 55)
            command("gamemode survival AngryGuest")
            command("effect give AngryGuest minecraft:resistance infinite 4 true")
            result = {}
            for animal in ("boar", "bear", "black_bear"):
                result[animal] = {}
                for enabled in ("true", "false"):
                    command(f"gamerule minecraft:universal_anger {enabled}")
                    command("gamemode creative @s")
                    command("effect give @s minecraft:instant_health 1 4 true")
                    command("effect give @s minecraft:resistance infinite 3 true")
                    command("tp @s 2.5 101 2.5")
                    command("tp AngryGuest 8.5 101 6.5")
                    call("host", "wait", 10)
                    target = f"@e[type=naturalist:{animal},tag=lan_anger,limit=1]"
                    rows = command(f"summon naturalist:{animal} 8.5 101 8.0 {{Tags:[lan_anger],PersistenceRequired:1b}}")
                    assert any("Summoned" in row for row in rows), rows
                    rows = command(f"damage {target} 1 minecraft:player_attack by AngryGuest")
                    assert any("Applied 1.0 damage" in row for row in rows), rows
                    command("tp AngryGuest 40.5 101 40.5")
                    command("tp @s 8.5 101 6.5")
                    command("gamemode survival @s")
                    before = health()
                    call("host", "wait", 85)
                    result[animal][enabled] = {"includes_bystander": health() < before}
                    command(f"kill {target}")
            return result
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    call(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    options = parser.parse_args()
    result = capture(options.version)
    expected = json.loads(options.fixture.read_text())["universal_anger"]
    print(json.dumps(result, indent=2), flush=True)
    differences = [(animal, rule, sample["includes_bystander"])
                   for animal, values in result.items()
                   for rule, sample in values.items()
                   if sample["includes_bystander"] != expected[animal][rule]["includes_bystander"]]
    assert not differences, differences
