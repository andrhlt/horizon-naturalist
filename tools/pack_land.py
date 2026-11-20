"""Copy boar and turkey resources from the pinned Java release."""
import json
from pathlib import Path
from zipfile import ZipFile

ANIMALS = {
    "mole": ("cow", 10, 1, 2, {"max_health": 8, "movement_speed": 0.2}),
    "dirt_trail": ("cow", 0, 0, 0, {"max_health": 1, "movement_speed": 0}),
    "rat": ("cow", 10, 1, 3, {"max_health": 6, "movement_speed": 0.2, "follow_range": 16}),
    "snake": ("cow", 10, 1, 1, {"max_health": 12, "movement_speed": 0.18,
                                  "attack_damage": 3, "follow_range": 20}),
    "lion": ("cow", 5, 3, 5, {"max_health": 30, "movement_speed": 0.25,
                                "attack_damage": 7, "follow_range": 32, "step_height": 1}),
    "tiger": ("cow", 8, 1, 1, {"max_health": 32, "movement_speed": 0.25,
                                 "attack_damage": 8, "follow_range": 32, "step_height": 1}),
    "tortoise": ("cow", 10, 1, 1, {"max_health": 30, "movement_speed": 0.17000000178813934,
                                    "attack_damage": 2, "knockback_resistance": 0.6}),
    "zebra": ("donkey", 1, 2, 6, {"max_health": 53, "movement_speed": 0.22499999403953552,
                                  "jump_strength": 0.7, "step_height": 1, "follow_range": 16}),
    "bear": ("cow", 10, 1, 2, {"max_health": 30, "movement_speed": 0.25, "attack_damage": 6,
                              "knockback_resistance": 0.6, "follow_range": 20, "step_height": 1}),
    "black_bear": ("cow", 10, 1, 2, {"max_health": 24, "movement_speed": 0.25, "attack_damage": 5,
                                    "knockback_resistance": 0.5, "follow_range": 20, "step_height": 1}),
    "rhino": ("cow", 1, 1, 3, {"max_health": 60, "movement_speed": 0.2, "attack_damage": 10,
                              "attack_knockback": 1.5, "knockback_resistance": 0.75,
                              "follow_range": 16, "step_height": 1}),
    "komodo_dragon": ("cow", 20, 1, 2,
                      {"max_health": 30, "movement_speed": 0.2, "attack_damage": 5, "step_height": 1}),
    "lizard": ("cow", 10, 1, 1, {"max_health": 8, "movement_speed": 0.30000001192092896, "attack_damage": 2}),
    "lizard_tail": ("cow", 0, 0, 0, {"max_health": 6, "movement_speed": 0.7}),
    "desert_scorpion": ("cow", 20, 1, 3, {"max_health": 12, "movement_speed": 0.27, "attack_damage": 2}),
    "jungle_scorpion": ("cow", 30, 1, 3, {"max_health": 16, "movement_speed": 0.27, "attack_damage": 3}),
    "boar": ("pig", 10, 3, 4, {"max_health": 24, "movement_speed": 0.25, "attack_damage": 4,
                              "attack_knockback": 1, "knockback_resistance": 0.5, "follow_range": 16}),
    "capybara": ("cow", 10, 2, 4, {"max_health": 14, "movement_speed": 0.2}),
    "turkey": ("chicken", 17, 1, 2, {"max_health": 6, "movement_speed": 0.25, "attack_damage": 2}),
}
ITEMS = {"rat": "rabbit_foot", "tooth": "stick", "fat": "stick", "hide": "leather", "drumstick": "cooked_chicken", "cooked_drumstick": "cooked_chicken",
         "music_disc_death_by_hogs": "music_disc_13", "music_disc_wild_ones": "music_disc_13",
         "scorpion_poison_gland": "stick", "lizard_tail": "cooked_chicken",
         "cooked_lizard_tail": "cooked_chicken", "morsel": "cooked_chicken",
         "clam_meat": "cooked_chicken", "crab_meat": "cooked_chicken",
         "cooked_clam_meat": "cooked_chicken", "cooked_crab_meat": "cooked_chicken",
         "mammoth_meat": "cooked_chicken", "cooked_mammoth_meat": "cooked_beef",
         "fur": "leather", "bushmeat": "cooked_chicken", "cooked_bushmeat": "cooked_beef"}
FOODS = {
    "clam_meat": (2, 0.4000000059604645),
    "crab_meat": (2, 0.4000000059604645),
    "cooked_clam_meat": (5, 6.0),
    "cooked_crab_meat": (5, 6.0),
    "mammoth_meat": (3, 1.8000000715255737),
    "cooked_mammoth_meat": (8, 12.800000190734863),
    "morsel": (2, 1.2000000476837158),
    "cooked_lizard_tail": (5, 6.0),
    "lizard_tail": (2, 3.200000047683716),
    "drumstick": (3, 1.8000000715255737),
    "cooked_drumstick": (6, 7.200000286102295),
    "bushmeat": (3, 1.8000000715255737),
    "cooked_bushmeat": (8, 12.800000190734863),
}


