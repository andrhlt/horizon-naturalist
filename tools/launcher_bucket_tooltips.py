"""Replay the pinned Jellyfish retaliation, poison, and bucket variants."""
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import time

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-ray-", ignore_cleanup_errors=True) as work:
    home = Path(work)
    socket = home / "fish.sock"

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                capture_output=True, check=True, text=True, timeout=75)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def data(target, key):
        return next(row.split("following entity data: ", 1)[1]
                    for row in cmd(f"data get entity {target} {key}") if "following entity data: " in row)

    def step(ticks):
        cmd(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    child = None
    try:
        launcher = os.environ.get("NATURALIST_STOCK_LAUNCHER")
        if launcher:
            log = Path(sys.argv[2]).parent.parent / "bucket-tooltips/play.log"
            with log.open("w") as handle:
                child = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Bucket Tooltips {home.name}", "--creative", "--arg=--control-socket",
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
                "--world", "Bucket Tooltips", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        inputs = {
            "small_bass": ('bass', 'custom_data', '"naturalist:bass"'),
            "legacy_large_bass": ('bass', 'custom_data', '-1'),
            "legacy_blue_jellyfish": ('jellyfish', 'custom_data', '-2'),
            "legacy_mobula_ray": ('ray', 'custom_data', '-2'),
            "glow_anglerfish": ('anglerfish', 'custom_data', '"naturalist:glow"'),
            "entity_only_ray": ('ray', 'bucket_entity_data', '"naturalist:stingray"'),
            "invalid_ray": ('ray', 'custom_data', '"Not an identifier"'),
            "plain_catfish": ('catfish', 'custom_data', '"naturalist:catfish"'),
        }
        output = {}
        for scenario, (species, tag, variant) in inputs.items():
            cmd(f"item replace entity @s weapon.mainhand with naturalist:{species}_bucket"
                f"[minecraft:{tag}={{Variant:{variant}}}]")
            label = "Bucket tooltip fixture "
            output[scenario] = next(json.loads(row.split(label, 1)[1])
                for row in cmd("naturalist_fixture_bucket_tooltip") if label in row)
        print(json.dumps(output, indent=2), flush=True)
        assert output == reference, (output, reference)
    finally:
        if socket.exists():
            ctl("quit")
        if child is not None:
            try:
                child.wait(timeout=8)
            except subprocess.TimeoutExpired:
                child.terminate()
                child.wait(timeout=8)
