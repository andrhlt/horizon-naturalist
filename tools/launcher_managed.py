"""Start a headless fixture client through Horizon Launcher Play.

Using Play gives the temporary fixture home and game directory managed cache
leases before horizon-ctl connects to the control socket.
"""
import os
from pathlib import Path
import subprocess
import time


def start(version: Path, home: Path, socket: Path, world: str, extra_args=()):
    launcher = Path(os.environ.get(
        "HORIZON_LAUNCHER", "/Applications/Horizon.app/Contents/MacOS/horizon-launcher"
    ))
    command = [
        str(launcher), "play", "--allow-jar-downloads", "--instance", "app-2132",
        "--version", version.name, "--world", world, "--creative", "--json",
    ]
    for arg in ("--headless", "--control", "--control-socket", str(socket),
                "--home", str(home), "--game-dir", str(home), *map(str, extra_args)):
        command.extend(("--arg", arg))
    env = os.environ.copy()
    env["HORIZON_LAUNCHER_HOME"] = str(Path.home() / "Library" / "Application Support" / "horizon-launcher")
    env["HORIZON_CONTROL"] = str(socket)
    process = subprocess.Popen(command, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True, env=env)
    deadline = time.monotonic() + 300
    while time.monotonic() < deadline:
        if socket.exists():
            return process
        status = process.poll()
        if status is not None:
            error = process.stderr.read() if process.stderr else ""
            raise RuntimeError(f"Horizon Launcher Play exited {status}: {error}")
        time.sleep(0.1)
    process.terminate()
    raise TimeoutError(f"headless control socket did not open: {socket}")


def stop(process):
    if process is not None:
        try:
            process.wait(timeout=90)
        except subprocess.TimeoutExpired:
            process.terminate()
            process.wait(timeout=15)
