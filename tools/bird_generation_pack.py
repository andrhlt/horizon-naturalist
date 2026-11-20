"""A taiga Bird override usable in both headless world-generation fixtures."""
import json


def write_pack(datapacks):
    folder = datapacks / "bird_gen_fixture"
    files = {
        "pack.mcmeta": {"pack": {"description": "Taiga Bird variant fixture",
                                "min_format": 88, "max_format": 107}},
        "data/bird_gen_fixture/tags/worldgen/biome/northern_woods.json": {
            "values": ["minecraft:taiga"],
        },
        "data/bird_gen_fixture/naturalist/bird_variant/taiga_bird.json": {
            "texture": "naturalist:entity/bird/cardinal",
            "biomes": "#bird_gen_fixture:northern_woods",
            "priority": 2,
            "weight": 1,
        },
    }
    for name, contents in files.items():
        path = folder / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(contents, indent=2) + "\n")
