"""Compare Capture Net use in air on a headless Horizon client."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-net-swing-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        socket = root / "net.sock"

        def ctl(*args):
            run = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                 capture_output=True, check=True, text=True, timeout=65)
            return json.loads(run.stdout.splitlines()[-1])

        def command(text):
            return ctl("command", text)["chat"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", root,
                "--game-dir", root, "--world", "Capture Net Swing", "--test-terrain",
                "--creative", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 101 8.5")
            ctl("wait", 25)
            command("fill 0 100 0 15 100 15 minecraft:stone")
            command("fill 0 101 0 15 114 15 minecraft:air")
            command("tp @s 8.5 101 8.5")
            command("gamemode survival @s")
            command("item replace entity @s hotbar.0 with naturalist:capture_net")
            ctl("wait", 5)
            ctl("look-at", 8.5, 115.0, 9.5)
            before = ctl("status")
            ctl("mouse", "right", "click")
            ctl("wait", 6)
            status = ctl("status")
            after = status["heard"]
            sounds = {name: count - before["heard"].get(name, 0) for name, count in after.items()
                      if count > before["heard"].get(name, 0)}
            held = status["held"]
            particles = {name: count - before["particles"].get(name, 0)
                         for name, count in status["particles"].items()
                         if count > before["particles"].get(name, 0)}
            return {"sounds": sounds, "particles": particles, "held": held}
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    result = capture(args.version)
    print(json.dumps(result, indent=2))
    assert result["sounds"].get("minecraft:entity.player.attack.sweep") == 1, result
    assert result["particles"].get("smoke", 0) == 1, result
    assert result["held"] == {"count": 1, "item": "naturalist:capture_net"}, result
