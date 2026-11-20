"""Replay real Ant ambient, stone steps, hurt and death sound IDs headlessly."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-ant-audio-", ignore_cleanup_errors=True) as temporary:
        root = Path(temporary)
        socket = root / "ant.sock"

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=75)
            assert result.returncode == 0, (args, result.stdout, result.stderr)
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(text):
            return ctl("command", text)["chat"]

        def sounds():
            return ctl("status")["heard"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", root, "--game-dir", root,
                "--world", "Ant Audio", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 10.5 103 6.5")
            ctl("wait", 15)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 105 15 minecraft:air")
            cmd("tp @s 10.5 100 6.5")
            before = sounds()
            cmd("item replace entity @s weapon.mainhand with naturalist:ant_spawn_egg")
            ctl("wait", 5)
            ctl("look-at-block", 8, 99, 8, "up")
            ctl("mouse", "right", "click")
            ctl("wait", 8)
            cmd("summon naturalist:ant 5.5 100 8.5 {Tags:[ant-step],NoAI:0b,HillCooldown:1200,Motion:[0.8d,-0.02d,0.0d]}")
            ctl("wait", 20)
            cmd("damage @e[tag=ant-step,limit=1] 1 minecraft:generic")
            cmd("summon naturalist:ant 6.5 100 8.5 {Tags:[ant-die]}")
            cmd("damage @e[tag=ant-die,limit=1] 20 minecraft:generic")
            ctl("wait", 25)
            increments = {name: count - before.get(name, 0) for name, count in sounds().items()
                          if count > before.get(name, 0)}
            print(json.dumps(increments, indent=2), flush=True)
            return increments
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())
    sounds = capture(args.version)
    assert all(sounds.get(event["sound"], 0) > 0 for event in expected), sounds
    assert not any("entity.wolf." in name for name in sounds), sounds
