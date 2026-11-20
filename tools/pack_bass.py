"""Extract the bass resources from the pinned Naturalist release."""
import json
from zipfile import ZipFile

from prepare import aquatic_item

ITEMS = {"bass": "cod", "cooked_bass": "cooked_cod", "bass_bucket": "cod_bucket",
         "bass_spawn_egg": "cod_spawn_egg"}


def pack_bass(jar, pack, dump):
    with ZipFile(jar) as source:
        exact = {
            *[f"assets/naturalist/{folder}/{item}{ext}" for item in ITEMS
              for folder, ext in [("items", ".json"), ("models/item", ".json"), ("textures/item", ".png")]],
            *[f"assets/naturalist/textures/entity/{name}.png" for name in ("bass", "bass_medium", "bass_large")],
            "assets/naturalist/textures/item/bucket_bass.png",
            "assets/naturalist/textures/item/bucket_content/bass.png",
            "data/naturalist/loot_table/entities/bass.json",
            "data/naturalist/tags/worldgen/biome/has_bass.json",
            "data/naturalist/tags/worldgen/biome/blacklist/blacklist_bass.json",
            *[f"data/naturalist/recipe/{name}.json" for name in
              ("cooked_bass", "cooked_bass_from_campfire_cooking", "cooked_bass_from_smoking")],
        }
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith("data/naturalist/naturalist/bass_variant/")):
                file = pack / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_bytes(source.read(name))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key == "entity.naturalist.bass"
             or key.removeprefix("item.naturalist.") in ITEMS or key.startswith("tooltip.naturalist.bass_")
             or key.startswith("naturalist.subtitles.entity.bass.")),
            ("assets/naturalist/sounds.json", lambda key: key == "entity.bass.flop"),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if selected(key)})
            dump(pack, file, existing)
    for item, template in ITEMS.items():
        definition = aquatic_item(template)
        food = {"bass": (2, 0.4), "cooked_bass": (5, 6.0)}.get(item)
        if food:
            nutrition, saturation = food
            definition["components"] = {"minecraft:food": {"nutrition": nutrition, "saturation": saturation}}
        dump(pack, f"data/naturalist/horizon/item/{item}.json", definition)
    dump(pack, "data/naturalist/horizon/entity_type/bass.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 3, "minecraft:movement_speed": 0.7, "minecraft:attack_damage": 1},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/bass.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_bass",
        "spawners": {"type": "naturalist:bass", "weight": 10, "minCount": 3, "maxCount": 6},
    })
