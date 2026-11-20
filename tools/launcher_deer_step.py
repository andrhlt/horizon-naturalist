"""Compare the launcher-built deer's emitted steps on stone and grass against Java."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def replay(version, expected):
    with tempfile.TemporaryDirectory(prefix="nat-step-", ignore_cleanup_errors=True) as temp:
        home = Path(temp)
        socket = home / "step.sock"

        def control(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    check=True, capture_output=True, text=True, timeout=45)
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            return control("command", line)["chat"]

        def heard():
            return control("status")["heard"]

        try:
            control("launch", "--headless", "--game", version / "horizon-client", "--resources",
                    version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                    "--world", "Naturalist Deer Steps", "--creative", "--test-terrain",
                    "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("time set noon")
            command("tp @s 11.5 101 7.5")
            control("wait", 30)
            command("fill 0 101 0 15 108 15 minecraft:air")
            command("item replace entity @s weapon.mainhand with minecraft:apple")
            for material, (step, _, _) in zip(("stone", "grass_block"), expected, strict=True):
                command("kill @e[type=naturalist:deer]")
                command(f"fill 0 100 0 15 100 15 minecraft:{material}")
                command("tp @s 11.5 101 7.5")
                command("summon naturalist:deer 3.5 101 7.5 {PersistenceRequired:1b}")
                before = heard()
                caught = {}
                for _ in range(12):
                    control("wait", 10)
                    caught = {sound: count - before.get(sound, 0) for sound, count in heard().items()
                              if count > before.get(sound, 0) and sound.endswith(".step")}
                    if caught:
                        break
                mobs = control("entities", 18, "naturalist:deer")["entities"]
                print(json.dumps({"material":material, "heard":caught,"mobs":mobs}),flush=True)
                assert caught.get(step, 0) > 0, (step, caught)
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    replay(args.version, json.loads(args.reference.read_text()))
