"""Extract Jellyfish assets and spawn data from the pinned Java release."""
import json
from zipfile import ZipFile

from prepare import aquatic_item

ITEMS = {"jellyfish_bucket": "cod_bucket", "jellyfish_spawn_egg": "cod_spawn_egg"}
COLORS = ("blue", "green", "orange", "pink", "white")


def pack_jellyfish(jar, pack, dump):
    with ZipFile(jar) as source:
        exact = {
            *[f"assets/naturalist/{folder}/{item}.json" for item in ITEMS for folder in ("items", "models/item")],
            "assets/naturalist/textures/item/jellyfish_spawn_egg.png",
            *[f"assets/naturalist/textures/item/bucket_{color}_jellyfish.png" for color in COLORS],
            "data/naturalist/loot_table/entities/jellyfish.json",
            "data/naturalist/tags/worldgen/biome/has_jellyfish.json",
            "data/naturalist/tags/worldgen/biome/blacklist/blacklist_jellyfish.json",
        }
        prefixes = (
            "assets/naturalist/textures/entity/jellyfish/",
            "assets/naturalist/models/item/variant/jellyfish_bucket/",
            "assets/naturalist/sounds/mob/jellyfish/",
            "data/naturalist/naturalist/jellyfish_variant/",
        )
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith(prefixes)):
                file = pack / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_bytes(source.read(name))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key == "entity.naturalist.jellyfish"
             or key.removeprefix("item.naturalist.") in ITEMS or key.startswith("naturalist.subtitles.entity.jellyfish.")),
            ("assets/naturalist/sounds.json", lambda key: key.startswith("entity.jellyfish.")),
        ):
            original = json.loads(source.read(file))
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in original.items() if selected(key)})
            dump(pack, file, existing)
    for item, template in ITEMS.items():
        dump(pack, f"data/naturalist/horizon/item/{item}.json", aquatic_item(template))
    dump(pack, "data/naturalist/horizon/entity_type/jellyfish.json", {
        "template": "minecraft:wandering_trader",
        "attributes": {"minecraft:max_health": 3, "minecraft:movement_speed": 0.5},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/jellyfish.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_jellyfish",
        "spawners": {"type": "naturalist:jellyfish", "weight": 10, "minCount": 1, "maxCount": 3},
    })
