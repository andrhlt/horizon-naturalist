"""Recolor one Shell through all 16 Java recipes via a real headless LAN inventory."""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
import time


def capture(version, recipes):
    with tempfile.TemporaryDirectory(prefix="nat-shell-craft-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        script = root / "clicks.json"
        guest_env = {**os.environ, "HORIZON_TICK_SCRIPT": str(script)}

        def ctl(side, *args, env=None):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                capture_output=True, text=True, timeout=75, env=env,
            )
            assert reply.returncode == 0, (args, reply.stdout, reply.stderr)
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("host", "command", text)["chat"]

        def launch(side, *options, env=None):
            home = root / side
            home.mkdir()
            ctl(side, "launch", "--headless", "--game", version / "horizon-client",
                 "--resources", version / ".local/horizon-client/26.3", "--home", home,
                 "--game-dir", home, "--render-distance", 4, *options, env=env)

        def craft(dye):
            # The tick driver translates Horizon menu slots to native packet indices.
            steps = {4: ["key e tap"], 27: ["key e tap"]}
            for tick, slot in zip(range(7, 25, 3), (0, 100, 1, 101, 200, 0), strict=True):
                steps[tick] = [f"click-slot {slot}"]
            script.write_text(json.dumps({"name": dye, "ticks": 30, "steps": steps}))
            script.with_suffix(".done").unlink(missing_ok=True)
            script.with_suffix(".go").touch()
            deadline = time.monotonic() + 8
            while time.monotonic() < deadline:
                if script.with_suffix(".done").exists():
                    assert script.with_suffix(".done").read_text() == dye
                    return
                time.sleep(0.05)
            raise AssertionError(f"Timed out driving survival inventory for {dye}")

        try:
            launch("host", "--world", "Snail Shell Crafting", "--creative", "--test-terrain", "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            port = ctl("host", "lan", "open", 0, "commands")["port"]
            launch("guest", env=guest_env)
            ctl("guest", "connect", f"127.0.0.1:{port}", "CraftGuest")
            ctl("guest", "wait-for", "loaded", "--timeout", 55)
            cmd("gamemode survival CraftGuest")
            cmd("clear CraftGuest")
            cmd("item replace entity CraftGuest hotbar.0 with naturalist:snail_shell")
            actual = {}
            for recipe in recipes:
                shell, dye = recipe["ingredients"]
                expected = recipe["result"]["components"]["minecraft:custom_data"]["Color"]
                cmd(f"item replace entity CraftGuest hotbar.1 with {dye}")
                craft(dye)
                raw = "\n".join(cmd("data get entity CraftGuest Inventory"))
                match = re.search(r"Color:\s*(\d+)\b", raw)
                assert shell in raw and dye not in raw and match, (dye, raw)
                actual[dye] = int(match[1])
                assert actual[dye] == expected, (dye, raw, expected)
            return actual
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    ctl(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())
    print(json.dumps(capture(args.version, expected), indent=2), flush=True)
