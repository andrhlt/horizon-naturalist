"""Copy the Java Glow Goop assets and register its waterlogged Horizon block."""
import json
from zipfile import ZipFile


def pack_glow_goop(jar, pack, dump):
    files = (
        "assets/naturalist/blockstates/glow_goop.json",
        "assets/naturalist/items/glow_goop.json",
        "assets/naturalist/models/item/glow_goop.json",
        "assets/naturalist/textures/item/glow_goop.png",
        *(f"assets/naturalist/models/block/goop_{level}.json" for level in (1, 2, 3)),
        *(f"assets/naturalist/textures/item/glow_goop_{level}.png" for level in (1, 2, 3)),
        "data/naturalist/loot_table/blocks/glow_goop.json",
        "data/naturalist/recipe/glow_goop.json",
        "data/naturalist/recipe/glow_goop_from_campfire_cooking.json",
        "data/naturalist/recipe/glow_goop_from_smoking.json",
        "data/naturalist/recipe/spectral_arrow_from_glow_goop.json",
        "data/naturalist/advancement/recipes/naturalist/glow_goop.json",
        "data/naturalist/advancement/recipes/naturalist/spectral_arrow_from_glow_goop.json",
    )
    with ZipFile(jar) as source:
        for entry in files:
            target = pack / entry
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(source.read(entry))
        lang = "assets/naturalist/lang/en_us.json"
        translation = json.loads((pack / lang).read_text())
        original = json.loads(source.read(lang))
        for key in ("item.naturalist.glow_goop", "naturalist.configuration.glow_goop_tooltip"):
            translation[key] = original[key]
        dump(pack, lang, translation)
    dump(pack, "data/naturalist/horizon/block/glow_goop.json", {
        "template": "minecraft:heavy_core",
        "components": {"minecraft:rarity": "common"},
        "properties": {
            "hardness": 0.5, "explosion_resistance": 0.5, "requires_tool": False,
            "sounds": "minecraft:honey_block", "render_shape": "INVISIBLE", "occludes": False,
        },
        "shapes": {"default": []},
        "collision_shapes": {"default": []},
        "state_properties": {"goop": {"type": "enum", "values": ["1", "2", "3"]}},
        "default_state": {"goop": "1", "waterlogged": "false"},
        "state_light": {"property": "goop", "values": {"1": 5, "2": 10, "3": 15}},
    })
