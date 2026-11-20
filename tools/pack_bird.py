"""Extract the bird resources from the pinned Naturalist release."""
import json
from zipfile import ZipFile

SPAWNS = ("bluejay", "canary", "cardinal", "finch", "robin", "sparrow")


def pack_bird(jar, pack, dump):
    with ZipFile(jar) as source:
        exact = {
            "assets/naturalist/items/bird_spawn_egg.json",
            "assets/naturalist/models/item/bird_spawn_egg.json",
            "assets/naturalist/textures/item/bird_spawn_egg.png",
            "data/naturalist/loot_table/entities/bird.json",
            "data/naturalist/tags/item/bird_food_items.json",
            *[f"data/naturalist/tags/worldgen/biome/has_{name}.json" for name in SPAWNS],
            *[f"data/naturalist/tags/worldgen/biome/blacklist/blacklist_{name}.json" for name in SPAWNS],
        }
        prefixes = ("assets/naturalist/textures/entity/bird/", "assets/naturalist/sounds/mob/bird/",
                    "data/naturalist/naturalist/bird_variant/")
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith(prefixes)):
                file = pack / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_bytes(source.read(name))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key in ("entity.naturalist.bird", "item.naturalist.bird_spawn_egg")
             or key.startswith("naturalist.subtitles.entity.bird.")),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.bird.")),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if selected(key)})
            dump(pack, file, existing)
    dump(pack, "data/naturalist/horizon/item/bird_spawn_egg.json", {"template": "minecraft:parrot_spawn_egg"})
    dump(pack, "data/naturalist/horizon/entity_type/bird.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 6, "minecraft:movement_speed": 0.2,
                       "minecraft:flying_speed": 0.800000011920929},
    })
    for biome in SPAWNS:
        dump(pack, f"data/naturalist/neoforge/biome_modifier/bird_{biome}.json", {
            "type": "neoforge:add_spawns",
            "biomes": {
                "type": "neoforge:and",
                "values": [
                    f"#naturalist:has_{biome}",
                    {"type": "neoforge:not", "value": f"#naturalist:blacklist/blacklist_{biome}"},
                ],
            },
            "spawners": {"type": "naturalist:bird", "weight": 10, "minCount": 3, "maxCount": 4},
        })
