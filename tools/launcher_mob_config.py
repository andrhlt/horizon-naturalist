"""Replay Naturalist’s per-mob configuration in a headless Horizon world."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import time

from mob_config_reference import exercise


def replay(version, expected):
    with tempfile.TemporaryDirectory(prefix="nat-config-", ignore_cleanup_errors=True) as temp:
        home = Path(temp)
        socket = home / "step.sock"

        def control(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    check=True, capture_output=True, text=True, timeout=45)
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            return control("command", line)["chat"]

        config = home / "config" / "naturalist-server.toml"
        config.parent.mkdir()
        config.write_text("""[disable_mobs]
remove_all_bugs = true
[disable_mobs.land_mobs]
deer_removed = true
""")

        try:
            control("launch", "--headless", "--game", version / "horizon-client", "--resources",
                    version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                    "--world", "Naturalist Mob Configuration", "--creative", "--test-terrain",
                    "--render-distance", 4, "--wait")
            command("tp @s 8.5 151 5.5")
            control("wait", 25)
            actual = exercise(command, lambda: control("wait", 10))
            print(json.dumps({"configured": actual}, indent=2), flush=True)
            assert actual == expected, (actual, expected)

            config.write_text("[disable_mobs]\nremove_all_bugs = false\n")
            command("tick unfreeze")
            time.sleep(1.4)
            control("wait", 25)
            reloaded = exercise(command, lambda: control("wait", 10))
            assert all(reloaded["summoned"].values()) and all(reloaded["survivors"].values()), reloaded
            print(json.dumps({"after_reload": reloaded}, indent=2), flush=True)
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    replay(args.version, json.loads(args.reference.read_text()))
