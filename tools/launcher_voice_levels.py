"""Inspect the effective category and volume of actual mob hurt audio."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile


def capture(version, species):
    with tempfile.TemporaryDirectory(prefix="nat-voice-levels-", ignore_cleanup_errors=True) as tmp:
        home = Path(tmp)
        socket = home / "voice.sock"
        log = socket.with_suffix(".log")

        def ctl(*args):
            process = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                     check=True, capture_output=True, text=True, timeout=75,
                                     env={**os.environ, "NATURALIST_VOICE_TRACE": "1"})
            return json.loads(process.stdout.splitlines()[-1])

        def cmd(line):
            result = ctl("command", line)
            if not result["ok"]:
                raise RuntimeError((line, result))
            return result["chat"]

        def launch():
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--world", "Naturalist Voice Levels", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")

        try:
            launch()
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("forceload add 0 0")
            cmd("fill 1 100 1 15 100 15 minecraft:stone")
            cmd("fill 1 101 1 15 105 15 minecraft:air")
            cmd("tp @s 8.5 101 4.5")
            cmd("gamemode spectator @s")
            output = {}
            for animal in species:
                who = f"@e[type=naturalist:{animal},tag=voice_fixture,limit=1]"
                cmd(f"summon naturalist:{animal} 8.5 101 8.5 "
                    "{Tags:[voice_fixture],NoAI:1b,PersistenceRequired:1b}")
                before = len(log.read_text().splitlines())
                applied = cmd(f"damage {who} 1 minecraft:generic")
                assert any("Applied 1.0 damage" in line for line in applied), applied
                ctl("wait", 3)
                observed = [line.split("[naturalist-voice] ", 1)[1]
                            for line in log.read_text().splitlines()[before:]
                            if "[naturalist-voice] " in line]
                emitted = {tuple(line.split()) for line in observed}
                assert len(emitted) == 1, (animal, observed)
                _, volume, source = emitted.pop()
                output[animal] = {"volume": float(volume), "source": source}
                cmd(f"kill {who}")
            return output
        finally:
            if socket.exists():
                ctl("quit")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    options = parser.parse_args()
    expected = json.loads(options.fixture.read_text())
    actual = capture(options.version, expected)
    print(json.dumps(actual, indent=2), flush=True)
    assert actual == expected, (actual, expected)
