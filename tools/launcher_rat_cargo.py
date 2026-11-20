"""Replay Java Rat cargo persistence and death spill in a headless Horizon world."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import time

from rat_cargo_reference import WORLD, prepare, reopen


version = Path(sys.argv[1])
before, after = (json.loads(Path(path).read_text()) for path in sys.argv[2:4])

with tempfile.TemporaryDirectory(prefix="nat-rat-cargo-", ignore_cleanup_errors=True) as folder:
    home = Path(folder)
    socket = home / "control.sock"

    def ctl(*args):
        proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                              check=True, capture_output=True, text=True, timeout=75)
        return json.loads(proc.stdout.splitlines()[-1])

    def command(line):
        return ctl("command", line)["chat"]

    def launch(*mode):
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", WORLD, *mode, "--test-terrain", "--render-distance", 4, "--wait")

    try:
        launch("--creative")
        command("tp @s 7.5 92 3.5")
        ctl("wait", 24)
        actual = prepare(command)
        print("Before restart", actual, flush=True)
        assert actual == before, (before, actual)
        ctl("quit")
        deadline = time.monotonic() + 10
        while socket.exists() and time.monotonic() < deadline:
            time.sleep(0.1)
        launch()
        actual = reopen(command)
        print("After restart and death", actual, flush=True)
        assert actual == after, (after, actual)
    finally:
        if socket.exists():
            ctl("quit")
