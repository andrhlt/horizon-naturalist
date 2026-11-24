"""Launch the pinned developer artifact through horizon-ctl for offline fixtures."""
import json
import os
from pathlib import Path
import subprocess
import time

LAUNCHER = Path("/Applications/Horizon.app/Contents/MacOS/horizon-launcher")


class SelectedHeadless:
    def __init__(self, version: Path, socket: Path, world: str, *game_args: str):
        self.version = version
        self.socket = socket
        self.world = world
        self.process = None
        self.env = os.environ.copy()
        self.env.pop("HORIZON_HOME", None)
        socket.parent.mkdir(parents=True, exist_ok=True)
        socket.unlink(missing_ok=True)
        self.log = socket.with_suffix(".launcher.log")
        resources = Path.home() / "Library/Application Support/horizon-launcher/versions/nightly-2026-11-20-0a875da/.local/horizon-client/26.3"
        render_args = [arg for arg in game_args if arg == "--offscreen"]
        render_args.extend(arg for arg in game_args if arg != "--offscreen")
        mode_args = [] if "--offscreen" in render_args else ["--headless"]
        with self.log.open("wb") as output:
            self.process = subprocess.Popen(
                [version / "horizon-ctl", "--socket", socket, "launch",
                 "--world", world, "--game", version / "horizon-client", "--wait",
                 *mode_args, "--test-terrain", "--render-distance", "4",
                 "--name", "Player", "--resources", resources, *render_args],
                stdout=output, stderr=subprocess.STDOUT, env=self.env)
        deadline = time.monotonic() + 90
        while time.monotonic() < deadline:
            if self.process.poll() is not None and not self.socket.exists():
                raise RuntimeError(f"horizon-ctl launch exited {self.process.returncode}: {self.log.read_text(errors='replace')[-3000:]}")
            try:
                self.call("state")
                return
            except (RuntimeError, json.JSONDecodeError, OSError):
                time.sleep(0.25)
        raise TimeoutError(f"No control socket for {world}; see {self.log}")

    def call(self, *args):
        result = subprocess.run(
            [self.version / "horizon-ctl", "--socket", self.socket, *map(str, args)],
            capture_output=True, check=False, text=True, timeout=100, env=self.env)
        if result.returncode:
            raise RuntimeError(f"horizon-ctl failed ({result.returncode}): {result.stderr}\n{result.stdout}")
        return json.loads(result.stdout.splitlines()[-1])

    def command(self, line: str):
        return self.call("command", line)["chat"]

    def close(self):
        if self.process is None:
            return
        try:
            self.call("quit")
        except Exception:
            pass
        try:
            self.process.wait(timeout=30)
        except subprocess.TimeoutExpired:
            self.process.terminate()
            self.process.wait(timeout=10)
