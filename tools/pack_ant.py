"""Extract the pinned Java Ant and Ant Hill content into the Horizon pack."""
import json
import gzip
import random
import struct
from zipfile import ZipFile

ANT_ITEMS = {
    "ant": "stick",
    "queen_ant": "stick",
    "ant_spawn_egg": "cow_spawn_egg",
}

SMALL_LAYOUT = (
    (0, 0, -1), (-1, 0, 0), (0, 0, 0), (1, 0, 0), (-1, 0, 1),
    (0, 0, 1), (-1, 1, 0), (0, 1, 0), (0, 1, 1), (0, 2, 0),
)
BIG_LAYOUT = (
    (-1, 0, -1), (0, 0, -1), (-1, 0, 0), (0, 0, 0), (1, 0, 0),
    (-1, 0, 1), (0, 0, 1), (1, 0, 1), (-1, 1, 0), (0, 1, 0),
    (1, 1, 0), (0, 1, 1), (0, 2, 0), (0, 2, 1), (0, 3, 0),
)


def _nbt_name(name):
    value = name.encode("utf-8")
    return struct.pack(">H", len(value)) + value


def _nbt_tag(kind, name, payload):
    return bytes((kind,)) + _nbt_name(name) + payload


def _nbt_compound(entries):
    return b"".join(_nbt_tag(kind, name, payload) for kind, name, payload in entries) + b"\0"


def _nbt_list(kind, values):
    return bytes((kind,)) + struct.pack(">i", len(values)) + b"".join(values)


def _nbt_int_list(values):
    return _nbt_list(3, [struct.pack(">i", value) for value in values])


def _state_nbt(open_, workers, queen):
    values = (str(open_).lower(), str(workers), str(queen).lower())
    properties = _nbt_compound([
        (8, key, struct.pack(">H", len(value.encode())) + value.encode())
        for key, value in zip(("open", "workers", "has_queen"), values)
    ])
    return _nbt_compound([
        (8, "Name", struct.pack(">H", len(b"naturalist:ant_hill")) + b"naturalist:ant_hill"),
        (10, "Properties", properties),
    ])


def _encode_template(layout, seed):
    """Build 12 deterministic structure variants with 4–6 functional entrances."""
    height = max(y for _, y, _ in layout) + 1
    palette_states = [_state_nbt(False, 0, False), _state_nbt(True, 0, False),
                      _state_nbt(True, 1, False), _state_nbt(True, 2, False),
                      _state_nbt(True, 0, True)]
    templates = []
    for variant in range(12):
        count = 4 + variant % 3
        openings = set(random.Random(seed + variant).sample(range(len(layout)), count))
        queen = variant % count
        open_order = [i for i in range(len(layout)) if i in openings]
        queen_cell = open_order[queen]
        states = []
        for index in range(len(layout)):
            if index not in openings:
                states.append(0)
            elif index == queen_cell:
                states.append(4)
            else:
                states.append(1 + ((variant + index) % 3))
        templates.append(_encode_single_template(layout, height, palette_states, states))
    return templates


def _rotated_footprint(layout):
    """All cells any vanilla template rotation may place around the origin."""
    return sorted({
        rotated
        for x, y, z in layout
        for rotated in ((x, y, z), (-z, y, x), (-x, y, -z), (z, y, -x))
    })


def _encode_single_template(layout, height, palette_states, block_states):
    blocks = []
    for (x, y, z), state in zip(layout, block_states):
        blocks.append(_nbt_compound([
            (9, "pos", _nbt_int_list((x + 1, y, z + 1))),
            (3, "state", struct.pack(">i", state)),
            (10, "nbt", _nbt_compound([
                (8, "id", struct.pack(">H", len(b"naturalist:ant_hill")) + b"naturalist:ant_hill"),
                (9, "Storage", _nbt_list(0, [])),
            ])),
        ]))
    root = _nbt_compound([
        (9, "size", _nbt_int_list((3, height, 3))),
        (9, "blocks", _nbt_list(10, blocks)),
        (9, "palette", _nbt_list(10, palette_states)),
    ])
    return gzip.compress(bytes((10,)) + _nbt_name("") + root, mtime=0)


