"""Package assets and declarations for the butterfly lifecycle."""
import json
from zipfile import ZipFile

INSECTS = ("butterfly", "caterpillar")
VANILLA_CAUGHT = ("bee", "silverfish", "endermite")

SOUTH_SHAPES = (
    (6, 7, 11, 10, 12, 15),
    (5, 5, 9, 11, 12, 15),
    (4, 3, 7, 12, 12, 15),
    (4, 3, 7, 12, 12, 15),
)


def chrysalis_shapes():
    values = {}
    for age, (x0, y0, z0, x1, y1, z1) in enumerate(SOUTH_SHAPES):
        shapes = {
            "south": (x0, y0, z0, x1, y1, z1),
            "north": (x0, y0, 16 - z1, x1, y1, 16 - z0),
            "east": (z0, y0, x0, z1, y1, x1),
            "west": (16 - z1, y0, x0, 16 - z0, y1, x1),
        }
        for facing, bounds in shapes.items():
            values[f"{age},{facing}"] = [[v / 16 for v in bounds]]
    return {"properties": ["age", "facing"], "values": values}


def pack_insects(jar, pack, dump):
    names = (*INSECTS, "chrysalis")
    exact = {
        "assets/naturalist/blockstates/chrysalis.json",
        "assets/naturalist/items/capture_net.json",
        "assets/naturalist/models/item/capture_net.json",
        "assets/naturalist/particles/capture_net_swing.json",
        "assets/naturalist/textures/item/capture_net.png",
        "data/naturalist/recipe/capture_net.json",
        "data/naturalist/recipe/catch_bee.json",
        "data/naturalist/recipe/catch_cod.json",
        *[f"assets/naturalist/textures/item/{name}.png" for name in VANILLA_CAUGHT],
        "data/naturalist/advancement/recipes/naturalist/capture_net.json",
        "assets/naturalist/textures/entity/caterpillar.png",
        "data/naturalist/loot_table/blocks/chrysalis.json",
        "data/naturalist/tags/block/butterflies_spawnable_on.json",
        "data/naturalist/tags/worldgen/biome/has_butterfly.json",
        "data/naturalist/tags/worldgen/biome/blacklist/blacklist_butterfly.json",
        *[f"assets/naturalist/{folder}/{name}.json"
          for folder in ("items", "models/item")
          for name in (*names, *VANILLA_CAUGHT, "butterfly_spawn_egg", "caterpillar_spawn_egg")],
        *[f"data/naturalist/loot_table/entities/{name}.json" for name in INSECTS],
    }
    prefixes = (
        "assets/naturalist/models/block/chrysalis_stage",
        "assets/naturalist/textures/particle/capture_net_swing_",
        "assets/naturalist/textures/block/chrysalis_stage",
        "assets/naturalist/textures/entity/butterfly/",
        "assets/naturalist/sounds/mob/butterfly/",
        "assets/naturalist/sounds/mob/snail/",
        *[f"assets/naturalist/textures/item/{name}" for name in names],
        *[f"data/naturalist/naturalist/{name}_variant/" for name in INSECTS],
    )
    with ZipFile(jar) as source:
        for entry in source.namelist():
            if entry.endswith("/") or not (entry in exact or entry.startswith(prefixes)):
                continue
            file = pack / entry
            file.parent.mkdir(parents=True, exist_ok=True)
            file.write_bytes(source.read(entry))
        for file, select in (
            ("assets/naturalist/lang/en_us.json", lambda key: any(name in key for name in (*names, "capture_net"))
             or key in {f"item.naturalist.{name}" for name in VANILLA_CAUGHT}),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.butterfly.") or key == "entity.snail.forward"),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if select(key)})
            dump(pack, file, existing)

    for name, attributes, egg in (
        ("butterfly", {"max_health": 3, "movement_speed": 0.30000001192092896,
                       "flying_speed": 0.6000000238418579}, "bat_spawn_egg"),
        ("caterpillar", {"max_health": 3, "movement_speed": 0.10000000149011612}, "silverfish_spawn_egg"),
    ):
        dump(pack, f"data/naturalist/horizon/entity_type/{name}.json", {
            "template": "minecraft:wandering_trader",
            "attributes": {f"minecraft:{key}": value for key, value in attributes.items()},
        })
        dump(pack, f"data/naturalist/horizon/item/{name}_spawn_egg.json", {
            "template": f"minecraft:{egg}",
        })
        dump(pack, f"data/naturalist/horizon/item/{name}.json", {
            "template": "minecraft:stick", "max_stack_size": 1,
        })
    for name in ("scorpion", *VANILLA_CAUGHT):
        dump(pack, f"data/naturalist/horizon/item/{name}.json", {
            "template": "minecraft:stick", "max_stack_size": 1,
        })
    dump(pack, "data/naturalist/horizon/item/capture_net.json", {
        "template": "minecraft:shears", "max_stack_size": 1,
        "components": {"minecraft:max_damage": 64},
    })
    dump(pack, "data/naturalist/horizon/block/chrysalis.json", {
        "template": "minecraft:dirt",
        "properties": {"hardness": 0.2, "explosion_resistance": 3.0, "sounds": "minecraft:grass_block",
                       "occludes": False, "random_ticks": True},
        "collision_shapes": {"default": []},
        "shapes": chrysalis_shapes(),
        "state_properties": {
            "age": {"type": "enum", "values": ["0", "1", "2", "3"]},
            "facing": {"type": "enum", "values": ["north", "south", "west", "east"]},
        },
        "default_state": {"age": "0", "facing": "north"},
        "components": {"minecraft:max_stack_size": 1},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/butterfly.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_butterfly",
        "spawners": {"type": "naturalist:butterfly", "weight": 10, "minCount": 3, "maxCount": 6},
    })
