"""Start a headless test client through Horizon Launcher's managed Play path."""

import json
import os
from pathlib import Path
import subprocess
import time
import uuid


LAUNCHER = Path("/Applications/Horizon.app/Contents/MacOS/horizon-launcher")
LAUNCHER_HOME = Path.home() / "Library/Application Support/horizon-launcher"


def launch_managed(version, socket, game_dir, world, *, creative=False,
                   test_terrain=False, render_distance=4):
    version = Path(version).resolve()
    socket = Path(socket).resolve()
    game_dir = Path(game_dir).resolve()
    if not (version / ".installed").is_file():
        raise RuntimeError(f"Not an installed managed candidate: {version}")
    manifest_path = version / "native-mods.json"
    if not manifest_path.is_file():
        raise RuntimeError(f"Candidate lacks native-mods.json: {version}")
    manifest = json.loads(manifest_path.read_text())
    base = manifest.get("base")
    if not base:
        raise RuntimeError(f"{version.name} does not declare a launcher base")
    base_dir = LAUNCHER_HOME / "versions" / base
    if not (base_dir / ".installed").is_file():
        raise RuntimeError(f"Candidate launcher base is not installed: {base_dir}")
    resources = base_dir / ".local/horizon-client/26.3"
    if not (resources / "asset-index.json").is_file():
        raise RuntimeError(f"Prepared 26.3 resources are absent: {resources}")

    game_dir.mkdir(parents=True, exist_ok=True)
    world = f"{world} {uuid.uuid4().hex[:8]}"
    command = [str(LAUNCHER), "play", "--instance", "app-2131-verify",
               "--version", version.name, "--world", world, "--json"]
    if creative:
        command.append("--creative")
    for arg in ("--headless", "--control", "--control-socket", str(socket),
                "--resources", str(resources)):
        command.extend(("--arg", arg))
    if test_terrain:
        command.extend(("--arg", "--test-terrain"))
    command.extend(("--arg", "--render-distance", "--arg", str(render_distance)))

    env = os.environ.copy()
    env.update({
        "HORIZON_LAUNCHER_HOME": str(LAUNCHER_HOME),
        "HORIZON_LAUNCHER_DEVELOPER_OFFLINE": "1",
        "HORIZON_NO_AUDIO": "1",
        "HORIZON_CONTROL": str(socket),
    })
    log_path = game_dir / "managed-play.log"
    log = log_path.open("w")
    process = subprocess.Popen(command, cwd=game_dir, env=env, stdout=log,
                               stderr=subprocess.STDOUT, text=True)
    process._naturalist_log = log
    process._naturalist_log_path = log_path
    ctl = version / "horizon-ctl"
    deadline = time.monotonic() + 120
    last_error = None
    while time.monotonic() < deadline:
        if process.poll() is not None:
            break
        probe = subprocess.run([str(ctl), "--socket", str(socket), "status"],
                               capture_output=True, text=True, timeout=5)
        if probe.returncode == 0:
            return process
        last_error = probe.stderr.strip() or probe.stdout.strip()
        time.sleep(0.25)
    process._naturalist_log.close()
    output = log_path.read_text(errors="replace")
    raise RuntimeError(
        f"Managed Play did not become headless-control ready (exit={process.poll()}, "
        f"last control error={last_error!r}). Command: {command!r}\n{output}"
    )


def wait_managed(process):
    if process is None:
        return
    try:
        code = process.wait(timeout=30)
    finally:
        if not process._naturalist_log.closed:
            process._naturalist_log.close()
    if code:
        output = process._naturalist_log_path.read_text(errors="replace")
        raise RuntimeError(f"Managed Play exited {code}:\n{output}")
