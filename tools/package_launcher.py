"""Build the private Horizon launcher preview from a local Naturalist jar."""
import argparse
from datetime import datetime, timezone
import hashlib
from itertools import groupby
from io import BytesIO
import json
from pathlib import Path
import re
from textwrap import indent
import tempfile
import tomllib
from zipfile import ZIP_DEFLATED, ZipFile

from pack_alligator import pack_alligator
from pack_hippo import pack_hippo
from pack_shark import pack_shark
from pack_elephants import pack_elephants
from pack_whale import pack_whale
from pack_hedgehog import pack_hedgehog
from pack_anglerfish import pack_anglerfish
from pack_ant import pack_ant
from pack_bass import pack_bass
from pack_bird import pack_bird
from pack_catfish import pack_catfish
from pack_duck import pack_duck
from pack_cooked_egg import pack_cooked_egg
from pack_decor import pack_decor
from pack_plush_bear import pack_plush_bear
from pack_insects import INSECTS, pack_insects
from pack_jellyfish import pack_jellyfish
from pack_knapsack import pack_knapsack
from pack_starfish import pack_starfish
from pack_clam import pack_clam
from pack_crab import pack_crab
from pack_glow_goop import pack_glow_goop
from pack_giraffe import pack_giraffe
from pack_giant_isopod import pack_giant_isopod
from pack_land import pack_land
from pack_ostrich import pack_ostrich
from pack_blobfish import pack_blobfish
from pack_piranha import pack_piranha
from pack_ray import pack_ray
from pack_spawn_tables import pack_spawn_tables
from pack_snail import pack_snail
from pack_vulture import pack_vulture
from pack_wings import WINGS, pack_wings
from prepare import ROOT, dump, pack_deer

GAME = "26.3"
MIXINS = "5a64af938ec86b00d53d16a408aabc0038fb3e57"


def deer_renderer():
    source = ROOT / "src"
    deer = (source / "deer.rs").read_text()
    body = deer[deer.index("use deer_animations::*;"):]
    start = body.index("impl Game<'_> {") + len("impl Game<'_> {")
    end = body.index("    pub(super) fn tick_naturalist_deer_animations(")
    body = body[:start] + "\n" + body[end:]
    body = body.replace("        let texture = self.deer_texture(mob, render.baby);", '''        let resolved = super::naturalist_deer_variants::appearance_texture(self, mob.id, render.baby);
        let texture = resolved.as_deref().unwrap_or(if render.baby {
            "naturalist:entity/deer/deer_baby"
        } else {
            "naturalist:entity/deer/deer"
        });''')
    models = (source / "deer_models.rs").read_text()
    animations = (source / "deer_animations.rs").read_text()
    return '''use super::{Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender};
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};

mod deer_models {
''' + models + '\n}\nmod deer_animations {\n' + animations + '\n}\n\n' + body


def animal_renderer(species):
    source = ROOT / "src"
    models = (source / f"{species}_models.rs").read_text()
    animations = (source / f"{species}_animations.rs").read_text()
    renderer = (source / f"{species}_render.rs").read_text()
    return (renderer + f"\nmod {species}_models {{\n" + models
            + f"\n}}\nmod {species}_animations {{\n" + animations + "\n}\n")


def elephant_renderer():
    source = ROOT / "src"
    parts = [(source / "elephant_render.rs").read_text()]
    for name in ("elephant_models", "mammoth_models", "elephant_animations"):
        body = (source / f"{name}.rs").read_text()
        parts.append(f"mod {name} {{\n{body}\n}}")
    return "\n".join(parts) + "\n"


