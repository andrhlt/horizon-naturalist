"""Package Naturalist's Cooked Egg and recipes that use eggs or hide."""
import json
from zipfile import ZipFile


ASSETS = (
    "assets/naturalist/items/cooked_egg.json",
    "assets/naturalist/models/item/cooked_egg.json",
    "assets/naturalist/textures/item/cooked_egg.png",
    "data/naturalist/advancement/recipes/food/cooked_egg.json",
    "data/naturalist/recipe/cooked_egg.json",
    "data/naturalist/recipe/cooked_egg_from_campfire_cooking.json",
    "data/naturalist/recipe/cooked_egg_from_smoking.json",
    "data/naturalist/advancement/recipes/food/cake.json",
    "data/naturalist/recipe/cake.json",
    "data/naturalist/recipe/pumpkin_pie.json",
    "data/naturalist/recipe/leather_from_smelting_hide.json",
)


def pack_cooked_egg(jar, pack, dump):
    with ZipFile(jar) as source:
        for entry in ASSETS:
            output = pack / entry
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_bytes(source.read(entry))
        locale = "assets/naturalist/lang/en_us.json"
        translations = json.loads((pack / locale).read_text())
        key = "item.naturalist.cooked_egg"
        translations[key] = json.loads(source.read(locale))[key]
        dump(pack, locale, translations)
        common_eggs = "data/c/tags/item/eggs.json"
        path = pack / common_eggs
        tag = json.loads(path.read_text()) if path.is_file() else {"values": []}
        # NeoForge supplies the three vanilla eggs to this common tag.
        original = json.loads(source.read(common_eggs))["values"]
        for egg in (*original, "minecraft:blue_egg", "minecraft:brown_egg", "minecraft:egg"):
            if egg not in tag["values"]:
                tag["values"].append(egg)
        dump(pack, common_eggs, tag)
    dump(pack, "data/naturalist/horizon/item/cooked_egg.json", {
        "template": "minecraft:cooked_chicken",
        "components": {"minecraft:food": {"nutrition": 5, "saturation": 6.0}},
    })
