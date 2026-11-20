"""Exercise the native Glow Goop client configuration in a real headless client."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import time
import tomllib


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-goop-client-config-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        socket = root / "goop.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", root,
                "--game-dir", root, "--world", "Goop client configuration", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            config = root / "config/naturalist-client.toml"
            default = tomllib.loads(config.read_text())
            assert default == {"glow_goop_tooltip": True}, default
            # Exercise the live parser, then change the valid value with no restart.
            config.write_text("glow_goop_tooltip = [")
            warning = "Invalid Naturalist client config"
            logfile = root / "goop.log"
            deadline = time.monotonic() + 7
            while time.monotonic() < deadline:
                if logfile.exists() and warning in logfile.read_text():
                    break
                time.sleep(0.2)
            assert logfile.exists() and warning in logfile.read_text()
            config.write_text("glow_goop_tooltip = false\n")
            time.sleep(1.4)
            give = ctl("command", "give @s naturalist:glow_goop")["chat"]
            assert any("Glow Goop" in row and "Gave" in row for row in give), give
            placement = ctl("command", "setblock ~ ~ ~2 naturalist:glow_goop[goop=3,waterlogged=true]")["chat"]
            assert any("Changed the block" in row for row in placement), placement
            return {"default": default, "observed_live_client_reload": True, "gameplay_with_tooltip_off": placement}
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    print(json.dumps(capture(args.version), indent=2), flush=True)