# Stock limits native packages to 512 file entries. These are the shared
# hooks whose species helpers have identical arguments and ordering.
SHARED_HEAD_HOOKS = {
    "horizon_client_lib::mobs::ai::tick": "mob_tick",
    "horizon_client_lib::mobs::ai::step::step_sound": "mob_step_sound",
    "horizon_client_lib::mobs::ai::feeding::takes_food": "mob_takes_food",
    "horizon_client_lib::mobs::living::Mob::on_climbable": "mob_on_climbable",
    "horizon_client::game::<GameLevel<'_> as LevelAccess>::before_update_shape": "block_update_shape",
    "horizon_client::game::native_hooks::Game<'_>::native_use_item": "use_item",
    "horizon_client::game::native_hooks::Game<'_>::native_use_item_on": "use_item_on",
    "horizon_client::game::native_hooks::Game<'_>::native_tooltip": "native_tooltip",
    "horizon_client::game::native_hooks::Game<'_>::native_block_entity": "block_entity",
    "horizon_client::game::creatures::Game<'_>::handle_mob_events_reusable": "mob_events",
    "horizon_client::game::<GameLevel<'_> as LevelAccess>::before_state_for_placement": "block_placement",
    "horizon_client::game::native_hooks::Game<'_>::native_loot_drops": "loot_drops",
    "horizon_client_lib::mobs::tame::register": "register_tame_mob",
    "horizon_client::game::item_entities::Game<'_>::tick_items": "tick_dropped_item_family",
    "horizon_client::game::native_ticks::Game<'_>::native_tick_post_batch": "native_tick_post_batch",
    "horizon_client::game::lan_mobs::Game<'_>::lan_guest_mob_events": "lan_guest_mob_events",
    "horizon_client::game::native_hooks::Game<'_>::native_mobs_joined": "native_mobs_joined",
    "horizon_client_lib::mobs::ai::native_register_goals": "register_goals",
    "horizon_client::game::creatures::Game<'_>::interact_mob": "interact_mob",
    "horizon_client::game::creatures::Game<'_>::tick_creatures": "tick_creatures",
    "horizon_client::game::creatures::Game<'_>::tick_creature_entities": "tick_creature_entities",
    "horizon_client::game::native_hooks::Game<'_>::native_tick": "native_tick",
    "horizon_client_lib::entity_nbt::read_mob_home": "read_mob_home",
    "horizon_client_lib::mobs::ai::Mob::pushable": "mob_pushable",
    "horizon_client::game::creatures::Game<'_>::mob_died": "mob_died",
    "horizon_client_lib::mobs::ai::push_entities": "mob_push_entities",
    "horizon_client::game::mob_looting::Game<'_>::tick_mob_looting": "mob_looting",
    "horizon_client::game::lan::mob_meta": "mob_meta",
    "horizon_client_lib::mobs::ai::requires_custom_persistence": "mob_persistence",
    "horizon_client_lib::mobs::mob_can_use_portal": "mob_portal",
    "horizon_client::game::Game<'_>::process_level_events": "process_level_events",
    "horizon_client::game::creatures::Game<'_>::breed": "breed",
    "horizon_client::game::commands::Game<'_>::run_command": "run_command",
    "horizon_client::game::natural_spawning::Game<'_>::finalize_spawned": "finalize_spawned",
    "horizon_client::game::persist::hurt_by_refs::Game<'_>::put_last_hurt_by_mob": "save_mob_state",
    "horizon_client::game::remote_mob_use::Game<'_>::remote_mob_answers": "remote_mob_answers",
    "horizon_client::game::Game<'_>::mob_hit_within": "mob_hit_within",
    "horizon_client_lib::mobs::spawning::finalize": "finalize_mob",
    "horizon_client_lib::mobs::ai::custom_server_ai_step": "mob_ai_step",
    "horizon_client_lib::mobs::living::Mob::is_baby": "mob_is_baby",
    "horizon_client::game::<GameLevel<'_> as LevelAccess>::before_random_tick": "random_block_tick",
    "horizon_client_lib::mobs::living::Mob::awake_dims_in": "awake_dims",
    "horizon_client::game::creatures::prep::Game<'_>::prepare_mobs": "prepare_birds",
    "horizon_client::game::allays::Game<'_>::tick_allays": "tick_bird_forage",
    "horizon_client_lib::mobs::living::Mob::jump_power_with": "mob_jump_power",
    "horizon_client_lib::mobs::riders::passenger_attachment": "passenger_attachment",
    "horizon_client_lib::mobs::Mob::damage_with": "mob_damage_with",
    "horizon_client::game::remote_mob_data::apply": "remote_mob_metadata",
    "horizon_client::game::creatures::Game<'_>::mob_hits_player": "mob_hits_player",
    "horizon_client::game::combat::Game<'_>::mob_attacks_mob": "mob_hits_mob",
    "horizon_client_lib::mobs::Mob::refuses_damage": "mob_damage_immunity",
    "horizon_client_lib::mobs::riders::read_nbt": "mob_rider_nbt",
    "horizon_client_lib::mobs::equine::ridden_step": "mob_ridden_step",
}


def head_signature(entry):
    fields = {"id", "target", "kind", "helper", "bindings", "cancellable"}
    if entry["kind"] != "HEAD" or not entry.keys() <= fields:
        return None
    bindings = tuple((binding["ty"], binding["value"]) for binding in entry.get("bindings", ()))
    return entry.get("cancellable", False), bindings


