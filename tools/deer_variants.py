"""Create a Java-compatible custom deer data pack and probe its spawning semantics."""
import json
import time
from pathlib import Path

PACK = "deer_variant_fixture"
NAMESPACE = "deer_fixture"

def write_pack(folder: Path):
    def write(path, data):
        file = folder / path
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_text(json.dumps(data, indent=2) + "\n")

    write("pack.mcmeta", {"pack": {"description": "Deer variant fixture",
                                  "min_format": 88, "max_format": 107}})
    write(f"data/{NAMESPACE}/tags/worldgen/biome/woodland.json",
          {"values": ["minecraft:forest"]})
    variants = {
        "woodland": {"biomes": f"#{NAMESPACE}:woodland", "weight": 1, "priority": 4,
                     "baby_texture": "naturalist:entity/deer/deer_baby"},
        "lower_priority": {"biomes": ["minecraft:forest"], "weight": 999, "priority": 3},
        "disabled": {"biomes": ["minecraft:forest"], "weight": 0, "priority": 99},
        "fallback": {"weight": 1, "priority": 100},
    }
    for name, data in variants.items():
        write(f"data/{NAMESPACE}/naturalist/deer_variant/{name}.json",
              {"texture": "naturalist:entity/deer/white_deer", **data})

def capture(command, y):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 31 15")
    command("tick freeze")

    def variant(selector):
        lines = command(f"data get entity {selector} Variant")
        line = next(line for line in lines if "following entity data: " in line)
        return json.loads(line.split("following entity data: ", 1)[1])

    output = {}
    for scene, x, biome in [("woodland", 8, "forest"), ("fallback", 24, "plains")]:
        command(f"fillbiome {x - 4} {y - 4} 4 {x + 3} {y + 4} 11 minecraft:{biome}")
        # Plain summons run finalizeSpawn. Move any earlier probe outside
        # selection range without changing the biome at the spawn position.
        selector = f"@e[type=naturalist:deer,x={x},y={y},z=8,distance=..2,sort=nearest,limit=1]"
        command(f"kill {selector}")
        command(f"summon naturalist:deer {x + 0.5} {y} 8.5")
        output[scene] = variant(selector)
        assert output[scene] == f"{NAMESPACE}:{scene}", output
    inputs = {
        "explicit": (f'"{NAMESPACE}:woodland"', f"{NAMESPACE}:woodland"),
        "unknown": (f'"{NAMESPACE}:retired"', f"{NAMESPACE}:retired"),
        "legacy": ("127", "naturalist:deer"),
    }
    for name, (token, expected) in inputs.items():
        tag = f"variant_{name}"
        target = f"@e[type=naturalist:deer,tag={tag},limit=1]"
        command(f"kill {target}")
        command(f"summon naturalist:deer 8.5 {y} 12.5 {{NoAI:1b,Variant:{token},Tags:[{tag}]}}")
        output[name] = variant(target)
        assert output[name] == expected, output
    command(f"kill @e[type=naturalist:deer,x=24,y={y},z=8,distance=..2]")
    command(f"kill @e[tag=variant_parents]")
    command(f"fill 20 {y - 1} 10 31 {y - 1} 15 minecraft:stone")
    command(f"fill 20 {y} 10 31 {y + 4} 15 minecraft:air")
    for x in (24.5, 25.5):
        command(f'summon naturalist:deer {x} {y} 12.5 '
                f'{{Age:0,InLove:600,Variant:"{NAMESPACE}:woodland",Tags:[variant_parents]}}')
    command("tick step 85")
    time.sleep(4.6)
    child = (f"@e[type=naturalist:deer,x=25,y={y},z=12,distance=..3,"
             "tag=!variant_parents,sort=nearest,limit=1]")
    output["inherited"] = variant(child)
    assert output["inherited"] == f"{NAMESPACE}:woodland", output
    return output
