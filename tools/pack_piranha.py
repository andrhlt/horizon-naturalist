"""Extract the piranha resources from the pinned Naturalist release."""
import json
from zipfile import ZipFile

from prepare import aquatic_item

ITEMS = {"piranha": "cod", "cooked_piranha": "cooked_cod", "piranha_bucket": "cod_bucket",
         "piranha_spawn_egg": "cod_spawn_egg"}


def pack_piranha(jar, pack, dump):
    with ZipFile(jar) as source:
        exact = {
            *[f"assets/naturalist/{folder}/{item}{ext}" for item in ITEMS
              for folder, ext in [("items", ".json"), ("models/item", ".json"), ("textures/item", ".png")]],
            *[f"assets/naturalist/textures/entity/{name}.png" for name in ("piranha",)],
            "assets/naturalist/textures/item/bucket_piranha.png",
            "assets/naturalist/textures/item/bucket_content/piranha.png",
            "data/naturalist/loot_table/entities/piranha.json",
            "data/naturalist/tags/entity_type/piranha_hostiles.json",
            "data/naturalist/tags/worldgen/biome/has_piranha.json",
            "data/naturalist/tags/worldgen/biome/blacklist/blacklist_piranha.json",
            *[f"data/naturalist/recipe/{name}.json" for name in
              ("cooked_piranha", "cooked_piranha_from_campfire_cooking", "cooked_piranha_from_smoking")],
        }
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith((
                "data/naturalist/naturalist/piranha_variant/", "assets/naturalist/sounds/mob/piranha/",
            ))):
                file = pack / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_bytes(source.read(name))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key == "entity.naturalist.piranha"
             or key.removeprefix("item.naturalist.") in ITEMS or key.startswith("tooltip.naturalist.piranha_")
             or key.startswith("naturalist.subtitles.entity.piranha.")),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.piranha.")),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if selected(key)})
            dump(pack, file, existing)
    for item, template in ITEMS.items():
        definition = aquatic_item(template)
        food = {"piranha": (2, 0.4000000059604645), "cooked_piranha": (5, 6.0)}.get(item)
        if food:
            nutrition, saturation = food
            definition["components"] = {"minecraft:food": {"nutrition": nutrition, "saturation": saturation}}
        dump(pack, f"data/naturalist/horizon/item/{item}.json", definition)
    dump(pack, "data/naturalist/horizon/entity_type/piranha.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 6, "minecraft:movement_speed": 1.2, "minecraft:attack_damage": 2},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/piranha.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_piranha",
        "spawners": {"type": "naturalist:piranha", "weight": 8, "minCount": 2, "maxCount": 4},
    })
