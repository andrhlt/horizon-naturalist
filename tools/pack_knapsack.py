"""Package the pinned Knapsack assets and original Java recipe."""
import json
from zipfile import ZipFile


ASSETS = (
    "assets/naturalist/items/knapsack.json",
    "assets/naturalist/models/item/knapsack.json",
    "assets/naturalist/models/item/knapsack_filled.json",
    "assets/naturalist/textures/item/knapsack.png",
    "assets/naturalist/textures/item/empty_knapsack.png",
    "assets/naturalist/sounds/item/knapsack/place.ogg",
    "assets/naturalist/sounds/item/knapsack/pickup.ogg",
    "data/naturalist/recipe/knapsack.json",
    "data/naturalist/advancement/recipes/naturalist/knapsack.json",
)


def pack_knapsack(jar, pack, dump):
    with ZipFile(jar) as source:
        for name in ASSETS:
            entry = pack / name
            entry.parent.mkdir(parents=True, exist_ok=True)
            entry.write_bytes(source.read(name))
        for name, keys in (
            ("assets/naturalist/lang/en_us.json", (
                "item.naturalist.knapsack",
                "naturalist.subtitles.item.knapsack.pickup",
                "naturalist.subtitles.item.knapsack.place",
            )),
            ("assets/naturalist/sounds.json", ("item.knapsack.pickup", "item.knapsack.place")),
        ):
            resource = json.loads((pack / name).read_text())
            original = json.loads(source.read(name))
            resource.update({key: original[key] for key in keys})
            dump(pack, name, resource)
    dump(pack, "data/naturalist/horizon/item/knapsack.json", {
        "template": "minecraft:stick",
        "max_stack_size": 1,
    })
