"""Exercise Antivenom potion impacts on two real headless LAN clients."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

from launcher_antivenom import POTION


def capture(version, summoned=False, thrower="host", forms=("splash_potion", "lingering_potion")):
    with tempfile.TemporaryDirectory(prefix="nat-antivenom-lan-", ignore_cleanup_errors=True) as work:
        root = Path(work)

        def call(side, *options):
            process = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, options)],
                check=True, capture_output=True, text=True, timeout=65)
            return json.loads(process.stdout.splitlines()[-1])

        def launch(side, *options):
            home = root / side
            home.mkdir()
            call(side, "launch", "--headless", "--game", version / "horizon-client",
                 "--resources", version / ".local/horizon-client/26.3", "--home", home,
                 "--game-dir", home, "--render-distance", 4, *options)

        def command(line):
            return call("host", "command", line)["chat"]

        def effects(side):
            return {entry["id"].removeprefix("minecraft:") for entry in call(side, "effects")["effects"]["active"]}

        def wait_until(expected):
            for _ in range(12):
                call("host", "wait", 5)
                result = {side: sorted(effects(side)) for side in ("host", "guest")}
                if all(set(value) == expected for value in result.values()):
                    return result
            raise AssertionError((expected, result))

        try:
            launch("host", "--world", "Naturalist Antivenom LAN", "--creative", "--test-terrain", "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 101 6.8")
            call("host", "wait", 25)
            command("fill 1 100 1 15 100 15 minecraft:stone")
            command("fill 1 101 1 15 106 15 minecraft:air")
            command("tp @s 8.5 101 6.8")
            port = call("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            call("guest", "connect", f"127.0.0.1:{port}", "CureFixture")
            call("guest", "wait-for", "loaded", "--timeout", 55)
            command("tp CureFixture 8.5 101 8.5")
            command("gamemode survival @a")
            if thrower == "guest":
                command("tp @s 8.5 101 8.5")
                command("tp CureFixture 8.5 101 6.8")
            for side in ("host", "guest"):
                assert abs(call(side, "state")["player"]["position"][1] - 101.0) < 0.1

            output = {}
            for form in forms:
                command("effect give @a minecraft:poison 30 0 true")
                command("effect give @a minecraft:regeneration 30 0 true")
                wait_until({"poison", "regeneration"})
                if summoned:
                    rows = command(f'summon minecraft:{form} 8.5 101.14 7.65 '
                                   f'{{Motion:[0.0d,-0.2d,0.0d],Item:{{id:"minecraft:{form}",'
                                   'count:1,components:{"minecraft:potion_contents":'
                                   '{potion:"naturalist:antivenom"}}}}')
                    assert any("Summoned" in row for row in rows), rows
                else:
                    target = "@s" if thrower == "host" else "CureFixture"
                    command(f"item replace entity {target} weapon.mainhand with minecraft:{form}[{POTION}]")
                    call(thrower, "look-at", 8.5, 101.2, 8.5)
                    call(thrower, "mouse", "right", "click")
                try:
                    output[form] = wait_until({"regeneration"})
                except AssertionError:
                    for side, player in (("host", "@s"), ("guest", "CureFixture")):
                        print(form, side, command(f"data get entity {player} active_effects"), flush=True)
                        print(form, side, call(side, "state")["player"], flush=True)
                    raise
            return output
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    call(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--summoned", action="store_true")
    parser.add_argument("--thrower", choices=("host", "guest"), default="host")
    parser.add_argument("--form", choices=("splash_potion", "lingering_potion"), action="append")
    args = parser.parse_args()
    options = {"forms": args.form} if args.form else {}
    print(json.dumps(capture(args.version, args.summoned, args.thrower, **options), indent=2))
