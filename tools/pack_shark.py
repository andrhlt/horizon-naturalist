"""Extract pinned Great White Shark assets, loot, item tags, and spawn tables."""
import json
from pathlib import Path
from zipfile import ZipFile


def pack_shark(jar, pack, dump):
    kind = "great_white_shark"
    tooth = {"assets/naturalist/items/tooth.json", "assets/naturalist/models/item/tooth.json",
             "assets/naturalist/textures/item/tooth.png"}
    with ZipFile(jar) as source:
        for name in source.namelist():
            if name.endswith("/") or not name.startswith(("assets/", "data/")):
                continue
            if kind not in Path(name).parts and kind not in Path(name).name and name not in tooth:
                continue
            destination = pack / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(source.read(name))
        for name in ("lang/en_us", "sounds"):
            path = f"assets/naturalist/{name}.json"
            entries = json.loads((pack / path).read_text())
            entries.update({key: value for key, value in json.loads(source.read(path)).items()
                            if kind in key or key == "item.naturalist.tooth"})
            dump(pack, path, entries)
    dump(pack, f"data/naturalist/horizon/entity_type/{kind}.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {
            "minecraft:max_health": 80, "minecraft:follow_range": 16, "minecraft:movement_speed": 1,
            "minecraft:attack_damage": 8, "minecraft:knockback_resistance": 0.75, "minecraft:step_height": 0.6,
        },
    })
    paths = {"great_white_shark_spawn_egg": "minecraft:cod_spawn_egg", "tooth": "minecraft:bone"}
    for item, template in paths.items():
        dump(pack, f"data/naturalist/horizon/item/{item}.json", {"template": template})
    dump(pack, f"data/naturalist/neoforge/biome_modifier/{kind}.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_great_white_shark",
        "spawners": {"type": f"naturalist:{kind}", "weight": 4, "minCount": 1, "maxCount": 2},
    })
