"""Extract Giant Isopod assets, food, loot, and biome data from the pinned release."""
import json
from zipfile import ZipFile

from prepare import aquatic_item

ITEMS = {"giant_isopod_bucket": "cod_bucket", "giant_isopod_spawn_egg": "cow_spawn_egg"}
COLORS = ("brown", "blue")


def pack_giant_isopod(jar, pack, dump):
    with ZipFile(jar) as source:
        exact = {
            *(f"assets/naturalist/{folder}/{item}.json" for item in ITEMS for folder in ("items", "models/item")),
            "assets/naturalist/textures/item/giant_isopod_spawn_egg.png",
            *(f"assets/naturalist/textures/item/bucket_{color}_giant_isopod.png" for color in COLORS),
            "data/naturalist/loot_table/entities/giant_isopod.json",
            "data/naturalist/tags/item/giant_isopod_food.json",
            "data/naturalist/tags/worldgen/biome/has_giant_isopod.json",
            "data/naturalist/tags/worldgen/biome/blacklist/blacklist_giant_isopod.json",
        }
        prefixes = (
            "assets/naturalist/textures/entity/giant_isopod/",
            "assets/naturalist/models/item/variant/giant_isopod_bucket/",
            "assets/naturalist/sounds/mob/giant_isopod/",
            "data/naturalist/naturalist/giant_isopod_variant/",
        )
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith(prefixes)):
                file = pack / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_bytes(source.read(name))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key == "entity.naturalist.giant_isopod"
             or key.removeprefix("item.naturalist.") in ITEMS or key.startswith("naturalist.subtitles.entity.giant_isopod.")),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.giant_isopod.")),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if selected(key)})
            dump(pack, file, existing)
    for item, template in ITEMS.items():
        dump(pack, f"data/naturalist/horizon/item/{item}.json", aquatic_item(template))
    dump(pack, "data/naturalist/horizon/entity_type/giant_isopod.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 12, "minecraft:movement_speed": 0.4},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/giant_isopod.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_giant_isopod",
        "spawners": {"type": "naturalist:giant_isopod", "weight": 8, "minCount": 1, "maxCount": 2},
    })

    for tag in ("aquatic", "arthropod", "can_breathe_under_water"):
        path = f"data/minecraft/tags/entity_type/{tag}.json"
        source = pack / path
        entries = json.loads(source.read_text()) if source.is_file() else {"replace": False, "values": []}
        entries["values"].append("naturalist:giant_isopod")
        dump(pack, path, entries)