def _pack_hill_feature(size, layout, pack, dump):
    feature_id = f"naturalist:ant_hill_{size}"
    templates = _encode_template(layout, 0xA17 + len(layout) * 31)
    choices = []
    for variant, nbt in enumerate(templates):
        template_id = f"ant_hill_{size}_{variant}"
        path = pack / f"data/naturalist/structure/{template_id}.nbt"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(nbt)
        choices.append({"weight": 1, "data": {"id": f"naturalist:{template_id}"}})
    dump(pack, f"data/naturalist/worldgen/feature/{feature_id.split(':')[1]}.json", {
        "type": "minecraft:template",
        "templates": choices,
    })

    placed_path = pack / f"data/naturalist/worldgen/placed_feature/{feature_id.split(':')[1]}.json"
    placed = json.loads(placed_path.read_text())
    cells = [{
        "type": "minecraft:replaceable",
        "offset": list(position),
    } for position in _rotated_footprint(layout)]
    predicate = {
        "type": "minecraft:all_of",
        "predicates": [
            {
                "type": "minecraft:matching_block_tag",
                "tag": "minecraft:substrate_overworld",
                "offset": [0, -1, 0],
            },
            *cells,
        ],
    }
    heightmap = next(i for i, modifier in enumerate(placed["placement"])
                     if modifier.get("type") == "minecraft:heightmap")
    placed["placement"].insert(heightmap + 1, {
        "type": "minecraft:block_predicate_filter",
        "predicate": predicate,
    })
    dump(pack, f"data/naturalist/worldgen/placed_feature/{feature_id.split(':')[1]}.json", placed)


def pack_ant(jar, pack, dump):
    items = (*ANT_ITEMS, "ant_hill")
    exact = {
        *(f"assets/naturalist/{folder}/{item}.json" for item in items for folder in ("items", "models/item")),
        *(f"assets/naturalist/textures/item/{item}.png" for item in ANT_ITEMS),
        "assets/naturalist/textures/entity/ant.png",
        "assets/naturalist/blockstates/ant_hill.json",
        "assets/naturalist/models/block/ant_hill.json",
        "assets/naturalist/models/block/ant_hill_open.json",
        "assets/naturalist/textures/block/ant_hill.png",
        "assets/naturalist/textures/block/ant_hill_open.png",
        "data/naturalist/loot_table/entities/ant.json",
        "data/naturalist/loot_table/blocks/ant_hill.json",
        "data/naturalist/tags/worldgen/biome/has_ant_hill.json",
        "data/naturalist/neoforge/biome_modifier/add_ant_hills.json",
        "data/naturalist/worldgen/placed_feature/ant_hill_small.json",
        "data/naturalist/worldgen/placed_feature/ant_hill_big.json",
    }
    prefixes = (
        "assets/naturalist/sounds/mob/ant/",
        "data/naturalist/naturalist/ant_variant/",
    )
    with ZipFile(jar) as source:
        for entry in source.namelist():
            if entry.endswith("/") or (entry not in exact and not entry.startswith(prefixes)):
                continue
            path = pack / entry
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(source.read(entry))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key in {
                "entity.naturalist.ant", "block.naturalist.ant_hill",
                "entity.naturalist.carried_food", *(f"item.naturalist.{item}" for item in ANT_ITEMS),
            } or key.startswith("naturalist.subtitles.entity.ant.")),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.ant.")),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if selected(key)})
            dump(pack, file, existing)
    # Horizon supports vanilla template features and structure templates. Keep
    # Naturalist's existing biome/rareness/surface placement chain, but express
    # the custom hill geometry as data-pack templates with a substrate/space filter.
    _pack_hill_feature("small", SMALL_LAYOUT, pack, dump)
    _pack_hill_feature("big", BIG_LAYOUT, pack, dump)
    for item, template in ANT_ITEMS.items():
        definition = {"template": f"minecraft:{template}"}
        if item != "ant_spawn_egg":
            definition["max_stack_size"] = 1
        dump(pack, f"data/naturalist/horizon/item/{item}.json", definition)
    dump(pack, "data/naturalist/horizon/entity_type/ant.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {
            "minecraft:max_health": 6.0,
            "minecraft:movement_speed": 0.2,
            "minecraft:attack_damage": 2.0,
            "minecraft:follow_range": 20.0,
        },
        "sounds": {
            "ambient": "naturalist:entity.ant.ambient",
            "hurt": "naturalist:entity.ant.hurt",
            "death": "naturalist:entity.ant.death",
            "step": "naturalist:entity.ant.step",
        },
    })
    dump(pack, "data/naturalist/horizon/entity_type/carried_food.json", {
        "template": "minecraft:wandering_trader",
    })
    dump(pack, "data/naturalist/horizon/block/ant_hill.json", {
        "template": "minecraft:rooted_dirt",
        "properties": {
            "hardness": 0.5,
            "explosion_resistance": 0.2,
            "sounds": "minecraft:rooted_dirt",
            "random_ticks": {"property": "workers", "values": ["1", "2", "3"]},
        },
        "state_properties": {
            "open": {"type": "enum", "values": ["false", "true"]},
            "workers": {"type": "enum", "values": ["0", "1", "2", "3"]},
            "has_queen": {"type": "enum", "values": ["false", "true"]},
        },
        "default_state": {"open": "false", "workers": "0", "has_queen": "false"},
    })
    tag = "data/minecraft/tags/entity_type/arthropod.json"
    existing = json.loads((pack / tag).read_text()) if (pack / tag).is_file() else {"values": []}
    existing["values"].append("naturalist:ant")
    dump(pack, tag, existing)
