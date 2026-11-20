"""Replay official Naturalist pack formation in the headless Horizon client."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-bass-natural-", ignore_cleanup_errors=True) as work:
    home = Path(work)
    socket = home / "natural.sock"

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                check=True, capture_output=True, text=True, timeout=75)
        return json.loads(result.stdout.splitlines()[-1])

    child = None
    try:
        launcher = os.environ.get("NATURALIST_STOCK_LAUNCHER")
        if launcher:
            log = Path(sys.argv[2]).parent.parent / "natural/play.log"
            with log.open("w") as handle:
                child = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Bass Natural Formation {home.name}", "--creative", "--arg=--control-socket",
                    f"--arg={socket}", "--arg=--headless", "--arg=--test-terrain", "--arg=--render-distance", "--arg=4"],
                    stdout=handle, stderr=subprocess.STDOUT)
            for _ in range(50):
                if child.poll() is not None:
                    raise RuntimeError(f"Horizon Play exited with code {child.returncode}")
                if socket.exists() and ctl("state")["screen"] == "playing":
                    break
                time.sleep(1)
            else:
                raise RuntimeError("Timed out waiting for Horizon Play to load the fixture world")
        else:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Bass Natural Formation", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        result = ctl("command", "naturalist_fixture_bass_natural")
        assert result["ok"], result
        data = next(json.loads(row.split("Bass natural fixture ", 1)[1])
                    for row in result["chat"] if "Bass natural fixture " in row)
        print(json.dumps(data, indent=2), flush=True)
        assert data.pop("rejects_after_target"), data
        reference.pop("cluster_max")
        assert data == reference, (data, reference)
    finally:
        if socket.exists():
            ctl("quit")
        if child is not None:
            try:
                child.wait(timeout=8)
            except subprocess.TimeoutExpired:
                child.terminate()
                child.wait(timeout=8)
