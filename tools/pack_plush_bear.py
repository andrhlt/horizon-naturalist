"""Package the pinned original Plush Bear assets and block definition."""
import json
from zipfile import ZipFile


ASSETS = (
    "assets/naturalist/blockstates/plush_bear.json",
    "assets/naturalist/items/plush_bear.json",
    "assets/naturalist/models/block/plush_bear.json",
    "assets/naturalist/models/item/plush_bear.json",
    "assets/naturalist/textures/block/plush_bear.png",
    "assets/naturalist/textures/item/plush_bear.png",
    "data/naturalist/advancement/recipes/naturalist/plush_bear.json",
    "data/naturalist/loot_table/blocks/plush_bear.json",
    "data/naturalist/recipe/plush_bear.json",
)
FACING = ("north", "east", "south", "west")


def pack_plush_bear(jar, pack, dump):
    with ZipFile(jar) as source:
        for entry in ASSETS:
            output = pack / entry
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_bytes(source.read(entry))
        locale = "assets/naturalist/lang/en_us.json"
        translations = json.loads((pack / locale).read_text())
        key = "block.naturalist.plush_bear"
        translations[key] = json.loads(source.read(locale))[key]
        dump(pack, locale, translations)

    x_axis = [3, 0, 2, 13, 15, 14]
    z_axis = [2, 0, 3, 14, 15, 13]
    shape = {"property": "facing", "values": {
        facing: [[coordinate / 16 for coordinate in (x_axis if facing in ("east", "west") else z_axis)]]
        for facing in FACING
    }}
    dump(pack, "data/naturalist/horizon/block/plush_bear.json", {
        "template": "minecraft:stone",
        "properties": {"hardness": 0.8, "explosion_resistance": 0.8, "occludes": False,
                       "requires_tool": False, "sounds": "minecraft:white_wool", "map_color": 0},
        "shapes": shape,
        "collision_shapes": shape,
        "state_properties": {"facing": {"type": "enum", "values": list(FACING)}},
        "default_state": {"facing": "north"},
    })
