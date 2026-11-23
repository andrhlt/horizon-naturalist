"""Apply the Java biome inclusions and exclusions in the launcher data."""
import json
from pathlib import Path
from zipfile import ZipFile


# Naturalist's AddAnimalsBiomeModifier emits one standard modifier per row in
# this order. Horizon applies synthetic modifiers by resource ID, so retain the
# Java row order to preserve weighted spawn selection for a fixed RNG stream.
JAVA_SPAWN_ORDER = (
    "alligator", "anglerfish", "ray", "blobfish", "whale", "great_white_shark",
    "piranha", "bass", "bear", "black_bear", "bird_bluejay", "boar", "butterfly",
    "bird_canary", "bird_cardinal", "capybara", "catfish", "clam", "coral_snake",
    "crab", "deer", "desert_scorpion", "jungle_scorpion", "dragonfly", "duck",
    "elephant", "bird_finch", "firefly", "forest_foxes", "forest_rabbits", "giant_isopod",
    "hedgehog", "jellyfish", "giraffe", "hippo", "komodo_dragon", "lion", "lizard",
    "mammoth", "mole", "ostrich", "rat", "rhino", "bird_robin", "snail", "snake",
    "rattlesnake", "bird_sparrow", "starfish", "tiger", "tortoise", "turkey", "vulture",
    "zebra",
)


def preserve_java_spawn_order(pack):
    modifiers = pack / "data/naturalist/neoforge/biome_modifier"
    for index, name in enumerate(JAVA_SPAWN_ORDER):
        source = modifiers / f"{name}.json"
        if source.exists():
            source.rename(modifiers / f"{index:03}_{name}.json")


def pack_spawn_tables(jar, pack, dump):
    conventions = Path(__file__).with_name("biome_tags")
    for source in sorted(conventions.rglob("*.json")):
        destination = pack / "data/c/tags/worldgen/biome" / source.relative_to(conventions)
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(source.read_bytes())

    with ZipFile(jar) as original:
        for name in ("remove_farm_animals_savanna", "remove_farm_animals_swamp", "remove_forest_pigs"):
            destination = f"data/naturalist/neoforge/biome_modifier/{name}.json"
            (pack / destination).write_bytes(original.read(destination))

        for species, name, low, high in (
            ("fox", "forest_foxes", 1, 2),
            ("rabbit", "forest_rabbits", 2, 3),
        ):
            tag = f"data/naturalist/tags/worldgen/biome/blacklist/blacklist_{name}.json"
            (pack / tag).write_bytes(original.read(tag))
            dump(pack, f"data/naturalist/neoforge/biome_modifier/{name}.json", {
                "type": "neoforge:add_spawns",
                "biomes": {
                    "type": "neoforge:and",
                    "values": [
                        "#minecraft:is_forest",
                        {"type": "neoforge:not", "value": f"#naturalist:blacklist/blacklist_{name}"},
                    ],
                },
                "spawners": {"type": f"minecraft:{species}", "weight": 10, "minCount": low, "maxCount": high},
            })

    preserve_java_spawn_order(pack)

    tags = pack / "data/naturalist/tags/worldgen/biome/blacklist"
    for row in sorted((pack / "data/naturalist/neoforge/biome_modifier").glob("*.json")):
        modifier = json.loads(row.read_text())
        selection = modifier.get("biomes")
        prefix = "#naturalist:has_"
        if modifier.get("type") != "neoforge:add_spawns" or not isinstance(selection, str) or not selection.startswith(prefix):
            continue
        name = selection.removeprefix(prefix)
        if not (tags / f"blacklist_{name}.json").is_file():
            continue
        modifier["biomes"] = {
            "type": "neoforge:and",
            "values": [selection, {"type": "neoforge:not", "value": f"#naturalist:blacklist/blacklist_{name}"}],
        }
        dump(pack, row.relative_to(pack).as_posix(), modifier)
