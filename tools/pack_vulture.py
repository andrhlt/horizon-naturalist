"""Package the pinned Naturalist Vulture assets and Horizon registry entries."""
import json
from zipfile import ZipFile


def pack_vulture(jar, pack, dump):
    exact = {
        "assets/naturalist/items/vulture_spawn_egg.json",
        "assets/naturalist/models/item/vulture_spawn_egg.json",
        "assets/naturalist/textures/entity/vulture.png",
        "assets/naturalist/textures/item/vulture_spawn_egg.png",
        "data/naturalist/loot_table/entities/vulture.json",
        "data/naturalist/tags/block/vultures_spawnable_on.json",
        "data/naturalist/tags/block/vulture_perch_blocks.json",
        "data/naturalist/tags/entity_type/vulture_hostiles.json",
        "data/naturalist/tags/worldgen/biome/has_vulture.json",
        "data/naturalist/tags/worldgen/biome/blacklist/blacklist_vulture.json",
    }
    prefixes = (
        "assets/naturalist/textures/entity/vulture/",
        "assets/naturalist/sounds/mob/vulture/",
        "data/naturalist/naturalist/vulture_variant/",
    )
    with ZipFile(jar) as source:
        for entry in source.namelist():
            if not entry.endswith("/") and (entry in exact or entry.startswith(prefixes)):
                file = pack / entry
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_bytes(source.read(entry))
        for file, select in (
            ("assets/naturalist/lang/en_us.json", lambda key: "vulture" in key),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.vulture.")),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if select(key)})
            dump(pack, file, existing)
    dump(pack, "data/naturalist/horizon/entity_type/vulture.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {
            "minecraft:max_health": 14.0,
            "minecraft:movement_speed": 0.30000001192092896,
            "minecraft:flying_speed": 0.6000000238418579,
            "minecraft:attack_damage": 4.0,
        },
    })
    dump(pack, "data/naturalist/horizon/item/vulture_spawn_egg.json", {
        "template": "minecraft:parrot_spawn_egg",
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/vulture.json", {
        "type": "neoforge:add_spawns",
        "biomes": "#naturalist:has_vulture",
        "spawners": {"type": "naturalist:vulture", "weight": 3, "minCount": 3, "maxCount": 5},
    })
