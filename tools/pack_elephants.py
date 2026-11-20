"""Extract the pinned Elephant and Mammoth items, entities, assets, and spawn tables."""
import json
from zipfile import ZipFile

SPECIES = {
    "elephant": (100, 12, 0.75, 1),
    "mammoth": (120, 14, 0.85, 2),
}


def pack_elephants(jar, pack, dump):
    items = ("elephant_spawn_egg", "mammoth_spawn_egg", "mammoth_meat", "cooked_mammoth_meat")
    exact = {
        *(f"assets/naturalist/{folder}/{item}.json" for item in items for folder in ("items", "models/item")),
        *(f"assets/naturalist/textures/item/{item}.png" for item in items),
        "assets/naturalist/textures/gui/container/elephant.png",
        "data/naturalist/advancement/recipes/food/cooked_mammoth_meat.json",
    }
    for species in SPECIES:
        exact.update({
            f"data/naturalist/loot_table/entities/{species}.json",
            f"data/naturalist/tags/worldgen/biome/has_{species}.json",
            f"data/naturalist/tags/worldgen/biome/blacklist/blacklist_{species}.json",
        })
    prefixes = ("assets/naturalist/textures/entity/elephant/", "assets/naturalist/textures/entity/mammoth/",
                "assets/naturalist/sounds/mob/elephant/", "assets/naturalist/sounds/mob/mammoth/",
                "data/naturalist/naturalist/elephant_variant/", "data/naturalist/naturalist/mammoth_variant/",
                "data/naturalist/recipe/cooked_mammoth_meat")
    with ZipFile(jar) as source:
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith(prefixes)):
                target = pack / name
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(source.read(name))
        for file, matches in (
            ("assets/naturalist/lang/en_us.json", lambda key: key.removeprefix("item.naturalist.") in items
             or key in ("entity.naturalist.mammoth", "entity.naturalist.elephant")
             or ".entity.elephant." in key or ".entity.mammoth." in key or key == "container.naturalist.elephant"),
            ("assets/naturalist/sounds.json", lambda key: key.startswith(("entity.elephant.", "entity.mammoth."))),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if matches(key)})
            dump(pack, file, existing)

    for menu in ("elephant", "elephant_unchested"):
        dump(pack, f"data/naturalist/horizon/menu/{menu}.json", {"slots": 27})

    for species, (health, attack, resistance, minimum) in SPECIES.items():
        dump(pack, f"data/naturalist/horizon/entity_type/{species}.json", {
            "template": "minecraft:wandering_trader",
            "attributes": {f"minecraft:{key}": value for key, value in {
                "max_health": health, "movement_speed": 0.2, "attack_damage": attack,
                "attack_knockback": 1.5, "knockback_resistance": resistance,
                "follow_range": 15, "step_height": 1,
            }.items()},
        })
        dump(pack, f"data/naturalist/horizon/item/{species}_spawn_egg.json",
            {"template": "minecraft:cow_spawn_egg"})
        dump(pack, f"data/naturalist/neoforge/biome_modifier/{species}.json", {
            "type": "neoforge:add_spawns", "biomes": f"#naturalist:has_{species}",
            "spawners": {"type": f"naturalist:{species}", "weight": 5, "minCount": minimum, "maxCount": 3},
        })

    for item, template, nutrition, saturation in (
        ("mammoth_meat", "beef", 3, 1.8000000715255737),
        ("cooked_mammoth_meat", "cooked_beef", 8, 12.800000190734863),
    ):
        dump(pack, f"data/naturalist/horizon/item/{item}.json", {
            "template": f"minecraft:{template}",
            "components": {"minecraft:food": {"nutrition": nutrition, "saturation": saturation}},
        })
