"""Copy the pinned Hedgehog assets, variant catalog and gameplay data."""
import json
from zipfile import ZipFile


def pack_hedgehog(jar, pack, dump):
    exact = {
        "assets/naturalist/items/hedgehog.json",
        "assets/naturalist/items/hedgehog_spawn_egg.json",
        "assets/naturalist/models/item/hedgehog.json",
        "assets/naturalist/models/item/hedgehog_spawn_egg.json",
        "data/naturalist/damage_type/hedgehog_throw.json",
        "data/naturalist/loot_table/entities/hedgehog.json",
        "data/naturalist/tags/item/hedgehog_food_items.json",
        "data/naturalist/tags/worldgen/biome/has_hedgehog.json",
        "data/naturalist/tags/worldgen/biome/blacklist/blacklist_hedgehog.json",
    }
    prefixes = (
        "assets/naturalist/models/item/variant/hedgehog_",
        "assets/naturalist/textures/entity/hedgehog/",
        "assets/naturalist/textures/item/hedgehog_",
        "assets/naturalist/sounds/mob/hedgehog/",
        "data/naturalist/naturalist/hedgehog_variant/",
    )
    with ZipFile(jar) as source:
        for name in source.namelist():
            if name.endswith("/") or not (name in exact or name.startswith(prefixes)):
                continue
            destination = pack / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(source.read(name))
        for name, select in (
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.hedgehog.")),
            ("assets/naturalist/lang/en_us.json", lambda key: key in {
                "entity.naturalist.hedgehog", "item.naturalist.hedgehog", "item.naturalist.hedgehog_spawn_egg",
            } or key.startswith(("naturalist.subtitles.entity.hedgehog.", "tooltip.naturalist.hedgehog_"))),
        ):
            existing = json.loads((pack / name).read_text())
            existing.update({key: value for key, value in json.loads(source.read(name)).items() if select(key)})
            dump(pack, name, existing)

    dump(pack, "data/naturalist/horizon/item/hedgehog.json", {
        "template": "minecraft:stick", "max_stack_size": 1,
    })
    dump(pack, "data/naturalist/horizon/item/hedgehog_spawn_egg.json", {
        "template": "minecraft:cow_spawn_egg",
    })
    dump(pack, "data/naturalist/horizon/entity_type/hedgehog.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 12.0, "minecraft:movement_speed": 0.2},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/hedgehog.json", {
        "type": "neoforge:add_spawns",
        "biomes": "#naturalist:has_hedgehog",
        "spawners": {"type": "naturalist:hedgehog", "weight": 10, "minCount": 1, "maxCount": 3},
    })
