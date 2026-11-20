"""Extract the pinned Java Alligator, egg assets, and spawn lists."""
import json
from pathlib import Path
from zipfile import ZipFile


def pack_alligator(jar, pack, dump):
    groups = (
        "assets/naturalist/textures/entity/alligator/",
        "assets/naturalist/sounds/mob/alligator/",
        "data/naturalist/naturalist/alligator_variant/",
    )
    with ZipFile(jar) as source:
        for name in source.namelist():
            if not name.endswith("/") and (
                name.startswith(groups)
                or name == "data/naturalist/tags/entity_type/safe_egg_walkers.json"
                or ("alligator" in Path(name).name and name.startswith(("assets/", "data/")))
            ):
                path = pack / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(source.read(name))
        for name in ("lang/en_us", "sounds"):
            path = f"assets/naturalist/{name}.json"
            old = json.loads((pack / path).read_text())
            old.update({key: val for key, val in json.loads(source.read(path)).items() if "alligator" in key})
            dump(pack, path, old)

    for tag, entry in (
        ("minecraft/tags/entity_type/can_breathe_under_water", "naturalist:alligator"),
        ("naturalist/tags/item/eggs", "naturalist:alligator_egg"),
        ("c/tags/item/eggs", "naturalist:alligator_egg"),
        ("c/tags/item/animal_foods", "#naturalist:alligator_food_items"),
    ):
        name = f"data/{tag}.json"
        path = pack / name
        data = json.loads(path.read_text()) if path.is_file() else {"values": []}
        if entry not in data["values"]:
            data["values"].append(entry)
        dump(pack, name, data)
    dump(pack, "data/naturalist/horizon/entity_type/alligator.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {
            "minecraft:max_health": 30, "minecraft:movement_speed": 0.2,
            "minecraft:attack_damage": 8, "minecraft:knockback_resistance": 0.6,
            "minecraft:step_height": 1,
        },
    })
    dump(pack, "data/naturalist/horizon/item/alligator_spawn_egg.json", {"template": "minecraft:cow_spawn_egg"})
    dump(pack, "data/naturalist/horizon/block/alligator_egg.json", {"template": "minecraft:turtle_egg"})
    dump(pack, "data/naturalist/neoforge/biome_modifier/alligator.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_alligator",
        "spawners": {"type": "naturalist:alligator", "weight": 10, "minCount": 2, "maxCount": 3},
    })
