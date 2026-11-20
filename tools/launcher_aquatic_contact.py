"""Replay aquatic contact physics on a headless installed version."""
import json
import math
from pathlib import Path
import subprocess
import sys
import tempfile
from aquatic_contact_cases import capture_contact


version = Path(sys.argv[1])
expected = json.loads(Path(sys.argv[2]).read_text()) if len(sys.argv) > 2 else None
with tempfile.TemporaryDirectory(prefix="nat-contact-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "contact.sock"

    def ctl(*args):
        result = subprocess.run(
            [version / "horizon-ctl", "--socket", socket, *map(str, args)],
            check=True, capture_output=True, text=True, timeout=75,
        )
        return json.loads(result.stdout.splitlines()[-1])

    def command(line):
        return ctl("command", line)["chat"]

    def step(ticks):
        command(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    try:
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Aquatic contact", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        command("tp @s 8.5 92 4.5")
        ctl("wait", 20)
        command("tick freeze")
        actual = capture_contact(command, step)
        print(json.dumps(actual), flush=True)
        if expected is not None:
            for name, reference in expected.items():
                readings = actual[name]
                assert all(math.isclose(x, y, abs_tol=2e-5) for x, y in zip(readings, reference, strict=True)), (name, readings, reference)
    finally:
        if socket.exists():
            ctl("quit")
