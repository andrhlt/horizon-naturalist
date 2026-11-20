"""Extract the Starfish entity, four placeable colors, loot, and spawn data."""
import json
from zipfile import ZipFile

from prepare import aquatic_item

COLORS = ("orange", "purple", "blue", "red")
BLOCKS = tuple(f"{color}_starfish" for color in COLORS)
ITEMS = {"starfish_bucket": "cod_bucket", "starfish_spawn_egg": "cod_spawn_egg"}


def pack_starfish(jar, pack, dump):
    with ZipFile(jar) as source:
        exact = {
            *(f"assets/naturalist/{folder}/{name}.json" for name in (*ITEMS, *BLOCKS)
              for folder in ("items", "models/item")),
            *(f"assets/naturalist/{folder}/{name}.json" for name in BLOCKS
              for folder in ("blockstates", "models/block")),
            *(f"assets/naturalist/textures/block/{name}.png" for name in BLOCKS),
            *(f"assets/naturalist/textures/item/bucket_{name}.png" for name in BLOCKS),
            *(f"data/naturalist/loot_table/blocks/{name}.json" for name in BLOCKS),
            "assets/naturalist/textures/item/starfish_spawn_egg.png",
            "data/naturalist/tags/worldgen/biome/has_starfish.json",
            "data/naturalist/tags/worldgen/biome/blacklist/blacklist_starfish.json",
        }
        prefixes = (
            "assets/naturalist/textures/entity/starfish/",
            "assets/naturalist/models/item/variant/starfish_bucket/",
            "data/naturalist/naturalist/starfish_variant/",
        )
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith(prefixes)):
                p = pack / name
                p.parent.mkdir(parents=True, exist_ok=True)
                p.write_bytes(source.read(name))
        locale = "assets/naturalist/lang/en_us.json"
        translated = json.loads(source.read(locale))
        current = json.loads((pack / locale).read_text())
        current.update({key: value for key, value in translated.items()
                        if key == "entity.naturalist.starfish"
                        or key.removeprefix("item.naturalist.") in ITEMS
                        or key.removeprefix("block.naturalist.") in BLOCKS})
        dump(pack, locale, current)
    for item, template in ITEMS.items():
        dump(pack, f"data/naturalist/horizon/item/{item}.json", aquatic_item(template))
    for block in BLOCKS:
        dump(pack, f"data/naturalist/horizon/block/{block}.json", {
            "template": "minecraft:resin_clump",
            "properties": {"hardness": 0.0, "light": 0, "sounds": "minecraft:seagrass", "occludes": False},
        })
    dump(pack, "data/naturalist/horizon/entity_type/starfish.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 4, "minecraft:movement_speed": 0.0},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/starfish.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_starfish",
        "spawners": {"type": "naturalist:starfish", "weight": 8, "minCount": 1, "maxCount": 2},
    })
    dump(pack, "data/naturalist/loot_table/entities/starfish.json", {"type": "minecraft:entity", "pools": []})
