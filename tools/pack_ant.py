"""Extract the pinned Java Ant and Ant Hill content into the Horizon pack."""
import json
from zipfile import ZipFile

ANT_ITEMS = {
    "ant": "stick",
    "queen_ant": "stick",
    "ant_spawn_egg": "cow_spawn_egg",
}


def pack_ant(jar, pack, dump):
    items = (*ANT_ITEMS, "ant_hill")
    exact = {
        *(f"assets/naturalist/{folder}/{item}.json" for item in items for folder in ("items", "models/item")),
        *(f"assets/naturalist/textures/item/{item}.png" for item in ANT_ITEMS),
        "assets/naturalist/textures/entity/ant.png",
        "assets/naturalist/blockstates/ant_hill.json",
        "assets/naturalist/models/block/ant_hill.json",
        "assets/naturalist/models/block/ant_hill_open.json",
        "assets/naturalist/textures/block/ant_hill.png",
        "assets/naturalist/textures/block/ant_hill_open.png",
        "data/naturalist/loot_table/entities/ant.json",
        "data/naturalist/loot_table/blocks/ant_hill.json",
        "data/naturalist/tags/worldgen/biome/has_ant_hill.json",
        "data/naturalist/neoforge/biome_modifier/add_ant_hills.json",
        "data/naturalist/worldgen/placed_feature/ant_hill_small.json",
        "data/naturalist/worldgen/placed_feature/ant_hill_big.json",
    }
    prefixes = (
        "assets/naturalist/sounds/mob/ant/",
        "data/naturalist/naturalist/ant_variant/",
    )
    with ZipFile(jar) as source:
        for entry in source.namelist():
            if entry.endswith("/") or (entry not in exact and not entry.startswith(prefixes)):
                continue
            path = pack / entry
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(source.read(entry))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key in {
                "entity.naturalist.ant", "block.naturalist.ant_hill",
                "entity.naturalist.carried_food", *(f"item.naturalist.{item}" for item in ANT_ITEMS),
            } or key.startswith("naturalist.subtitles.entity.ant.")),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.ant.")),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if selected(key)})
            dump(pack, file, existing)
    # Keep the Java placement chain and decoration IDs loadable. These no-op
    # configured features do not build hills: pinned Horizon has no mod feature
    # callback for Naturalist's `basic` feature type yet. See the migration
    # notes and APP-2124's recorded engine limitation.
    for size in ("small", "big"):
        dump(pack, f"data/naturalist/worldgen/feature/ant_hill_{size}.json", {"type": "minecraft:no_op"})
    for item, template in ANT_ITEMS.items():
        definition = {"template": f"minecraft:{template}"}
        if item != "ant_spawn_egg":
            definition["max_stack_size"] = 1
        dump(pack, f"data/naturalist/horizon/item/{item}.json", definition)
    dump(pack, "data/naturalist/horizon/entity_type/ant.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {
            "minecraft:max_health": 6.0,
            "minecraft:movement_speed": 0.2,
            "minecraft:attack_damage": 2.0,
            "minecraft:follow_range": 20.0,
        },
        "sounds": {
            "ambient": "naturalist:entity.ant.ambient",
            "hurt": "naturalist:entity.ant.hurt",
            "death": "naturalist:entity.ant.death",
            "step": "naturalist:entity.ant.step",
        },
    })
    dump(pack, "data/naturalist/horizon/entity_type/carried_food.json", {
        "template": "minecraft:wandering_trader",
    })
    dump(pack, "data/naturalist/horizon/block/ant_hill.json", {
        "template": "minecraft:rooted_dirt",
        "properties": {
            "hardness": 0.5,
            "explosion_resistance": 0.2,
            "sounds": "minecraft:rooted_dirt",
            "random_ticks": {"property": "workers", "values": ["1", "2", "3"]},
        },
        "state_properties": {
            "open": {"type": "enum", "values": ["false", "true"]},
            "workers": {"type": "enum", "values": ["0", "1", "2", "3"]},
            "has_queen": {"type": "enum", "values": ["false", "true"]},
        },
        "default_state": {"open": "false", "workers": "0", "has_queen": "false"},
    })
    tag = "data/minecraft/tags/entity_type/arthropod.json"
    existing = json.loads((pack / tag).read_text()) if (pack / tag).is_file() else {"values": []}
    existing["values"].append("naturalist:ant")
    dump(pack, tag, existing)
