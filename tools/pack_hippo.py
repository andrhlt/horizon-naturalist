"""Extract the pinned Java Hippo assets, melon advancement, loot, and spawn list."""
import json
from pathlib import Path
from zipfile import ZipFile


def pack_hippo(jar, pack, dump):
    prefixes = (
        "assets/naturalist/textures/entity/hippo/",
        "assets/naturalist/sounds/mob/hippo/",
        "data/naturalist/naturalist/hippo_variant/",
    )
    with ZipFile(jar) as source:
        for name in source.namelist():
            if not name.endswith("/") and (
                name.startswith(prefixes) or ("hippo" in Path(name).name and name.startswith(("assets/", "data/")))
            ):
                path = pack / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(source.read(name))
        for name in ("lang/en_us", "sounds"):
            path = f"assets/naturalist/{name}.json"
            old = json.loads((pack / path).read_text())
            old.update({key: val for key, val in json.loads(source.read(path)).items() if "hippo" in key})
            dump(pack, path, old)
    dump(pack, "data/naturalist/horizon/entity_type/hippo.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {
            "minecraft:max_health": 70, "minecraft:follow_range": 20, "minecraft:movement_speed": 0.2,
            "minecraft:attack_damage": 9, "minecraft:knockback_resistance": 0.75, "minecraft:step_height": 1,
        },
    })
    dump(pack, "data/naturalist/horizon/item/hippo_spawn_egg.json", {"template": "minecraft:cow_spawn_egg"})
    dump(pack, "data/naturalist/neoforge/biome_modifier/hippo.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_hippo",
        "spawners": {"type": "naturalist:hippo", "weight": 10, "minCount": 1, "maxCount": 3},
    })
