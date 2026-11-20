"""Compare every rotated Shark and Whale hitbox face to actual pinned Java geometry."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
baseline = "--baseline" in sys.argv[3:]
env = os.environ.copy()
env.pop("NATURALIST_VERIFY_MULTIPART_GEOMETRY", None)
if not baseline:
    env["NATURALIST_VERIFY_MULTIPART_GEOMETRY"] = "1"


with tempfile.TemporaryDirectory(prefix="nat-rotated-parts-", ignore_cleanup_errors=True) as tmp:
    home = Path(tmp)
    socket = home / "mob.sock"

    def ctl(*args):
        call = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                              env=env, capture_output=True, check=True, text=True, timeout=75)
        return json.loads(call.stdout.splitlines()[-1])

    def command(line):
        return ctl("command", line)["chat"]

    try:
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Naturalist rotated multipart geometry", "--creative", "--test-terrain", "--wait")
        command("gamerule minecraft:spawn_mobs false")
        command("tp @s 32.5 104 32.5")
        ctl("wait-ready")
        command("tick freeze")
        for species, x, data in (("great_white_shark", 30.5, ""), ("whale", 36.5, ""), ("whale", 42.5, "Age:-24000,")):
            rows = command(f"summon naturalist:{species} {x} 100.625 32.875 {{{data}NoAI:1b,NoGravity:1b}}")
            assert any("Summoned" in line for line in rows), rows
        marker = "Multipart rotated fixture "
        rows = command("naturalist_fixture_multipart_rotation")
        actual = next(json.loads(line.partition(marker)[2]) for line in rows if marker in line)
        geometry_error = []
        differences = 0
        for sample, bodies in actual.items():
            for native, java in zip(bodies, reference[sample], strict=True):
                for native_box, java_box in zip(native, java, strict=True):
                    for a, b in zip(native_box, java_box, strict=True):
                        differences += int(a != b)
                        geometry_error.append(abs(a - b))
        result = {"boxes": sum(len(body) for items in actual.values() for body in items),
                  "faces": len(geometry_error), "different": differences,
                  "maximum_error": max(geometry_error)}
        print(json.dumps(result), flush=True)
        if baseline:
            assert result["maximum_error"] > 0.0002, (result, actual)
        else:
            assert actual == reference, (result, actual, reference)
    finally:
        if socket.exists():
            ctl("quit")
