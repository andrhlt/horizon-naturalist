"""Capture six wildlife frames with the supported offscreen renderer."""
import json
from pathlib import Path
import sys
import time
from launcher_selected_headless import SelectedHeadless

version = Path(sys.argv[1])
output = Path(sys.argv[2])
output.mkdir(parents=True, exist_ok=True)
world = f"APP2129 Wildlife Render {time.time_ns()}"
socket = Path("/private/tmp") / f"{world.replace(' ', '-')}.sock"
client = SelectedHeadless(version, socket, world, "--creative", "--offscreen")
ctl = client.call

def cmd(line):
    return ctl("command", line)["chat"]

try:
    cmd("gamerule minecraft:spawn_mobs false")
    cmd("tp @s 8.5 101 5.5 0 13")
    ctl("wait", 20)
    cmd("fill 2 100 2 15 100 15 minecraft:grass_block")
    cmd("fill 2 101 2 15 106 15 minecraft:air")
    cmd("fill 2 99 2 15 99 15 minecraft:stone")
    names = ("alligator", "capybara", "catfish", "clam", "hippo", "piranha")
    records = []
    for name in names:
        x, z = 8.5, 9.5
        if name in ("capybara", "hippo"):
            cmd(f"fill {int(x)-1} 100 {int(z)-1} {int(x)+1} 100 {int(z)+1} minecraft:grass_block")
        cmd("tp @s 8.5 101 5.5 0 15")
        cmd("kill @e[type=minecraft:item]")
        result = cmd(f"summon naturalist:{name} {x} 101 {z} {{NoAI:1b,PersistenceRequired:1b,Rotation:[180f,0f]}}")
        rows = ctl("entities", 32, f"naturalist:{name}")["entities"]
        assert rows, (name, result)
        entity = min(rows, key=lambda row: row["distance"])
        ctl("wait", 3)
        crosshair = ctl("looking-at")
        ctl("key", "f1", "tap")
        ctl("wait", 2)
        path = output / f"{name}.png"
        ctl("screenshot", path)
        ctl("key", "f1", "tap")
        ctl("wait", 3)
        assert path.exists() and path.stat().st_size > 5000, (name, path.stat().st_size if path.exists() else None)
        records.append({"species": name, "entity_id": entity["id"], "frame": str(path),
                        "bytes": path.stat().st_size, "crosshair": crosshair,
                        "artifact": version.name, "offscreen": True})
        print(json.dumps(records[-1]), flush=True)
        cmd(f"kill @e[type=naturalist:{name},limit=1]")
        ctl("wait", 30)
        remaining = ctl("entities", 32, f"naturalist:{name}")["entities"]
        assert not any(row["health"] > 0 for row in remaining), (name, remaining)
    (output / "manifest.json").write_text(json.dumps(records, indent=2) + "\n")
finally:
    client.close()
