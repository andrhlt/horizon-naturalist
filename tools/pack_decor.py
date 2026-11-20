"""Package the original Froglass and Shellstone blocks and their recipes."""
import json
from pathlib import PurePosixPath
from zipfile import ZipFile


FROGLASS = {
    f"{color}_froglass{suffix}": f"glass{suffix}"
    for color in ("azure", "verdant", "crimson")
    for suffix in ("", "_pane")
}
SHELLSTONE = {
    ("shellstone_bricks" if family == "shellstone_brick" and not shape else family + shape):
        "sandstone" + shape
    for family in ("shellstone", "shellstone_brick", "cut_shellstone", "smooth_shellstone")
    for shape in ("", "_slab", "_stairs", "_wall")
}
BLOCKS = FROGLASS | SHELLSTONE
TAGS = (
    "data/minecraft/tags/block/impermeable.json",
    "data/minecraft/tags/block/mineable/pickaxe.json",
    "data/naturalist/tags/block/shellstone.json",
    *(f"data/c/tags/{registry}/glass_{shape}.json"
      for registry in ("block", "item") for shape in ("blocks", "panes")),
    *(f"data/minecraft/tags/{registry}/{shape}.json"
      for registry in ("block", "item") for shape in ("slabs", "stairs", "walls")),
)


def pack_decor(jar, pack, dump):
    with ZipFile(jar) as source:
        directories = (
            "assets/naturalist/blockstates/", "assets/naturalist/items/",
            "assets/naturalist/models/block/", "assets/naturalist/models/item/",
            "assets/naturalist/textures/block/", "data/naturalist/loot_table/blocks/",
            "data/naturalist/recipe/", "data/naturalist/advancement/recipes/",
        )
        for entry in source.namelist():
            if entry.endswith("/") or not entry.startswith(directories):
                continue
            name = PurePosixPath(entry).stem
            if "froglass" not in name and "shellstone" not in name:
                continue
            output = pack / entry
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_bytes(source.read(entry))
        for entry in TAGS:
            incoming = json.loads(source.read(entry))
            output = pack / entry
            if output.is_file():
                existing = json.loads(output.read_text())
                existing["values"].extend(value for value in incoming["values"] if value not in existing["values"])
            else:
                existing = incoming
            dump(pack, entry, existing)
        locale = "assets/naturalist/lang/en_us.json"
        translation = json.loads((pack / locale).read_text())
        originals = json.loads(source.read(locale))
        translation.update({name: value for name, value in originals.items()
                            if name.removeprefix("block.naturalist.") in BLOCKS})
        dump(pack, locale, translation)

    for name, template in BLOCKS.items():
        dump(pack, f"data/naturalist/horizon/block/{name}.json", {"template": f"minecraft:{template}"})
