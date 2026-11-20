"""Package pinned dragonfly and firefly resources."""
import json
from zipfile import ZipFile

WINGS = ("dragonfly", "firefly")


def pack_wings(jar, pack, dump):
    exact = {
        "data/minecraft/tags/entity_type/frog_food.json",
        "data/naturalist/tags/block/dragonflies_spawnable_on.json",
        "data/naturalist/tags/block/fireflies_spawnable_on.json",
        *[f"data/naturalist/loot_table/entities/{name}.json" for name in WINGS],
        *[f"data/naturalist/tags/worldgen/biome/has_{name}.json" for name in WINGS],
        *[f"data/naturalist/tags/worldgen/biome/blacklist/blacklist_{name}.json" for name in WINGS],
        *[f"assets/naturalist/{folder}/{name}.json"
          for folder in ("items", "models/item")
          for name in (*WINGS, "dragonfly_spawn_egg", "firefly_spawn_egg")],
    }
    prefixes = (
        "assets/naturalist/models/item/variant/dragonfly_",
        "assets/naturalist/textures/entity/dragonfly/",
        "assets/naturalist/textures/entity/firefly",
        *[f"assets/naturalist/sounds/mob/{name}/" for name in WINGS],
        *[f"assets/naturalist/textures/item/{name}" for name in WINGS],
        *[f"data/naturalist/naturalist/{name}_variant/" for name in WINGS],
    )
    with ZipFile(jar) as source:
        for entry in source.namelist():
            if entry.endswith("/") or not (entry in exact or entry.startswith(prefixes)):
                continue
            file = pack / entry
            file.parent.mkdir(parents=True, exist_ok=True)
            file.write_bytes(source.read(entry))
        for file, select in (
            ("assets/naturalist/lang/en_us.json", lambda key: any(name in key for name in WINGS)),
            ("assets/naturalist/sounds.json", lambda key: any(key.startswith(f"entity.{n}.") for n in WINGS)),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if select(key)})
            dump(pack, file, existing)
        for event in json.loads(source.read("assets/naturalist/sounds.json"))["entity.dragonfly.loop"]["sounds"]:
            path = event if isinstance(event, str) else event["name"]
            namespace, name = path.split(":")
            entry = f"assets/{namespace}/sounds/{name}.ogg"
            if namespace == "naturalist" and entry in source.namelist():
                target = pack / entry
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(source.read(entry))
    for name, max_health in (("dragonfly", 6), ("firefly", 4)):
        attributes = {"minecraft:max_health": max_health}
        if name == "dragonfly":
            attributes["minecraft:movement_speed"] = 0.7
        else:
            attributes.update({"minecraft:movement_speed": 0.30000001192092896,
                               "minecraft:flying_speed": 0.6000000238418579})
        dump(pack, f"data/naturalist/horizon/entity_type/{name}.json", {
            "template": "minecraft:wandering_trader", "attributes": attributes,
        })
        dump(pack, f"data/naturalist/horizon/item/{name}_spawn_egg.json", {
            "template": "minecraft:parrot_spawn_egg",
        })
        dump(pack, f"data/naturalist/horizon/item/{name}.json", {
            "template": "minecraft:stick", "max_stack_size": 1,
        })
        dump(pack, f"data/naturalist/neoforge/biome_modifier/{name}.json", {
            "type": "neoforge:add_spawns", "biomes": f"#naturalist:has_{name}",
            "spawners": {"type": f"naturalist:{name}", "weight": 10, "minCount": 2, "maxCount": 4},
        })
    dump(pack, "data/minecraft/tags/entity_type/inverted_healing_and_harm.json", {
        "replace": False, "values": ["naturalist:dragonfly"],
    })
