"""Replay Dragonfly water entry on an installed headless Horizon build."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
from launcher_setup import prepare_home

from dragonfly_contact_arena import capture, stepper, vanilla_entry

parser = argparse.ArgumentParser()
parser.add_argument("version", type=Path)
parser.add_argument("--lan", action="store_true")
parser.add_argument("--jar", type=Path, required=True)
args = parser.parse_args()
version = args.version

with tempfile.TemporaryDirectory(prefix="nat-dragonfly-contact-", ignore_cleanup_errors=True) as work:
    root = Path(work)

    def ctl(side, *arguments):
        result = subprocess.run(
            [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, arguments)],
            check=True, capture_output=True, text=True, timeout=70,
        )
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("host", "command", line)["chat"]

    def launch(side, *options):
        home = root / side
        home.mkdir()
        prepare_home(home, args.jar)
        ctl(side, "launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--render-distance", 4, *options)

    def effects():
        sides = ("host", "guest") if args.lan else ("host",)
        if args.lan:
            ctl("guest", "wait", 1)
        result = {}
        for side in sides:
            status = ctl(side, "status")
            result[side] = {
                "bubble": status["particles"].get("bubble", 0),
                "splash": status["particles"].get("splash", 0),
                "splash_sounds": sum(value for key, value in status["heard"].items()
                                     if key.endswith(".splash")),
            }
        return result

    try:
        launch("host", "--world", "Dragonfly water entry", "--creative", "--test-terrain", "--wait")
        cmd("tp @s 8.5 108 8.5")
        ctl("host", "wait", 20)
        if args.lan:
            port = ctl("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            ctl("guest", "connect", f"127.0.0.1:{port}", "SplashWitness")
            ctl("guest", "wait-for", "loaded", "--timeout", 55)
            cmd("gamemode spectator SplashWitness")
            cmd("tp SplashWitness 9.5 104 8.5")
            ctl("guest", "wait", 10)
        wait = stepper(cmd)
        motion = capture(cmd, wait, effects)
        result = {"motion": motion, "vanilla": vanilla_entry(cmd, wait, effects)}
        print(json.dumps(result, indent=2), flush=True)
    finally:
        for side in ("guest", "host"):
            if (root / f"{side}.sock").exists():
                ctl(side, "quit")
