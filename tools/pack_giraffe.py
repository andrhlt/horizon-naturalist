"""Extract the pinned Giraffe content for the Horizon launcher."""
import json
from zipfile import ZipFile


def pack_giraffe(jar, pack, dump):
    egg = "giraffe_spawn_egg"
    exact = {
        *(f"assets/naturalist/{folder}/{egg}.json" for folder in ("items", "models/item")),
        f"assets/naturalist/textures/item/{egg}.png",
        "assets/naturalist/sounds/mob/giraffe_idle.ogg",
        "data/naturalist/advancement/husbandry/ride_giraffe_with_map.json",
        "data/naturalist/loot_table/entities/giraffe.json",
        "data/naturalist/tags/item/giraffe_food_items.json",
        "data/naturalist/tags/worldgen/biome/has_giraffe.json",
        "data/naturalist/tags/worldgen/biome/blacklist/blacklist_giraffe.json",
    }
    prefixes = ("assets/naturalist/textures/entity/giraffe/", "assets/naturalist/sounds/mob/giraffe/",
                "data/naturalist/naturalist/giraffe_variant/")
    with ZipFile(jar) as source:
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith(prefixes)):
                path = pack / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(source.read(name))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key in ("entity.naturalist.giraffe",
                f"item.naturalist.{egg}", "advancements.naturalist.husbandry.ride_giraffe_with_map.title",
                "advancements.naturalist.husbandry.ride_giraffe_with_map.description")
                or ".giraffe." in key),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.giraffe.")),
        ):
            authored = json.loads(source.read(file))
            combined = json.loads((pack / file).read_text())
            combined.update({key: value for key, value in authored.items() if selected(key)})
            dump(pack, file, combined)
    dump(pack, f"data/naturalist/horizon/item/{egg}.json", {"template": "minecraft:cow_spawn_egg"})
    dump(pack, "data/naturalist/horizon/entity_type/giraffe.json", {
        "template": "minecraft:horse",
        "attributes": {"minecraft:max_health": 32, "minecraft:movement_speed": 0.25, "minecraft:step_height": 1},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/giraffe.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_giraffe",
        "spawners": {"type": "naturalist:giraffe", "weight": 5, "minCount": 1, "maxCount": 3},
    })
