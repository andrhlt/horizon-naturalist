"""Compare the effective default, explicit override and removed Shell components."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-shell-default-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        socket = root / "default.sock"

        def ctl(*args):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                capture_output=True, text=True, timeout=75,
            )
            assert reply.returncode == 0, (args, reply.stdout, reply.stderr)
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def given_matches(given, required):
            cmd("item replace entity @s weapon.mainhand with naturalist:snail_shell" + given)
            test = cmd("execute if items entity @s weapon.mainhand naturalist:snail_shell" + required)
            return any("Test passed" in line for line in test)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", root,
                "--game-dir", root, "--world", "Snail Shell Default", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            brown = "[minecraft:custom_data={Color:12}]"
            white = "[minecraft:custom_data={Color:0}]"
            purple = "[minecraft:custom_data={Color:10}]"
            absence = "[!minecraft:custom_data]"
            return {
                "bare_is_brown": given_matches("", brown),
                "purple_is_purple": given_matches(purple, purple),
                "purple_is_not_brown": not given_matches(purple, brown),
                "white_is_white": given_matches(white, white),
                "removal_stays_absent": given_matches(absence, absence),
            }
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    result = capture(args.version)
    print(json.dumps(result, indent=2), flush=True)
    assert result == json.loads(args.fixture.read_text()), result
