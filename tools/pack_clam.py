"""Extract the pinned Clam family, food recipes, and biome modifiers."""
import json
from zipfile import ZipFile

ITEMS = {
    "clam_spawn_egg": "cod_spawn_egg",
    "clam_meat": "cod",
    "cooked_clam_meat": "cooked_cod",
}


def pack_clam(jar, pack, dump):
    net = "data/naturalist/horizon/item/capture_net.json"
    add_net = not (pack / net).is_file()
    items = set(ITEMS) | ({"capture_net"} if add_net else set())
    with ZipFile(jar) as source:
        exact = {
            *(f"assets/naturalist/{folder}/{item}.json" for item in items for folder in ("items", "models/item")),
            *(f"assets/naturalist/textures/item/{item}.png" for item in items),
            "data/naturalist/loot_table/entities/clam.json",
            "data/naturalist/tags/worldgen/biome/has_clam.json",
            "data/naturalist/tags/worldgen/biome/blacklist/blacklist_clam.json",
        }
        prefixes = (
            "assets/naturalist/textures/entity/clam/",
            "assets/naturalist/sounds/mob/clam/",
            "data/naturalist/naturalist/clam_variant/",
            "data/naturalist/recipe/cooked_clam_meat",
        )
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith(prefixes)):
                path = pack / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(source.read(name))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key == "entity.naturalist.clam"
             or key.removeprefix("item.naturalist.") in items or key.startswith("naturalist.subtitles.entity.clam.")),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.clam.")),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if selected(key)})
            dump(pack, file, existing)
    for item, template in ITEMS.items():
        dump(pack, f"data/naturalist/horizon/item/{item}.json", {"template": f"minecraft:{template}"})
    if add_net:
        dump(pack, net, {"template": "minecraft:shears", "max_stack_size": 1,
                         "components": {"minecraft:max_damage": 64}})
    dump(pack, "data/naturalist/horizon/entity_type/clam.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 30, "minecraft:movement_speed": 0, "minecraft:knockback_resistance": 1},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/clam.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_clam",
        "spawners": {"type": "naturalist:clam", "weight": 8, "minCount": 1, "maxCount": 1},
    })
