"""Copy Ostrich assets, tags and content registrations from the pinned Java mod."""
import json
from pathlib import PurePosixPath
from zipfile import ZipFile


def pack_ostrich(jar, pack, dump):
    with ZipFile(jar) as original:
        for name in original.namelist():
            path = PurePosixPath(name)
            if name.endswith("/") or not name.startswith(("assets/naturalist/", "data/naturalist/")):
                continue
            if not ("ostrich" in path.name or "ostrich" in path.parts or "ostrich_variant" in path.parts):
                continue
            if name.endswith(".json") and "neoforge/biome_modifier/" in name:
                continue
            destination = pack / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(original.read(name))
        filters = (
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.ostrich.")),
            ("assets/naturalist/lang/en_us.json", lambda key: "ostrich" in key),
        )
        for name, selected in filters:
            merged = json.loads((pack / name).read_text())
            merged.update({key: value for key, value in json.loads(original.read(name)).items() if selected(key)})
            dump(pack, name, merged)

    # The standard content loader accepts the neutral template. The native type
    # loader supplies the actual ridden attributes and the animal's dimensions.
    dump(pack, "data/naturalist/horizon/entity_type/ostrich.json", {
        "template": "minecraft:wandering_trader",
    })
    dump(pack, "data/naturalist/horizon/item/ostrich_spawn_egg.json", {
        "template": "minecraft:cow_spawn_egg",
    })
    shape = [[5 / 16, 0, 5 / 16, 11 / 16, 8 / 16, 11 / 16]]
    dump(pack, "data/naturalist/horizon/block/ostrich_egg.json", {
        "template": "minecraft:turtle_egg",
        "collision_shapes": {"default": shape},
        "shapes": {"default": shape},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/ostrich.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_ostrich",
        "spawners": {"type": "naturalist:ostrich", "weight": 28, "minCount": 1, "maxCount": 2},
    })
    for name in ("data/naturalist/tags/item/eggs.json", "data/c/tags/item/eggs.json"):
        target = pack / name
        tag = json.loads(target.read_text()) if target.is_file() else {"values": []}
        tag["values"].append("naturalist:ostrich_egg")
        dump(pack, name, tag)
