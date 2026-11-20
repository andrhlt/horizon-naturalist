"""Compare Bird NBT age handling with the pinned Java reference, including reload."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import time


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-bird-age-", ignore_cleanup_errors=True) as scratch:
        home = Path(scratch)
        socket = home / "bird-age.sock"

        def control(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def launch():
            control("launch", "--headless", "--game", version / "horizon-client", "--resources",
                    version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                    "--world", "Bird Age Reference", "--creative", "--test-terrain", "--wait")

        def command(text):
            return control("command", text)["chat"]

        def read(tag, field):
            rows = command(f"data get entity @e[tag={tag},limit=1] {field}")
            return next((row.split("following entity data: ", 1)[1] for row in rows
                         if "following entity data: " in row), None)

        def sample():
            out = {}
            for tag in ("bird_plain", "bird_explicit", "parrot", "deer"):
                assert read(tag, "UUID"), (tag, "entity missing")
                fields = ("Age",) if tag == "deer" else ("Age", "ForcedAge", "AgeLocked")
                out[tag] = {field: read(tag, field) for field in fields}
            return out

        def close():
            control("quit")
            deadline = time.monotonic() + 15
            while socket.exists() and time.monotonic() < deadline:
                time.sleep(0.1)
            assert not socket.exists(), "game still closing"

        try:
            launch()
            command("gamerule minecraft:spawn_mobs false")
            command("forceload add 0 0")
            control("wait", 20)
            command("tick freeze")
            cases = (("naturalist:bird", "bird_plain", ""),
                     ("naturalist:bird", "bird_explicit", ",Age:-24000,ForcedAge:22,AgeLocked:1b"),
                     ("minecraft:parrot", "parrot", ",Age:-24000,ForcedAge:22,AgeLocked:1b"),
                     ("naturalist:deer", "deer", ",Age:-24000,ForcedAge:22,AgeLocked:1b"))
            for index, (kind, tag, nbt) in enumerate(cases):
                response = command(f"summon {kind} {5 + index * 2} 101 8 {{NoAI:1b,Tags:[{tag}]{nbt}}}")
                assert any("Summoned" in row for row in response), response
            result = {"initial": sample()}
            for tag in ("bird_explicit", "parrot", "deer"):
                command(f"data merge entity @e[tag={tag},limit=1] {{Age:-12345}}")
            result["after_merge"] = sample()
            close()
            launch()
            command("tick freeze")
            result["reopened"] = sample()
            return result
        finally:
            if socket.exists():
                close()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    args = parser.parse_args()
    result = capture(args.version)
    print(json.dumps(result, indent=2), flush=True)
    expected = json.loads(args.reference.read_text())
    assert result == expected, (result, expected)
