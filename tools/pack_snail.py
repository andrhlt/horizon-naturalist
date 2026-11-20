"""Package Java Naturalist's snail textures, item models, sounds and spawn data."""
import json
from zipfile import ZipFile


def pack_snail(jar, pack, dump):
    exact = {
        "assets/naturalist/textures/item/bucket_content/snail.png",
        "assets/naturalist/blockstates/snail_eggs.json",
        "assets/naturalist/blockstates/snail_shell.json",
        "assets/naturalist/models/block/snail_eggs.json",
        "assets/naturalist/textures/block/snail_eggs.png",
        "data/naturalist/tags/block/alligator_egg_layable_on.json",
        "data/naturalist/tags/block/snail_shell_blacklist.json",
        "data/naturalist/loot_table/entities/snail.json",
        "data/naturalist/tags/worldgen/biome/has_snail.json",
        "data/naturalist/tags/worldgen/biome/blacklist/blacklist_snail.json",
    }
    prefixes = (
        "assets/naturalist/items/snail",
        "assets/naturalist/models/item/snail",
        "assets/naturalist/models/block/snail_shell/",
        "data/naturalist/recipe/snail_shell_",
        "assets/naturalist/textures/item/snail",
        "assets/naturalist/textures/entity/snail/",
        "assets/naturalist/sounds/mob/snail/",
        "data/naturalist/naturalist/snail_variant/",
    )
    with ZipFile(jar) as source:
        for entry in source.namelist():
            if entry.endswith("/"):
                continue
            if entry in exact or entry.startswith(prefixes):
                target = pack / entry
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(source.read(entry))
        for file, select in (
            ("assets/naturalist/lang/en_us.json", lambda key: "snail" in key),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.snail.")),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if select(key)})
            dump(pack, file, existing)
    dump(pack, "data/naturalist/horizon/entity_type/snail.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 3, "minecraft:movement_speed": 0.05000000074505806},
    })
    dump(pack, "data/naturalist/tags/item/snail_food.json", {"values": ["minecraft:beetroot"]})
    dump(pack, "data/naturalist/horizon/item/snail_spawn_egg.json", {"template": "minecraft:cow_spawn_egg"})
    dump(pack, "data/naturalist/horizon/item/snail.json", {"template": "minecraft:stick", "max_stack_size": 1})
    dump(pack, "data/naturalist/horizon/block/snail_eggs.json", {
        "template": "minecraft:dirt",
        "properties": {"hardness": 0, "explosion_resistance": 0, "sounds": "minecraft:frogspawn", "occludes": False},
        "collision_shapes": {"default": []},
        "shapes": {"default": [[0, 0, 0, 1, 1.5 / 16, 1]]},
    })
    colors = ["white", "orange", "magenta", "light_blue", "yellow", "lime", "pink", "gray",
              "light_gray", "cyan", "purple", "blue", "brown", "green", "red", "black"]
    dump(pack, "data/naturalist/horizon/block/snail_shell.json", {
        "template": "minecraft:dirt",
        "properties": {
            "hardness": 0.5, "explosion_resistance": 0.5, "occludes": False,
            "sounds": "minecraft:brain_coral_block", "render_shape": "INVISIBLE",
        },
        "collision_shapes": {"default": [[0.25, 0, 0.25, 0.75, 0.5, 0.75]]},
        "shapes": {"default": [[0.25, 0, 0.25, 0.75, 0.5, 0.75]]},
        "state_properties": {
            "color": {"type": "enum", "values": colors},
            "rotation": {"type": "enum", "values": [str(index) for index in range(16)]},
        },
        "default_state": {"color": "brown", "rotation": "0"},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/snail.json", {
        "type": "neoforge:add_spawns",
        "biomes": "#naturalist:has_snail",
        "spawners": {"type": "naturalist:snail", "weight": 10, "minCount": 2, "maxCount": 3},
    })