def combine_hooks(native, name, entries, signature):
    cancellable, bindings = signature
    sources = [entry["helper"] for entry in entries]
    declarations = []
    for filename in sources:
        source = (native / filename).read_text().rstrip()
        declaration = re.match(
            r"fn\s+(\w+)\s*\([^{}]*?\)\s*(?:->\s*([^{}]*?))?\s*\{", source,
            re.DOTALL,
        )
        if not declaration:
            return None
        function, result = declaration.groups()
        declarations.append((source, function, result.strip() if result else None))
    returns = {entry[2] for entry in declarations}
    if len(returns) != 1 or cancellable != (None not in returns):
        return None
    result = returns.pop()
    arguments = [binding["name"] for binding in entries[0]["bindings"]]
    formal = ", ".join(f"{argument}: {binding[0]}" for argument, binding in zip(arguments, bindings))
    rust = f"fn {name}({formal})"
    if result:
        rust += f" -> {result}"
    rust += " {\n"
    for source, function, _ in declarations:
        reborrows = [
            f"&mut *{argument}" if declared.startswith("&mut ") else argument
            for argument, (declared, _) in zip(arguments, bindings)
        ]
        body = f"{source}\n{function}({', '.join(reborrows)})"
        if cancellable:
            rust += "    if let std::ops::ControlFlow::Break(result) = {\n"
            rust += indent(body, "        ") + "\n"
            rust += "    } {\n        return std::ops::ControlFlow::Break(result);\n    }\n"
        else:
            rust += "    {\n" + indent(body + ";", "        ") + "\n    }\n"
    if cancellable:
        rust += "    std::ops::ControlFlow::Continue(())\n"
    return rust + "}\n"


def launcher_sources(native):
    """Bundle adjacent compatible HEAD callbacks in their original invocation order."""
    manifest_name = "horizon-mixin.toml"
    manifest = (native / manifest_name).read_text()
    injections = tomllib.loads(manifest)["injection"]
    sections = re.split(r"(?m)(?=^\[\[(?:injection|module|state)\]\])", manifest)
    positions = {
        match[1]: index for index, section in enumerate(sections)
        if (match := re.search(r'(?m)^id = "([^"]+)"', section))
    }
    callbacks = {}
    paths = {manifest_name}
    leftovers = {}
    for target, group_name in SHARED_HEAD_HOOKS.items():
        relevant = [entry for entry in injections if entry.get("target") == target]
        for index, (signature, group) in enumerate(groupby(relevant, head_signature)):
            entries = list(group)
            sources = [entry["helper"] for entry in entries]
            if signature is None or len(entries) < 2:
                continue
            name = f"naturalist_{group_name}_{index}"
            bundled = combine_hooks(native, name, entries, signature)
            if bundled is None:
                continue
            first, *rest = (positions[entry["id"]] for entry in entries)
            filename = f"{name}.rs"
            callbacks[filename] = bundled
            paths.update(source for source in sources if manifest.count(f'"{source}"') == 1)
            leftovers[first] = sections[first].replace(entries[0]["id"], name.replace("_", "-"), 1) \
                .replace(sources[0], filename, 1)
            leftovers.update((position, "") for position in rest)
    return "".join(leftovers.get(index, section) for index, section in enumerate(sections)), callbacks, paths