def pack_land(jar, pack, dump):
    names = {"scorpion", "tortoise_egg"} | set(ITEMS) | {f"{kind}_spawn_egg" for kind, spec in ANIMALS.items() if spec[1]}
    exact = {
        f"assets/naturalist/{folder}/{name}.{ext}"
        for name in names
        for folder, ext in (("items", "json"), ("models/item", "json"), ("textures/item", "png"))
    }
    exact.update({
        "assets/naturalist/textures/entity/mole_sand.png",
        "data/naturalist/tags/item/rat_food.json",
        "assets/naturalist/textures/item/rat_black.png",
        "assets/naturalist/textures/item/rat_brown.png",
        "assets/naturalist/textures/item/rat_white.png",
        "assets/naturalist/models/item/variant/rat_black.json",
        "assets/naturalist/models/item/variant/rat_brown.json",
        "assets/naturalist/models/item/variant/rat_white.json",
        "assets/naturalist/textures/item/snake_green_spawn_egg.png",
        "data/naturalist/tags/item/snake_tame_items.json",
        "data/naturalist/tags/item/snake_tempt_items.json",
        "data/naturalist/tags/entity_type/snake_hostiles.json",
        "data/naturalist/tags/worldgen/biome/has_coral_snake.json",
        "data/naturalist/tags/worldgen/biome/has_rattlesnake.json",
        "data/naturalist/tags/worldgen/biome/blacklist/blacklist_coral_snake.json",
        "data/naturalist/tags/worldgen/biome/blacklist/blacklist_rattlesnake.json",
        "assets/naturalist/textures/item/tortoise_brown_spawn_egg.png",
        "data/naturalist/tags/worldgen/biome/variant/tortoise/black.json",
        "data/naturalist/tags/block/komodo_dragons_spawnable_on.json",
        "data/naturalist/tags/block/rhino_charge_breakable.json",
        "data/naturalist/tags/entity_type/bear_hostiles.json",
        "data/naturalist/tags/item/bear_tempt_items.json",
        "data/naturalist/tags/item/bear_furs.json",
        "data/naturalist/tags/entity_type/scorpion_hostiles.json",
        "data/naturalist/tags/item/lizard_tempt_items.json",
        "data/naturalist/recipe/cooked_lizard_tail.json",
        "data/naturalist/recipe/cooked_lizard_tail_from_campfire_cooking.json",
        "data/naturalist/recipe/cooked_lizard_tail_from_smoking.json",
        "data/naturalist/tags/block/scorpions_spawnable_on.json",
        "assets/naturalist/textures/entity/boar_baby.png",
        "data/naturalist/tags/block/tortoise_egg_layable_on.json",
        "data/naturalist/tags/item/tortoise_tempt_items.json",
        "data/naturalist/tags/entity_type/safe_egg_walkers.json",
        "assets/naturalist/textures/entity/zebra_baby.png",
        "assets/naturalist/sounds/records/death_by_hogs.ogg",
        "assets/naturalist/sounds/records/wild_ones.ogg",
        "data/naturalist/jukebox_song/death_by_hogs.json",
        "data/naturalist/jukebox_song/wild_ones.json",
        "data/c/tags/item/music_discs.json",
        "data/naturalist/recipe/cooked_drumstick.json",
        "data/naturalist/recipe/cooked_drumstick_from_campfire_cooking.json",
        "data/naturalist/recipe/cooked_drumstick_from_smoking.json",
        "data/naturalist/recipe/cooked_bushmeat.json",
        "data/naturalist/recipe/cooked_bushmeat_from_campfire_cooking.json",
        "data/naturalist/recipe/cooked_bushmeat_from_smoking.json",
    })
    exact.update(f"data/naturalist/recipe/cooked_{kind}.json"
                 for meat in ("clam_meat", "crab_meat", "mammoth_meat")
                 for kind in (meat, f"{meat}_from_campfire_cooking", f"{meat}_from_smoking"))
    prefixes = ["assets/naturalist/textures/entity/rat/", "assets/naturalist/textures/entity/capybara/", "assets/naturalist/textures/entity/scorpion/",
                "assets/naturalist/sounds/mob/scorpion/", "assets/naturalist/textures/entity/lizard/",
                "assets/naturalist/textures/entity/bear/", "assets/naturalist/textures/entity/black_bear/",
                "assets/naturalist/textures/entity/tortoise/",
                "assets/naturalist/textures/entity/tiger/", "assets/naturalist/textures/entity/lion/", "assets/naturalist/textures/entity/snake/",
                "data/naturalist/tags/worldgen/biome/variant/tiger/"]
    for kind, (template, weight, min_count, max_count, attributes) in ANIMALS.items():
        exact.update({
            f"assets/naturalist/textures/entity/{kind}.png",
            f"data/naturalist/loot_table/entities/{kind}.json",
            f"data/naturalist/tags/item/{kind}_food_items.json",
            f"data/naturalist/tags/entity_type/{kind}_hostiles.json",
            f"data/naturalist/tags/worldgen/biome/has_{kind}.json",
            f"data/naturalist/tags/worldgen/biome/blacklist/blacklist_{kind}.json",
        })
        prefixes.extend((f"assets/naturalist/sounds/mob/{kind}/", f"data/naturalist/naturalist/{kind}_variant/"))
        dump(pack, f"data/naturalist/horizon/entity_type/{kind}.json", {
            "template": "minecraft:zombie",
            "attributes": {f"minecraft:{key}": value for key, value in attributes.items()},
        })
        if weight:
            dump(pack, f"data/naturalist/horizon/item/{kind}_spawn_egg.json",
                 {"template": f"minecraft:{template}_spawn_egg"})
            dump(pack, f"data/naturalist/neoforge/biome_modifier/{kind}.json", {
                "type": "neoforge:add_spawns", "biomes": f"#naturalist:has_{kind}",
                "spawners": {"type": f"naturalist:{kind}", "weight": weight, "minCount": min_count,
                             "maxCount": max_count},
            })
    for variant in ("coral_snake", "rattlesnake"):
        dump(pack, f"data/naturalist/neoforge/biome_modifier/{variant}.json", {
            "type": "neoforge:add_spawns", "biomes": f"#naturalist:has_{variant}",
            "spawners": {"type": "naturalist:snake", "weight": 10, "minCount": 1, "maxCount": 1},
        })
    for item, template in ITEMS.items():
        spec = {"template": f"minecraft:{template}"}
        if item == "rat":
            spec["max_stack_size"] = 1
        if item in FOODS:
            nutrition, saturation = FOODS[item]
            spec["components"] = {
                "minecraft:food": {"nutrition": nutrition, "saturation": saturation},
            }
        if item == "lizard_tail":
            spec["components"]["minecraft:consumable"] = {"on_consume_effects": [
                {"type": "minecraft:apply_effects",
                 "effects": [{"id": "minecraft:poison", "duration": 100, "show_icon": True}],
                 "probability": 1.0},
            ]}
        if item in {"music_disc_death_by_hogs", "music_disc_wild_ones"}:
            song = item.removeprefix("music_disc_")
            spec.update(max_stack_size=1, rarity="rare", components={
                "minecraft:jukebox_playable": f"naturalist:{song}",
            })
        dump(pack, f"data/naturalist/horizon/item/{item}.json", spec)
    dump(pack, "data/naturalist/horizon/block/tortoise_egg.json", {
        "template": "minecraft:turtle_egg",
        "state_properties": {"variant": {"type": "enum", "values": ["0", "1", "2"]}},
        "default_state": {"variant": "0"},
    })
    dump(pack, "data/naturalist/horizon/mob_effect/antivenom.json", {
        "category": "beneficial", "color": 0x6FD6B2,
    })
    exact.update(f"data/naturalist/recipe/brewing/{form}_awkward_scorpion_poison_gland.json"
                 for form in ("potion", "splash_potion", "lingering_potion"))
    sound_prefixes = tuple(f"entity.{kind}." for kind in ("scorpion", *ANIMALS))
    lang = {
        *[f"item.naturalist.{name}" for name in names],
        *[f"entity.naturalist.{kind}" for kind in ANIMALS],
        "block.naturalist.tortoise_egg",
        "item.naturalist.music_disc_death_by_hogs.desc",
        "item.naturalist.music_disc_wild_ones.desc",
        "jukebox_song.naturalist.death_by_hogs",
        "jukebox_song.naturalist.wild_ones",
        "effect.naturalist.antivenom",
        *[f"item.minecraft.{form}.effect.antivenom" for form in ("potion", "splash_potion", "lingering_potion", "tipped_arrow")],
    }
    with ZipFile(jar) as source:
        for name in source.namelist():
            if not name.endswith("/") and (name in exact or name.startswith(tuple(prefixes))
                    or "tortoise_egg" in Path(name).name and name.startswith(("assets/", "data/"))):
                file = pack / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_bytes(source.read(name))
        for file, selected in (
            ("assets/naturalist/lang/en_us.json", lambda key: key in lang or key.startswith(
                tuple(f"naturalist.subtitles.{prefix}" for prefix in sound_prefixes))),
            ("assets/naturalist/sounds.json", lambda key: key.startswith(sound_prefixes)
             or key in {"music_disc.death_by_hogs", "music_disc.wild_ones"}),
        ):
            existing = json.loads((pack / file).read_text())
            existing.update({key: value for key, value in json.loads(source.read(file)).items() if selected(key)})
            dump(pack, file, existing)

    for tag, value in ("naturalist/tags/item/eggs", "naturalist:tortoise_egg"), (
        "c/tags/item/eggs", "naturalist:tortoise_egg"), (
        "c/tags/item/animal_foods", "#naturalist:tortoise_tempt_items",
    ):
        name = f"data/{tag}.json"
        path = pack / name
        data = json.loads(path.read_text()) if path.is_file() else {"values": []}
        if value not in data["values"]:
            data["values"].append(value)
        dump(pack, name, data)
