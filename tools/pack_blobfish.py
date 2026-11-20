"""Extract the blobfish resources from the pinned Naturalist release."""
import json
from zipfile import ZipFile

from prepare import aquatic_item

ITEMS = {"blobfish": "cod", "cooked_blobfish": "cooked_cod", "blobfish_bucket": "cod_bucket",
         "blobfish_spawn_egg": "cod_spawn_egg"}


def pack_blobfish(jar, pack, dump):
    with ZipFile(jar) as source:
        exact = {
            *[f"assets/naturalist/{folder}/{item}{ext}" for item in ITEMS
              for folder, ext in [("items", ".json"), ("models/item", ".json"), ("textures/item", ".png")]],
            "assets/naturalist/textures/entity/blobfish/pink.png",
            "assets/naturalist/textures/entity/blobfish/gray.png",
            "assets/naturalist/textures/item/bucket_blobfish.png",
            "assets/naturalist/textures/item/bucket_content/blobfish.png",
            "data/naturalist/loot_table/entities/blobfish.json",
            "data/naturalist/tags/worldgen/biome/has_blobfish.json",
            "data/naturalist/tags/worldgen/biome/blacklist/blacklist_blobfish.json",
            *[f"data/naturalist/recipe/{name}.json" for name in
              ("cooked_blobfish", "cooked_blobfish_from_campfire_cooking", "cooked_blobfish_from_smoking")],
        }
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith((
                "data/naturalist/naturalist/blobfish_variant/", "assets/naturalist/sounds/mob/blobfish/",
            ))):
                file = pack / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_bytes(source.read(name))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key == "entity.naturalist.blobfish"
             or key.removeprefix("item.naturalist.") in ITEMS or key.startswith("tooltip.naturalist.blobfish_")
             or key.startswith("naturalist.subtitles.entity.blobfish.")),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.blobfish.")),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if selected(key)})
            dump(pack, file, existing)
    for item, template in ITEMS.items():
        definition = aquatic_item(template)
        food = {"blobfish": (2, 0.4000000059604645), "cooked_blobfish": (5, 6.0)}.get(item)
        if food:
            nutrition, saturation = food
            definition["components"] = {"minecraft:food": {"nutrition": nutrition, "saturation": saturation}}
        dump(pack, f"data/naturalist/horizon/item/{item}.json", definition)
    dump(pack, "data/naturalist/horizon/entity_type/blobfish.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 3, "minecraft:movement_speed": 1.0, "minecraft:attack_damage": 1},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/blobfish.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_blobfish",
        "spawners": {"type": "naturalist:blobfish", "weight": 6, "minCount": 1, "maxCount": 1},
    })
