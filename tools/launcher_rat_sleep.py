"""Replay the actual Java Rat sleep controls in a headless Horizon world."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

from rat_sleep_reference import sample

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())

with tempfile.TemporaryDirectory(prefix="nat-rat-sleep-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "control.sock"

    def ctl(*args):
        proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                              check=True, capture_output=True, text=True, timeout=75)
        return json.loads(proc.stdout.splitlines()[-1])

    def command(line):
        return ctl("command", line)["chat"]

    def wait(ticks):
        command(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    def inspect():
        tag = "Rat fixture "
        for row in command("naturalist_fixture_rat adult"):
            if tag in row:
                state = json.loads(row.split(tag, 1)[1])
                return [state["included"], state["sleeping"]]
        raise RuntimeError("Horizon Rat sleep probe returned no value")

    try:
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Naturalist Rat Sleep", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        command("tp @s 7.5 92 3.5")
        ctl("wait", 24)
        command("gamemode spectator @s")
        actual = sample(command, wait, inspect)
        print(actual, flush=True)
        expected = {case: values[:2] for case, values in reference.items()}
        if "--sleep-only" in sys.argv[3:]:
            actual = {case: values[1] for case, values in actual.items()}
            expected = {case: values[1] for case, values in expected.items()}
        assert actual == expected, (expected, actual)
    finally:
        if socket.exists():
            ctl("quit")