def package(jar, output):
    native = ROOT / "launcher"
    config = tomllib.loads((native / "horizon-mixin.toml").read_text())
    referenced_rust = {entry["helper"] for entry in config["injection"]}
    referenced_rust.update(entry["source"] for entry in config.get("module", ()))
    version = config["mod"]["version"]
    horizon = config["compatibility"]["horizon_revision"]
    output.mkdir(parents=True, exist_ok=True)
    artifact = output / f"naturalist-{version}.hnmod"
    prefix = "mixins/naturalist/"

    with tempfile.TemporaryDirectory(prefix="naturalist-launcher-") as work:
        content = Path(work)
        pack_deer(jar, content)
        pack_catfish(jar, content, dump)
        pack_blobfish(jar, content, dump)
        pack_ray(jar, content, dump)
        pack_jellyfish(jar, content, dump)
        pack_starfish(jar, content, dump)
        pack_alligator(jar, content, dump)
        pack_hippo(jar, content, dump)
        pack_shark(jar, content, dump)
        pack_whale(jar, content, dump)
        pack_crab(jar, content, dump)
        pack_giraffe(jar, content, dump)
        pack_giant_isopod(jar, content, dump)
        pack_piranha(jar, content, dump)
        pack_duck(jar, content, dump)
        pack_bass(jar, content, dump)
        pack_anglerfish(jar, content, dump)
        pack_bird(jar, content, dump)
        pack_insects(jar, content, dump)
        pack_clam(jar, content, dump)
        pack_wings(jar, content, dump)
        pack_vulture(jar, content, dump)
        pack_snail(jar, content, dump)
        pack_decor(jar, content, dump)
        pack_glow_goop(jar, content, dump)
        pack_plush_bear(jar, content, dump)
        pack_land(jar, content, dump)
        pack_knapsack(jar, content, dump)
        pack_elephants(jar, content, dump)
        pack_ostrich(jar, content, dump)
        pack_ant(jar, content, dump)
        pack_hedgehog(jar, content, dump)
        pack_cooked_egg(jar, content, dump)
        pack_spawn_tables(jar, content, dump)
        with ZipFile(artifact, "w", ZIP_DEFLATED) as archive:
            archive.writestr("horizon-native.json", json.dumps({
                "schema": 1, "id": "naturalist", "version": version, "game": GAME,
                "horizon": horizon, "mixins": MIXINS,
            }))
            manifest, generated, bundled = launcher_sources(native)
            archive.writestr(prefix + "horizon-mixin.toml", manifest)
            for name, text in generated.items():
                archive.writestr(prefix + name, text)
            for path in sorted(native.rglob("*")):
                relative = path.relative_to(native).as_posix()
                if path.parent == native and path.suffix == ".rs" and relative not in referenced_rust:
                    continue
                if path.is_file() and relative not in bundled:
                    archive.write(path, prefix + relative)
            archive.writestr(prefix + "deer_render.rs", deer_renderer())
            archive.writestr(prefix + "elephant_render.rs", elephant_renderer())
            for species in ("ostrich", "duck", "bass", "bird", "alligator", "hippo", "great_white_shark", "whale", "hedgehog", "capybara", "catfish", "piranha", "blobfish", "jellyfish", "starfish", "giant_isopod", "clam", "crab", "giraffe", "tortoise", "tiger", "lion", "snake", "rat", "mole", "dirt_trail", "ray", "anglerfish", "vulture", "snail", "ant", *INSECTS, *WINGS):
                archive.writestr(prefix + f"{species}_render.rs", animal_renderer(species))
            land = (ROOT / "src/land_render.rs").read_text()
            for species in ("boar", "turkey", "zebra", "desert_scorpion", "jungle_scorpion", "lizard", "lizard_tail", "komodo_dragon", "rhino"):
                for kind in ("models", "animations"):
                    name = f"{species}_{kind}"
                    body = (ROOT / "src" / f"{name}.rs").read_text()
                    land += f"\nmod {name} {{\n{body}\n}}\n"
            archive.writestr(prefix + "land_render.rs", land)
            bears = (ROOT / "src/bears_render.rs").read_text()
            for species in ("bear", "black_bear"):
                for kind in ("models", "animations"):
                    name = f"{species}_{kind}"
                    body = (ROOT / "src" / f"{name}.rs").read_text()
                    bears += f"\nmod {name} {{\n{body}\n}}\n"
            archive.writestr(prefix + "bears_render.rs", bears)
            content_zip = BytesIO()
            with ZipFile(content_zip, "w", ZIP_DEFLATED) as assets:
                assets.write(ROOT / "LICENSE-Naturalist", "LICENSE-Naturalist")
                for path in sorted(content.rglob("*")):
                    if not path.is_file():
                        continue
                    name = path.relative_to(content).as_posix()
                    entry = name
                    if name == "data/naturalist/horizon/entity_type/deer.json":
                        # The pinned stock content checker accepts this template.
                        # The native hook replaces it with the cow-based deer definition.
                        entity = json.loads(path.read_text())
                        entity["template"] = "minecraft:wandering_trader"
                        assets.writestr(entry, json.dumps(entity))
                    elif name == "horizon-mod.toml":
                        assets.writestr(entry, path.read_text().replace('version = "0.1.0"', f'version = "{version}"'))
                    else:
                        assets.write(path, entry)
            archive.writestr(prefix + "data/content.zip", content_zip.getvalue())

    data = artifact.read_bytes()
    now = datetime.now(timezone.utc)
    index = {"schema": 1, "generated": now.isoformat(), "projects": [{
        "id": "naturalist", "kind": "native-mod", "name": "Naturalist wildlife preview",
        "summary": "Private Naturalist wildlife preview for Horizon 26.3",
        "authors": ["Naturalist port"], "owners": ["naturalist-port"],
        "license": "MIT AND LicenseRef-Naturalist-Assets",
        "source": "https://github.com/crispytwig/Naturalist", "categories": [],
        "versions": [{"version": version, "published": now.date().isoformat(),
                      "api": f"native-1/{horizon}/{MIXINS}", "game": GAME,
                      "file": {"name": artifact.name, "url": artifact.as_uri(), "size": len(data),
                               "sha256": hashlib.sha256(data).hexdigest()}}],
    }]}
    (output / "local-index.json").write_text(json.dumps(index, indent=2) + "\n")
    return artifact


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("jar", type=Path, help="Naturalist 2.0.7 NeoForge for Minecraft 26.3")
    cli.add_argument("output", type=Path, help="Private directory for the package and local catalog")
    args = cli.parse_args()
    print(package(args.jar, args.output.resolve()))


if __name__ == "__main__":
    main()
