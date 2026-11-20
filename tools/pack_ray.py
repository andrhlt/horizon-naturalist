"""Extract Ray assets and spawn data from the pinned Java release."""
import json
from zipfile import ZipFile

from prepare import aquatic_item

ITEMS = {"ray_bucket": "cod_bucket", "ray_spawn_egg": "cod_spawn_egg"}
VARIANTS = ("eagle_ray", "mobula_ray", "stingray")


def pack_ray(jar, pack, dump):
    with ZipFile(jar) as source:
        exact = {
            *[f"assets/naturalist/{folder}/{item}.json" for item in ITEMS for folder in ("items", "models/item")],
            "assets/naturalist/textures/item/ray_spawn_egg.png",
            *[f"assets/naturalist/textures/item/bucket_{variant}.png" for variant in VARIANTS],
            "data/naturalist/loot_table/entities/ray.json",
            "data/naturalist/tags/worldgen/biome/has_ray.json",
            "data/naturalist/tags/worldgen/biome/blacklist/blacklist_ray.json",
        }
        prefixes = (
            "assets/naturalist/textures/entity/ray/",
            "assets/naturalist/models/item/variant/ray_bucket/",
            "assets/naturalist/sounds/mob/ray/",
            "data/naturalist/naturalist/ray_variant/",
        )
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith(prefixes)):
                file = pack / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_bytes(source.read(name))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key == "entity.naturalist.ray"
             or key.removeprefix("item.naturalist.") in ITEMS or key.startswith("naturalist.subtitles.entity.ray.")
             or key.startswith("tooltip.naturalist.ray_")),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.ray.")),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if selected(key)})
            dump(pack, file, existing)
    for item, template in ITEMS.items():
        dump(pack, f"data/naturalist/horizon/item/{item}.json", aquatic_item(template))
    dump(pack, "data/naturalist/horizon/entity_type/ray.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 14, "minecraft:movement_speed": 1.0, "minecraft:attack_damage": 2},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/ray.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_ray",
        "spawners": {"type": "naturalist:ray", "weight": 8, "minCount": 1, "maxCount": 2},
    })
