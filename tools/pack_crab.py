"""Extract the pinned Crab, caught items, food recipes, and spawn data."""
import json
from zipfile import ZipFile

COLORS = ("blue", "brown", "orange", "red", "yellow")
ITEMS = {"crab": "stick", "crab_spawn_egg": "wolf_spawn_egg", "crab_meat": "cod", "cooked_crab_meat": "cooked_cod"}


def pack_crab(jar, pack, dump):
    with ZipFile(jar) as source:
        exact = {
            *(f"assets/naturalist/{folder}/{item}.json" for item in ITEMS for folder in ("items", "models/item")),
            *(f"assets/naturalist/textures/item/{item}.png" for item in ITEMS if item != "crab"),
            *(f"assets/naturalist/textures/item/crab_{color}.png" for color in COLORS),
            *(f"assets/naturalist/models/item/variant/crab_{color}.json" for color in COLORS),
            "data/naturalist/loot_table/entities/crab.json",
            "data/naturalist/tags/item/crab_food.json",
            "data/naturalist/tags/worldgen/biome/has_crab.json",
            "data/naturalist/tags/worldgen/biome/blacklist/blacklist_crab.json",
        }
        prefixes = (
            "assets/naturalist/textures/entity/crab/",
            "assets/naturalist/sounds/mob/crab/",
            "data/naturalist/naturalist/crab_variant/",
            "data/naturalist/recipe/cooked_crab_meat",
        )
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith(prefixes)):
                path = pack / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(source.read(name))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key == "entity.naturalist.crab"
             or key.removeprefix("item.naturalist.") in ITEMS or key.startswith("naturalist.subtitles.entity.crab.")),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.crab.")),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if selected(key)})
            dump(pack, file, existing)
    for item, template in ITEMS.items():
        properties = {"template": f"minecraft:{template}"}
        if item == "crab":
            properties["max_stack_size"] = 1
        dump(pack, f"data/naturalist/horizon/item/{item}.json", properties)
    dump(pack, "data/naturalist/horizon/entity_type/crab.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 8, "minecraft:movement_speed": 0.15, "minecraft:attack_damage": 2},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/crab.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_crab",
        "spawners": {"type": "naturalist:crab", "weight": 10, "minCount": 1, "maxCount": 2},
    })

    arthropods = "data/minecraft/tags/entity_type/arthropod.json"
    path = pack / arthropods
    tag = json.loads(path.read_text()) if path.is_file() else {"replace": False, "values": []}
    if "naturalist:crab" not in tag["values"]:
        tag["values"].append("naturalist:crab")
    dump(pack, arthropods, tag)
