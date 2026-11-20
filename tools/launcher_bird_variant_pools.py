"""Verify biome-dependent Bird finalization through the installed headless client."""
import argparse
from collections import Counter
import json
from pathlib import Path
import socket
import subprocess
import tempfile


def capture(version, reference):
    with tempfile.TemporaryDirectory(prefix="nat-bird-variants-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        path = directory / "variants.sock"

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", path, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def raw(line):
            with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as channel:
                channel.settimeout(15)
                channel.connect(str(path))
                channel.sendall((line + "\n").encode())
                with channel.makefile() as stream:
                    return json.loads(stream.readline())

        def cmd(line):
            return raw("command " + line)["chat"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Bird Variant Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 8.5 100 8.5")
            ctl("wait", 20)
            cmd("tick freeze")
            target = "@e[type=naturalist:bird,x=8.5,y=100,z=8.5,distance=..0.2,limit=1]"
            result = {}
            for index, (biome, java_variants) in enumerate(reference.items()):
                reply = cmd(f"fillbiome 4 96 4 11 103 11 minecraft:{biome}")
                assert any("biome" in row.lower() and "set" in row.lower() for row in reply), reply
                pool = Counter()
                for roll in range(96 if len(java_variants) > 1 else 8):
                    spawned = cmd("summon naturalist:bird 8.5 100 8.5")
                    assert any("Summoned" in row for row in spawned), spawned
                    data = cmd(f"data get entity {target} Variant")
                    variant = next((row.split("following entity data: ", 1)[1].strip('"')
                                    for row in data if "following entity data: " in row), None)
                    pool[str(variant)] += 1
                    cmd(f"tp {target} {0.5 + 0.01 * roll} {104 + index} 8.5")
                result[biome] = dict(sorted(pool.items()))
                print(biome, result[biome], flush=True)
            print(json.dumps(result, indent=2), flush=True)
            assert all(set(result[biome]) == set(variants) for biome, variants in reference.items()), result
        finally:
            if path.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.fixture.read_text()))
