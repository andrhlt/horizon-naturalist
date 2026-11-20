"""Observe original Birds created during loading of a fresh taiga region."""
import argparse
from pathlib import Path
import re
import tempfile
import time

import deer_behavior
import java_reference
from bird_generation_pack import write_pack

parser = argparse.ArgumentParser()
for field in ("server", "java_home", "output"):
    parser.add_argument(field, type=Path)
parser.add_argument("--fixture-pack", action="store_true")
args = parser.parse_args()
args.variants = args.grazing = False
args.behavior = True

SELECT = "execute as @e[type=naturalist:bird] at @s if biome ~ ~ ~ minecraft:taiga run data get entity @s Variant"
ANCHOR = re.compile(r"\[(-?\d+), [^,]+, (-?\d+)\]")


def capture(cmd, _):
    cmd("gamerule minecraft:spawn_mobs true")
    cmd("gamerule minecraft:advance_time false")
    cmd("time set noon")
    rows = cmd("execute positioned 4096 100 4096 run locate biome minecraft:taiga")
    location = next((tuple(map(int, match.groups())) for row in rows if (match := ANCHOR.search(row))), None)
    assert location is not None, rows
    x, z = location
    print("Taiga anchor", location, flush=True)
    cmd(f"forceload add {x-64} {z-64} {x+63} {z+63}")
    deadline = time.monotonic() + 48
    variants = []
    while time.monotonic() < deadline:
        time.sleep(1)
        query = f"execute positioned {x} 100 {z} run " + SELECT.replace("type=naturalist:bird", "type=naturalist:bird,distance=..115")
        variants = [row.split("following entity data: ", 1)[-1].strip('"')
                    for row in cmd(query) if "following entity data:" in row]
        if len(variants) >= 3:
            break
    return {"seed": 49903, "taiga_anchor": location, "variants": variants}


path = args.server / "server.properties"
previous = path.read_bytes()
current, replaced = re.subn(rb"(?m)^level-seed=.*$", b"level-seed=49903", previous, count=1)
assert replaced == 1
current, level_type = re.subn(rb"(?m)^level-type=.*$", b"level-type=minecraft:normal", current, count=1)
assert level_type == 1
try:
    path.write_bytes(current)
    deer_behavior.capture = capture
    with tempfile.TemporaryDirectory(prefix="bird-worldgen-", dir=args.server) as world:
        if args.fixture_pack:
            write_pack(Path(world) / "datapacks")
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", Path(world).name])
finally:
    path.write_bytes(previous)
