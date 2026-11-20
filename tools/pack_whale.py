"""Package Whale assets and worldgen from the pinned Java release."""
import json
from pathlib import Path
from zipfile import ZipFile


def pack_whale(jar, pack, dump):
    kind = "whale"
    with ZipFile(jar) as source:
        for name in source.namelist():
            if name.endswith("/") or not name.startswith(("assets/", "data/")):
                continue
            if kind not in Path(name).parts and kind not in Path(name).name:
                continue
            destination = pack / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(source.read(name))
        for name in ("lang/en_us", "sounds"):
            path = f"assets/naturalist/{name}.json"
            entries = json.loads((pack / path).read_text())
            entries.update({key: value for key, value in json.loads(source.read(path)).items()
                            if kind in key})
            dump(pack, path, entries)
    dump(pack, f"data/naturalist/horizon/entity_type/{kind}.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {
            "minecraft:max_health": 80, "minecraft:follow_range": 24,
            "minecraft:movement_speed": 0.6, "minecraft:knockback_resistance": 0.9,
        },
    })
    dump(pack, "data/naturalist/horizon/item/whale_spawn_egg.json",
         {"template": "minecraft:cod_spawn_egg"})
    dump(pack, "data/naturalist/tags/item/whale_food_items.json", {"values": [
        "minecraft:cod", "minecraft:cooked_cod", "minecraft:salmon", "minecraft:cooked_salmon",
        "naturalist:crab_meat", "naturalist:cooked_crab_meat",
    ]})
    dump(pack, "data/naturalist/tags/worldgen/biome/has_whale.json", {"values": [
        {"id": "#minecraft:is_ocean", "required": False},
        {"id": "#c:is_ocean", "required": False},
    ]})
    dump(pack, "data/naturalist/tags/worldgen/biome/blacklist/blacklist_whale.json", {"values": [
        "minecraft:frozen_ocean", "minecraft:deep_frozen_ocean",
    ]})
    dump(pack, "data/naturalist/neoforge/biome_modifier/whale.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_whale",
        "spawners": {"type": "naturalist:whale", "weight": 5, "minCount": 1, "maxCount": 2},
    })
