"""Extract the catfish resources from the pinned Naturalist release."""
import json
from zipfile import ZipFile

from prepare import aquatic_item

ITEMS = {"catfish": "cod", "cooked_catfish": "cooked_cod", "catfish_bucket": "cod_bucket",
         "catfish_spawn_egg": "cod_spawn_egg"}


def pack_catfish(jar, pack, dump):
    with ZipFile(jar) as source:
        exact = {
            *[f"assets/naturalist/{folder}/{item}{ext}" for item in ITEMS
              for folder, ext in [("items", ".json"), ("models/item", ".json"), ("textures/item", ".png")]],
            *[f"assets/naturalist/textures/entity/{name}.png" for name in ("catfish",)],
            "assets/naturalist/textures/item/bucket_catfish.png",
            "assets/naturalist/textures/item/bucket_content/catfish.png",
            "data/naturalist/loot_table/entities/catfish.json",
            "data/naturalist/tags/entity_type/catfish_hostiles.json",
            "data/naturalist/tags/worldgen/biome/has_catfish.json",
            "data/naturalist/tags/worldgen/biome/blacklist/blacklist_catfish.json",
            *[f"data/naturalist/recipe/{name}.json" for name in
              ("cooked_catfish", "cooked_catfish_from_campfire_cooking", "cooked_catfish_from_smoking")],
        }
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith("data/naturalist/naturalist/catfish_variant/")):
                file = pack / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_bytes(source.read(name))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key == "entity.naturalist.catfish"
             or key.removeprefix("item.naturalist.") in ITEMS or key.startswith("tooltip.naturalist.catfish_")
             or key.startswith("naturalist.subtitles.entity.catfish.")),
            ("assets/naturalist/sounds.json", lambda key: key == "entity.catfish.flop"),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if selected(key)})
            dump(pack, file, existing)
    for item, template in ITEMS.items():
        definition = aquatic_item(template)
        food = {"catfish": (2, 0.4000000059604645), "cooked_catfish": (6, 9.600000381469727)}.get(item)
        if food:
            nutrition, saturation = food
            definition["components"] = {"minecraft:food": {"nutrition": nutrition, "saturation": saturation}}
        dump(pack, f"data/naturalist/horizon/item/{item}.json", definition)
    dump(pack, "data/naturalist/horizon/entity_type/catfish.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 6, "minecraft:movement_speed": 0.7, "minecraft:attack_damage": 1},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/catfish.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_catfish",
        "spawners": {"type": "naturalist:catfish", "weight": 10, "minCount": 1, "maxCount": 2},
    })
