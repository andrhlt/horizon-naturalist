"""Observe the stock server naturally spawning Whales in a warm ocean water column."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
with tempfile.TemporaryDirectory(prefix="nat-whale-natural-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "whale.sock"

    def ctl(*args):
        proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                              check=True, capture_output=True, text=True, timeout=75)
        return json.loads(proc.stdout.splitlines()[-1])

    def cmd(command):
        return ctl("command", command)["chat"]

    try:
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Whale natural spawning", "--seed", 17, "--creative",
            "--test-terrain", "--render-distance", 5, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("difficulty normal")
        cmd("tp @s 8.5 97 0.5")
        ctl("wait-ready")
        cmd("tick freeze")
        for offset in (0, 16, 32):
            east = 32 + offset
            rows = cmd(f"fill {east} 90 -16 {east + 15} 104 15 minecraft:water")
            assert any("block" in row.lower() for row in rows), rows
            rows = cmd(f"fillbiome {east} 90 -16 {east + 15} 104 15 minecraft:warm_ocean")
            assert any("biome" in row.lower() for row in rows), rows
        kind = "naturalist:whale"
        assert not ctl("entities", 88, kind)["entities"]
        cmd("gamerule minecraft:spawn_mobs true")
        rows = []
        for _ in range(12):
            cmd("tick step 80")
            ctl("wait", 83)
            rows = [row for row in ctl("entities", 88, kind)["entities"]
                    if 32 <= row["pos"][0] <= 80 and 89 <= row["pos"][1] <= 105]
            if any(not row["baby"] for row in rows) and any(row["baby"] for row in rows):
                break
        assert any(not row["baby"] for row in rows), ("No natural adult Whale observed", rows)
        assert any(row["baby"] for row in rows), ("No natural juvenile Whale observed", rows)
        print(json.dumps({"natural_whales": rows}), flush=True)
    finally:
        if socket.exists():
            ctl("quit")
