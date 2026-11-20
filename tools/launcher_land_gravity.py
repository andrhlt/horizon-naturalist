"""Track actual portal contact and the source-world animals it transports."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

import time

from land_gravity_reference import descent


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-mob-gravity-", ignore_cleanup_errors=True) as tmp:
        home = Path(tmp)
        socket = home / "anger.sock"

        def ctl(*args):
            process = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                     check=True, capture_output=True, text=True, timeout=75)
            return json.loads(process.stdout.splitlines()[-1])

        def cmd(line):
            result = ctl("command", line)
            if not result["ok"]:
                raise RuntimeError((line, result))
            return result["chat"]

        def launch():
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--world", "Naturalist Land Gravity", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")

        try:
            launch()
            cmd("tp @s 8.5 101 4.5")
            return descent(cmd, time.sleep)
        finally:
            if socket.exists():
                ctl("quit")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    options = parser.parse_args()
    sample = capture(options.version)
    expected = json.loads(options.fixture.read_text())
    print(json.dumps(sample, indent=2), flush=True)
    assert sample.keys() == expected.keys()
    for species, heights in sample.items():
        assert len(heights) == len(expected[species])
        assert all(abs(observed - target) <= 1e-6
                   for observed, target in zip(heights, expected[species])), (species, heights, expected[species])
