"""Extract the duck resources from the pinned Naturalist release."""
import json
from zipfile import ZipFile

ITEMS = {
    "duck": "chicken",
    "cooked_duck": "cooked_chicken",
    "duck_spawn_egg": "chicken_spawn_egg",
    "duck_egg": "egg",
    "duck_bucket": "water_bucket",
    "whistle": "stick",
}


def pack_duck(jar, pack, dump):
    with ZipFile(jar) as source:
        duck_items = {
            f"assets/naturalist/{folder}/{item}{ext}"
            for item in [*ITEMS, "bucket_duckling"]
            for folder, ext in [("items", ".json"), ("models/item", ".json"), ("textures/item", ".png")]
        }
        duck_data = {
            "data/naturalist/loot_table/entities/duck.json",
            "data/naturalist/tags/item/duck_food_items.json",
            "data/naturalist/tags/worldgen/biome/has_duck.json",
            "data/naturalist/tags/worldgen/biome/blacklist/blacklist_duck.json",
            "data/naturalist/recipe/cooked_duck.json",
            "data/naturalist/recipe/whistle.json",
            "data/naturalist/recipe/cooked_duck_from_campfire_cooking.json",
            "data/naturalist/recipe/cooked_duck_from_smoking.json",
        }
        for name in source.namelist():
            if name.endswith("/"):
                continue
            if (name in duck_items or name in duck_data
                    or name.startswith("assets/naturalist/textures/entity/duck/")
                    or name.startswith("assets/naturalist/sounds/mob/duck/")
                    or name.startswith("assets/naturalist/sounds/item/whistle/")
                    or name.startswith("data/naturalist/naturalist/duck_variant/")):
                file = pack / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_bytes(source.read(name))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key == "entity.naturalist.duck"
             or key.removeprefix("item.naturalist.") in ITEMS or key.startswith("naturalist.subtitles.entity.duck.")
             or key.startswith("naturalist.whistle.") or key == "naturalist.subtitles.item.whistle"),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.duck.") or key == "item.whistle"),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if selected(key)})
            dump(pack, file, existing)

    for item, template in ITEMS.items():
        definition = {"template": f"minecraft:{template}"}
        food = {"duck": (2, 1.2), "cooked_duck": (6, 7.2)}.get(item)
        if food:
            nutrition, saturation = food
            definition["components"] = {"minecraft:food": {"nutrition": nutrition, "saturation": saturation}}
        if item == "duck_egg":
            definition["max_stack_size"] = 64
        if item == "whistle":
            definition["max_stack_size"] = 1
        if item == "duck_bucket":
            definition["components"] = {"minecraft:bucket_entity_data": {"Age": -24000}}
        dump(pack, f"data/naturalist/horizon/item/{item}.json", definition)
    dump(pack, "data/naturalist/horizon/entity_type/duck.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 4, "minecraft:movement_speed": 0.25},
    })
    dump(pack, "data/naturalist/horizon/entity_type/duck_egg.json", {
        "template": "minecraft:snowball", "item": "naturalist:duck_egg", "damage": 0,
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/duck.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_duck",
        "spawners": {"type": "naturalist:duck", "weight": 10, "minCount": 3, "maxCount": 4},
    })
