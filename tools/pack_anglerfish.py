"""Extract Anglerfish assets and spawn data from the pinned Java release."""
import json
from zipfile import ZipFile

from prepare import aquatic_item

ITEMS = {"anglerfish_bucket": "cod_bucket", "anglerfish_spawn_egg": "cod_spawn_egg",
         "anglerfish": "cod", "cooked_anglerfish": "cooked_cod"}
FOOD = {"anglerfish": (2, 0.4), "cooked_anglerfish": (5, 6.0)}


def pack_anglerfish(jar, pack, dump):
    with ZipFile(jar) as source:
        exact = {
            *[f"assets/naturalist/{folder}/{item}.json" for item in ITEMS for folder in ("items", "models/item")],
            "assets/naturalist/textures/item/anglerfish_spawn_egg.png",
            *[f"assets/naturalist/textures/item/{item}.png" for item in
              ("bucket_anglerfish", "bucket_glow_anglerfish", "anglerfish", "cooked_anglerfish")],
            "data/naturalist/loot_table/entities/anglerfish.json",
            "data/naturalist/tags/worldgen/biome/has_anglerfish.json",
            "data/naturalist/tags/worldgen/biome/blacklist/blacklist_anglerfish.json",
        }
        prefixes = (
            "assets/naturalist/textures/entity/anglerfish/",
            "assets/naturalist/models/item/variant/anglerfish_bucket/",
            "assets/naturalist/sounds/mob/anglerfish/",
            "data/naturalist/naturalist/anglerfish_variant/",
            "data/naturalist/recipe/cooked_anglerfish",
        )
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith(prefixes)):
                file = pack / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_bytes(source.read(name))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key == "entity.naturalist.anglerfish"
             or key.removeprefix("item.naturalist.") in ITEMS or key.startswith("naturalist.subtitles.entity.anglerfish.")
             or key.startswith("tooltip.naturalist.anglerfish_")),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.anglerfish.")),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if selected(key)})
            dump(pack, file, existing)
    for item, template in ITEMS.items():
        definition = aquatic_item(template)
        if item in FOOD:
            nutrition, saturation = FOOD[item]
            definition["components"] = {"minecraft:food": {"nutrition": nutrition, "saturation": saturation}}
        dump(pack, f"data/naturalist/horizon/item/{item}.json", definition)
    dump(pack, "data/naturalist/horizon/entity_type/anglerfish.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 8, "minecraft:movement_speed": 1.2, "minecraft:attack_damage": 2},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/anglerfish.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_anglerfish",
        "spawners": {"type": "naturalist:anglerfish", "weight": 10, "minCount": 1, "maxCount": 1},
    })
