"""Stage the deer pack and Horizon engine changes from the pinned references."""
from pathlib import Path
import argparse
import hashlib
import json
import re
import shutil
from zipfile import ZipFile

ROOT = Path(__file__).resolve().parents[1]
JAR_SHA = "b1a220dc6bcb6101911226a57d75d62f2797461c3e529e8cf7110673b69632538c4a129b135f9572411cbc60336c096e056488fbe1528638196c247eef6492cd"
DEER = "naturalist:deer"


def dump(pack, path, value):
    target = pack / path
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(value, indent=2) + "\n")


def aquatic_item(template):
    definition = {"template": f"minecraft:{template}"}
    if template == "cod_bucket":
        # The Java Naturalist bucket is a normal water container. Horizon's
        # vanilla cod bucket provides the food values of the enclosed cod.
        definition["components"] = {"!minecraft:food": {}}
    return definition


def pack_deer(jar, pack):
    if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
        raise ValueError("Use Naturalist 2.0.7 NeoForge for Minecraft 26.3")
    items = {"antler": "stick", "venison": "beef", "cooked_venison": "cooked_beef",
             "deer_spawn_egg": "cow_spawn_egg"}
    with ZipFile(jar) as source:
        for name in source.namelist():
            if name.endswith("/"):
                continue
            include = (
                name.startswith("assets/naturalist/textures/entity/deer/")
                or name.startswith("assets/naturalist/sounds/entity/deer/")
                or name.startswith("data/naturalist/naturalist/deer_variant/")
                or name == "data/naturalist/loot_table/entities/deer.json"
                or name == "data/naturalist/tags/entity_type/deer_predators.json"
                or name.startswith("data/naturalist/tags/worldgen/biome/") and "deer" in name
                or any(name == f"assets/naturalist/{folder}/{item}{ext}"
                       for item in items
                       for folder, ext in [("items", ".json"), ("models/item", ".json"),
                                           ("textures/item", ".png")])
            )
            if name.startswith("data/naturalist/recipe/") and name.endswith(".json"):
                recipe = json.loads(source.read(name))
                include = (recipe.get("ingredient") == "naturalist:venison"
                           or "/brewing/" in name and ("_awkward_antler" in name or "_forest_dasher_" in name))
            if include:
                path = pack / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(source.read(name))
        names = json.loads(source.read("assets/naturalist/lang/en_us.json"))
        deer_names = ["entity.naturalist.deer", *(f"item.naturalist.{item}" for item in items)]
        deer_names.extend(
            key for key in names
            if key.endswith(".effect.forest_dasher") or key.startswith("naturalist.subtitles.entity.deer.")
        )
        dump(pack, "assets/naturalist/lang/en_us.json", {key: names[key] for key in deer_names})
        sounds = json.loads(source.read("assets/naturalist/sounds.json"))
        deer_sounds = {key: sound for key, sound in sounds.items() if key.startswith("entity.deer.")}
        dump(pack, "assets/naturalist/sounds.json", deer_sounds)
        for entry in deer_sounds.values():
            for sound in entry["sounds"]:
                sound_id = sound if isinstance(sound, str) else sound["name"]
                if not sound_id.startswith("naturalist:"):
                    continue
                name = f'assets/naturalist/sounds/{sound_id.split(":", 1)[1]}.ogg'
                path = pack / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(source.read(name))
    for item, template in items.items():
        value = {"template": f"minecraft:{template}"}
        food = {"venison": (2, 1.2), "cooked_venison": (6, 9.6)}.get(item)
        if food:
            value["components"] = {"minecraft:food": {"nutrition": food[0], "saturation": food[1]}}
        dump(pack, f"data/naturalist/horizon/item/{item}.json", value)
    dump(pack, "data/naturalist/horizon/entity_type/deer.json", {
        "template": "minecraft:cow",
        "attributes": {"minecraft:max_health": 10, "minecraft:movement_speed": 0.20000000298023224,
                       "minecraft:step_height": 2},
    })
    dump(pack, "data/naturalist/neoforge/biome_modifier/deer.json", {
        "type": "neoforge:add_spawns", "biomes": "#naturalist:has_deer",
        "spawners": {"type": DEER, "weight": 10, "minCount": 3, "maxCount": 5},
    })
    pack.joinpath("horizon-mod.toml").write_text(
        'id = "naturalist"\nname = "Naturalist Deer"\nversion = "0.1.0"\napi = "0.2"\n'
        'side = "both"\ncapabilities = ["registries"]\n')


def stage_engine(engine):
    def edit(file, changes, root="horizon-client"):
        file = engine / "crates" / root / "src" / file
        src = file.read_text()
        for old, new in changes:
            if src.count(old) != 1:
                raise ValueError(f"{file.name}: expected a unique marker for {old!r}")
            src = src.replace(old, new)
        file.write_text(src)

    edit('potions.rs', [
        ('    POTIONS.iter().find(|p| p.id == id)',
         '    const FOREST_DASHER: &[PotionDef] = &[\n        p("naturalist:forest_dasher", "forest_dasher", &[("speed", 400, 1), ("weakness", 400, 0)]),\n        p("naturalist:long_forest_dasher", "forest_dasher", &[("speed", 800, 1), ("weakness", 800, 0)]),\n        p("naturalist:strong_forest_dasher", "forest_dasher", &[("speed", 400, 2), ("weakness", 400, 1)]),\n    ];\n    POTIONS.iter().chain(FOREST_DASHER).find(|p| p.id == id)'),
    ], 'horizon-client-core')
    edit('brewing.rs', [
        ('fn brewing_sources(root: &Path) -> Vec<(String, Value)> {\n    let recipe = root.join("data/minecraft/recipe");',
         'fn brewing_sources(root: &Path, namespace: &str) -> Vec<(String, Value)> {\n    let recipe = root.join("data").join(namespace).join("recipe");'),
    ], 'horizon-client-render')
    edit('brewing.rs', [
        ('            Some((format!("minecraft:brewing/{name}"), v))',
         '            Some((format!("{namespace}:brewing/{name}"), v))'),
    ], 'horizon-client-render')
    edit('brewing.rs', [
        ('        let mut sources: Vec<(String, Value)> = brewing_sources(root);',
         '        let mut sources = brewing_sources(root, "minecraft");\n        sources.extend(brewing_sources(root, "naturalist"));'),
    ], 'horizon-client-render')
    edit('brewing.rs', [
        ('            if !id.starts_with("minecraft:brewing/") {',
         '            if !id.split_once(\':\').is_some_and(|(_, path)| path.starts_with("brewing/")) {'),
    ], 'horizon-client-render')
    edit('potions.rs', [
        ('pub fn potion(id: &str) -> Option<&\'static PotionDef> {\n    let id = id.trim_start_matches("minecraft:");\n    const FOREST_DASHER: &[PotionDef] = &[\n        p("naturalist:forest_dasher", "forest_dasher", &[("speed", 400, 1), ("weakness", 400, 0)]),\n        p("naturalist:long_forest_dasher", "forest_dasher", &[("speed", 800, 1), ("weakness", 800, 0)]),\n        p("naturalist:strong_forest_dasher", "forest_dasher", &[("speed", 400, 2), ("weakness", 400, 1)]),\n    ];\n',
         'pub const FOREST_DASHER: &[PotionDef] = &[\n    p("naturalist:forest_dasher", "forest_dasher", &[("speed", 400, 1), ("weakness", 400, 0)]),\n    p("naturalist:long_forest_dasher", "forest_dasher", &[("speed", 800, 1), ("weakness", 800, 0)]),\n    p("naturalist:strong_forest_dasher", "forest_dasher", &[("speed", 400, 2), ("weakness", 400, 1)]),\n];\n\npub fn potion(id: &str) -> Option<&\'static PotionDef> {\n    let id = id.trim_start_matches("minecraft:");\n'),
    ], 'horizon-client-core')
    edit('mod_content.rs', [
        ("impl Plan<'_> {\n    /// `entity-types.json`",
         "fn forest_dasher_ids() -> impl Iterator<Item = &'static str> {\n    horizon_client_core::potions::FOREST_DASHER.iter().map(|potion| potion.id)\n}\n\nimpl Plan<'_> {\n    /// `entity-types.json`"),
        ("                // Fluid tags go by these ids: a mod fluid's source, then its flowing form.",
         '                if self.entity_types.iter().any(|entity| entity.id == "naturalist:deer") {\n                    registries.push(("minecraft:potion", forest_dasher_ids().map(str::to_owned).collect()));\n                }\n                // Fluid tags go by these ids: a mod fluid\'s source, then its flowing form.'),
        ('                // Fluid tag files and fluid arguments.',
         '                if self.entity_types.iter().any(|entity| entity.id == "naturalist:deer")\n                    && let Some(J::Arr(ids)) = doc.at("by_registry/minecraft:potion/ids")\n                {\n                    ids.extend(forest_dasher_ids().map(|id| J::Str(id.to_owned())));\n                }\n                // Fluid tag files and fluid arguments.'),
    ], 'horizon-client')
    edit("content.rs", [
        ("pub const ENTITY_TEMPLATES: [&str; 4] = [", 'pub const ENTITY_TEMPLATES: [&str; 5] = [\n    "minecraft:cow",'),
        ("pub const LIVING_TEMPLATES: [&str; 3] = [", 'pub const LIVING_TEMPLATES: [&str; 4] = [\n    "minecraft:cow",'),
    ], "horizon-mods")
    edit("mod_content.rs", [
        ('    fields.insert("template".into(), to_raw_value(&t.template).map_err(fail)?);',
         '    if t.id == "naturalist:deer" {\n'
         '        fields.insert("width".into(), to_raw_value(&f64::from(1.3f32)).map_err(fail)?);\n'
         '        fields.insert("height".into(), to_raw_value(&f64::from(1.6f32)).map_err(fail)?);\n'
         '        fields.insert("eye_height".into(), to_raw_value(&f64::from(1.36f32)).map_err(fail)?);\n'
         '    }\n    fields.insert("template".into(), to_raw_value(&t.template).map_err(fail)?);'),
    ])
    edit("mobs.rs", [
        ('    pub fn voice_pitch(&mut self) -> f32 {',
         '    pub fn voice_pitch(&mut self) -> f32 {\n'
         '        if self.kind.id == "naturalist:deer" {\n'
         '            return (self.random.next_float() - self.random.next_float()) * 0.2 + 1.0;\n'
         '        }'),
        ("pub struct Seen {\n    pub in_water", "pub struct Seen {\n    pub naturalist_deer: bool,\n    pub naturalist_deer_predator: bool,\n    pub in_water"),
        ("pub struct Extra {\n", "pub struct Extra {\n    pub deer_panic_ticks: i32,\n    pub deer_panic_relay: Option<i32>,\n"),
        ("        ai::after_hurt(self, &source);", '        if self.kind.id == "naturalist:deer" {\n'
         '            let ticks = 100 + self.random.next_int_bound(100).unwrap_or(0);\n'
         '            self.extra.deer_panic_ticks = ticks;\n'
         '            self.extra.deer_panic_relay = Some(ticks);\n'
         '        }\n        ai::after_hurt(self, &source);'),
        ('                    food: def\n                        .food_tag()\n                        .map(tag)\n                        .unwrap_or_else(|| Arc::from(Vec::new())),',
         '                    food: if id == "naturalist:deer" {\n'
         '                        Arc::from(vec!["minecraft:apple".to_owned()])\n'
         '                    } else {\n                        def.food_tag().map(tag).unwrap_or_default()\n                    },'),
    ])
    edit("mobs/seen.rs", [('        Seen {\n', '''        Seen {
            naturalist_deer: self.kind.id == "naturalist:deer",
            naturalist_deer_predator: matches!(self.kind.id.as_str(),
                "naturalist:bear" | "naturalist:black_bear" | "naturalist:tiger"
                | "alexsmobs:grizzly_bear" | "alexsmobs:crocodile" | "alexsmobs:tiger"
                | "alexsmobs:snow_leopard" | "alexsmobs:anaconda" | "alexsmobs:komodo_dragon"
                | "alexsmobs:froststalker"),
''')])
    edit("mobs/farm.rs", [
        ("    match m.kind.species {", '''    if m.kind.id == "naturalist:deer" {
        use super::goals::{Avoid, AvoidEntity};
        g.add(0, Float);
        g.add(1, Panic::new(1.6));
        g.add(2, Breed::new(1.0));
        g.add(3, Tempt::new(1.25, food, true));
        g.add(4, AvoidEntity::new(Avoid::Player, 8.0, 1.5, 2.0)
            .player_when(|_, player| !player.creative && !player.spectator && !player.sneaking));
        g.add(4, AvoidEntity::new(Avoid::Enemies, 4.0, 1.5, 2.0));
        g.add(4, AvoidEntity::new(Avoid::DeerPredators, 10.0, 1.5, 2.0));
        g.add(5, FollowParent::new(1.5));
        g.add(6, EatBlock::default());
        g.add(7, Stroll::avoiding_water(1.0));
        g.add(8, LookAtPlayer::new(6.0));
        g.add(9, RandomLookAround::default());
        return;
    }
    match m.kind.species {'''),
    ])
    edit("mobs/goals.rs", [
        ("pub enum Avoid {\n    Player,\n    Species(Vec<Species>),\n", "pub enum Avoid {\n    Player,\n    Species(Vec<Species>),\n    Enemies,\n    DeerPredators,\n"),
        ('''        let Avoid::Species(list) = &self.what else {
            return None;
        };
        let id = m.id;''', '''        let filter = &self.what;
        let id = m.id;'''),
        ("""                && list.contains(&s.species)
                && seen_when""", """                && match filter {
                    Avoid::Species(list) => list.contains(&s.species),
                    Avoid::Enemies => s.hostile && !matches!(s.species,
                        Species::Slime | Species::MagmaCube | Species::Ghast | Species::Shulker | Species::Phantom),
                    Avoid::DeerPredators => s.naturalist_deer_predator,
                    Avoid::Player => false,
                }
                && seen_when"""),
        ("            Avoid::Species(_) => self.from,", "            Avoid::Species(_) | Avoid::Enemies | Avoid::DeerPredators => self.from,"),
        ("                let same = s.species == species", "                let same = s.species == species && s.naturalist_deer == deer"),
        ("        self.partner =\n            cx.w.nearby", '        let deer = m.kind.id == "naturalist:deer";\n        self.partner =\n            cx.w.nearby'),
        ("        let species = m.species();\n        let pos = m.position;\n        let mut best", '        let species = m.species();\n        let deer = m.kind.id == "naturalist:deer";\n        let pos = m.position;\n        let mut best'),
        ("                && s.species == species\n                && !s.baby", "                && s.species == species\n                && s.naturalist_deer == deer\n                && !s.baby"),
        ('impl Goal for Panic {\n', (ROOT / "launcher/deer_big_panic.rs").read_text() + "\n" + 'impl Goal for Panic {\n'),
        ('    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {\n        let wanted = if self.environmental {', '    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {\n        if let std::ops::ControlFlow::Break(can_panic) = deer_big_panic(self, m, cx) {\n            return can_panic;\n        }\n        let wanted = if self.environmental {'),
    ])
    edit("mobs/ai.rs", [
        ('pub fn voice_sounds(m: &Mob) -> [&\'static str; 3] {',
         'pub fn voice_sounds(m: &Mob) -> [&\'static str; 3] {\n'
         '    if m.kind.id == "naturalist:deer" {\n'
         '        return if m.is_baby() { ["naturalist:entity.deer.ambient_baby", "naturalist:entity.deer.hurt_baby", "naturalist:entity.deer.death_baby"] }\n'
         '            else { ["naturalist:entity.deer.ambient", "naturalist:entity.deer.hurt", "naturalist:entity.deer.death"] };\n'
         '    }'),
        ('fn custom_server_ai_step(m: &mut Mob, cx: &mut Cx) {',
         'fn custom_server_ai_step(m: &mut Mob, cx: &mut Cx) {\n'
         '    if m.kind.id == "naturalist:deer" {\n'
         '        m.extra.sprinting = !m.ai.nav.is_done() && m.ai.nav.speed_modifier >= 1.5;\n'
         '        m.extra.anim_ticks = (m.extra.anim_ticks - 1).max(0);\n'
         '    }'),
        ('pub fn ate(m: &mut Mob, events: &mut Vec<MobEvent>) {',
         'pub fn ate(m: &mut Mob, events: &mut Vec<MobEvent>) {\n    if m.kind.id == "naturalist:deer" {\n        if can_age_up(m) {\n            age_up(m, 60, false);\n        }\n        return;\n    }'),
    ])
    edit("mobs/spawn_rules.rs", [
        ("fn bright<L: SpawnLevel>", """pub(super) fn deer_spawn_rules<L: SpawnLevel>(l: &L, x: i32, y: i32, z: i32) -> bool {
    tag(l, "minecraft:substrate_overworld", x, y - 1, z) && bright(l, x, y, z)
}

fn bright<L: SpawnLevel>"""),
    ])
    edit("mobs/spawn_selection.rs", [
        ("    if !spawn_rules::check_spawn_rules(species, l, Reason::Natural, random, x, y, z) {",
         """    let allowed = if entry.entity == "naturalist:deer" {
        spawn_rules::deer_spawn_rules(l, x, y, z)
    } else {
        spawn_rules::check_spawn_rules(species, l, Reason::Natural, random, x, y, z)
    };
    if !allowed {"""),
        ("""                    if !l.no_collision(spawn_aabb(species, fx, fy, fz, width, height))
                        || !spawn_rules::check_spawn_rules(
                            species,
                            l,
                            Reason::ChunkGeneration,
                            rules,
                            bx,
                            pos[1],
                            bz,
                        )
                    {""",
         """                    let bounds = spawn_aabb(species, fx, fy, fz, width, height);
                    if !l.no_collision(bounds) {
                        continue;
                    }
                    let allowed = if entry.entity == "naturalist:deer" {
                        spawn_rules::deer_spawn_rules(l, bx, pos[1], bz)
                    } else {
                        spawn_rules::check_spawn_rules(species, l, Reason::ChunkGeneration, rules, bx, pos[1], bz)
                    };
                    if !allowed {"""),
    ])
    edit("game/remote.rs", [
        ("        self.native_tick();", "        self.tick_naturalist_deer_animations();\n        self.native_tick();"),
    ])
    edit("mobs/living.rs", [
        ("    pub fn awake_dims_in(&self, long_jumping: bool) -> (f64, f64, f64) {",
         "    pub fn awake_dims_in(&self, long_jumping: bool) -> (f64, f64, f64) {\n"
         "        if self.kind.id == \"naturalist:deer\" && self.is_baby() {\n"
         "            let scale = self.model_scale();\n"
         "            let scaled = |d: f64| f64::from(d as f32 * 0.5 * scale);\n"
         "            return (scaled(self.kind.width), scaled(self.kind.height), scaled(self.kind.eye_height));\n"
         "        }"),
    ])
    edit("mobs/sync.rs", [
        ("Species::Sheep if m.extra.anim_ticks > 0 =>", 'Species::Sheep | Species::Cow if (m.species() == Species::Sheep || m.kind.id == "naturalist:deer") && m.extra.anim_ticks > 0 =>'),
        ("Species::Sheep if event == 10 =>", 'Species::Sheep | Species::Cow if (m.species() == Species::Sheep || m.kind.id == "naturalist:deer") && event == 10 =>'),
    ])
    edit("items.rs", [
        ('        let name = entity.trim_start_matches("minecraft:");\n        let Some(table) = self.loot.get(&format!("minecraft:entities/{name}")) else {',
         '        let (namespace, name) = entity.split_once(\':\').unwrap_or(("minecraft", entity));\n        let Some(table) = self.loot.get(&format!("{namespace}:entities/{name}")) else {'),
    ], "horizon-items")
    edit("entity_nbt.rs", [
        ('    if is_animal(&m.kind.id) {', '    if is_animal(m.kind.class_id()) {'),
        ('    // AbstractFish/Axolotl.addAdditionalSaveData always write the flag.',
         '    if m.kind.id == "naturalist:deer" {\n'
         '        map.entry("Variant".into()).or_insert_with(|| Tag::String("naturalist:deer".into()));\n'
         '    }\n    // AbstractFish/Axolotl.addAdditionalSaveData always write the flag.'),
    ])
    edit("game/creatures.rs", [
        ('                    self.audio.play_bound(\n                        &format!("minecraft:{name}"),',
         '                    let key = if name.contains(\':\') { name.to_owned() } else { format!("minecraft:{name}") };\n'
         '                    self.audio.play_bound(\n                        &key,'),
        ('Species::Cow | Species::Mooshroom if item == "minecraft:bucket" && !m.is_baby() => {',
         'Species::Cow | Species::Mooshroom if item == "minecraft:bucket" && !m.is_baby() && m.kind.id != "naturalist:deer" => {'),
    ])
    edit("game.rs", [
        ("mod native_mob_draw;", "mod native_mob_draw;\nmod naturalist_deer;"),
        ("        self.tick_creatures()?;\n        if !self.entity_attachments.is_empty()", "        self.tick_creatures()?;\n        self.update_deer_herds();\n        if !self.entity_attachments.is_empty()"),
    ])
    edit("game/commands/world_cmds.rs", [
        ('        let value_arg = match ctx.get("value") {',
         '        if let Target::Mob(entity) = target\n'
         '            && attribute == "minecraft:step_height"\n'
         '            && matches!(id, "attribute.get" | "attribute.base.get")\n'
         '            && let Some(deer) = self.mobs.iter().find(|m| m.id == entity && m.kind.id == "naturalist:deer")\n'
         '        {\n'
         '            let value = deer.kind.step_height;\n'
         '            let message = if id == "attribute.get" { "commands.attribute.value.get.success" }\n'
         '                else { "commands.attribute.base_value.get.success" };\n'
         '            return Ok(Some(((value * scale) as i32, Text::translate(message, &[description, name, java_double_text(value)]))));\n'
         '        }\n'
         '        let value_arg = match ctx.get("value") {'),
    ])
    edit("game/native_mob_draw.rs", [
        ("            let mut drawn = false;\n            self.native_mob_submit", "            if self.draw_naturalist_deer(mob, render, draw) {\n                return true;\n            }\n            let mut drawn = false;\n            self.native_mob_submit"),
    ])
    edit('datapack.rs', [
        ('        "tags" => {\n            return Some(format!("tags/{ns}:{}", rest.strip_suffix(".json")?));\n        }\n        _ => return None,',
         '        "tags" => {\n            return Some(format!("tags/{ns}:{}", rest.strip_suffix(".json")?));\n        }\n        "naturalist" => {\n            return Some(format!("naturalist/deer_variant/{ns}:{}", rest\n                .strip_prefix("deer_variant/")?.strip_suffix(".json")?));\n        }\n        _ => return None,'),
        ('                "loot_table",\n                "predicate",',
         '                "loot_table",\n                "naturalist/deer_variant",\n                "predicate",'),
    ], 'horizon-client-render')
    edit('game.rs', [
        ("    suffocation: horizon_world::suffocation::SuffocationRegistry<'static>,\n    /// The pack files",
         "    suffocation: horizon_world::suffocation::SuffocationRegistry<'static>,\n    deer_variants: naturalist_deer::DeerVariants,\n    /// The pack files"),
    ], 'horizon-client')
    edit('game/world_entry.rs', [
        ('            Some(WorldTables {\n                suffocation,\n                files,',
         '            Some(WorldTables {\n                suffocation,\n                deer_variants: super::naturalist_deer::DeerVariants::load(root, &overlay),\n                files,'),
    ], 'horizon-client')
    edit('game/commands/execute.rs', [
        ('    pub(super) fn biome_matches(&self,',
         '    pub(in crate::game) fn biome_matches(&self,'),
    ], 'horizon-client')
    edit('mobs.rs', [
        ('pub struct Extra {\n    pub deer_panic_ticks:',
         'pub struct Extra {\n    pub naturalist_deer_variant: Option<String>,\n    pub deer_panic_ticks:'),
    ], 'horizon-client')
    edit('entity_nbt.rs', [
        ('        map.entry("Variant".into()).or_insert_with(|| Tag::String("naturalist:deer".into()));',
         '        let variant = m.extra.naturalist_deer_variant.as_deref().unwrap_or("naturalist:deer");\n        map.insert("Variant".into(), Tag::String(variant.into()));'),
        ('    let mut m = Mob::with_random(id, kind, position, yaw, random);',
         '    let mut m = Mob::with_random(id, kind, position, yaw, random);\n    if m.kind.id == "naturalist:deer"\n        && let Some(id) = text(tag.get("Variant"))\n    {\n        let (namespace, path) = id.split_once(\':\').unwrap_or(("minecraft", &id));\n        if !namespace.is_empty() && !path.is_empty()\n            && namespace.bytes().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || b"_-.".contains(&c))\n            && path.bytes().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || b"_-./".contains(&c))\n        {\n            m.extra.naturalist_deer_variant = Some(format!("{namespace}:{path}"));\n        }\n    }'),
    ], 'horizon-client')
    edit('game/creatures.rs', [
        ('            spawning::finalize(&mut mob, &facts, &difficulty);',
         '            self.finalize_deer_variant(&mut mob);\n            spawning::finalize(&mut mob, &facts, &difficulty);'),
        ('        let coin = self.bred_coat_coin(parent, p.species());',
         '        let coin = self.bred_coat_coin(parent, p.species());\n        let deer_variant = if p.kind.id == "naturalist:deer" {\n            let other = q.as_ref().filter(|q| q.kind.id == "naturalist:deer");\n            let choose_other = other.is_some() && self.mobs.iter_mut()\n                .find(|m| m.id == parent).is_some_and(|m| m.random.next_boolean());\n            if choose_other { other.and_then(|q| q.extra.naturalist_deer_variant.clone()) }\n            else { p.extra.naturalist_deer_variant.clone() }\n        } else { None };'),
        ('            child.extra.age = -24000;\n            // No Mob.finalizeSpawn either:',
         '            child.extra.age = -24000;\n            child.extra.naturalist_deer_variant = deer_variant;\n            // No Mob.finalizeSpawn either:'),
    ], 'horizon-client')
    edit('game/persist.rs', [
        ('            horizon_client::mobs::spawning::finalize(&mut m, &facts, &difficulty);',
         '            self.finalize_deer_variant(&mut m);\n            horizon_client::mobs::spawning::finalize(&mut m, &facts, &difficulty);'),
    ], 'horizon-client')
    edit('game/natural_spawning.rs', [
        ('        horizon_client::mobs::spawning::finalize(&mut mob, &facts, &difficulty);',
         '        self.finalize_deer_variant(&mut mob);\n        horizon_client::mobs::spawning::finalize(&mut mob, &facts, &difficulty);'),
    ], 'horizon-client')
    edit('game/spawner.rs', [
        ('            spawning::finalize(&mut mob, &facts, &difficulty);',
         '            self.finalize_deer_variant(&mut mob);\n            spawning::finalize(&mut mob, &facts, &difficulty);'),
    ], 'horizon-client')
    edit('game/lan.rs', [
        ('    mob_variant_meta(m, sync, meta, defaults);',
         '    mob_variant_meta(m, sync, meta, defaults);\n    if m.kind.id == "naturalist:deer" {\n        let default = "naturalist:deer";\n        let variant = m.extra.naturalist_deer_variant.as_deref().unwrap_or(default);\n        defaults.push((18, serializer::STRING, Value::String(default.into())));\n        if variant != default {\n            meta.push((18, serializer::STRING, Value::String(variant.into())));\n        }\n    }'),
    ], 'horizon-client')
    edit('game/remote_mob_data.rs', [
        ("        match (*index, value) {\n            // Entity's signed air",
         '        match (*index, value) {\n            (18, Value::String(variant)) if m.kind.id == "naturalist:deer" => {\n                m.extra.naturalist_deer_variant = Some(variant.clone());\n            }\n            // Entity\'s signed air'),
    ], 'horizon-client')
    edit('mobs.rs', [
        ('    pub naturalist_deer_variant: Option<String>,',
         '    pub naturalist_deer_variant: Option<String>,\n    pub naturalist_deer_texture: Option<String>,\n    pub naturalist_fawn_texture: Option<String>,'),
    ])
    edit('game/lan.rs', [
        ('        let mut states = mob_states::Later::new(self.world.game_time);',
         '        let mut states = mob_states::Later::new(self.world.game_time);\n        let mut deer = Vec::new();'),
        ('            states.state((slot, index), m, player_entity(), sync, sources, e);',
         '            states.state((slot, index), m, player_entity(), sync, sources, e);\n            if kind == "naturalist:deer" {\n                deer.push((slot, self.deer_texture(m, false), self.deer_texture(m, true)));\n            }'),
        ('        states.fill(&self.mobs, hub.sync_data(), self.attr_sources(), out);',
         '        states.fill(&self.mobs, hub.sync_data(), self.attr_sources(), out);\n        for (slot, adult, fawn) in deer {\n            let meta = &mut out.as_mut_slice()[slot].meta;\n            meta.push((19, serializer::STRING, Value::String(adult.into())));\n            meta.push((20, serializer::STRING, Value::String(fawn.into())));\n        }'),
    ])
    edit('game/remote_mob_data.rs', [
        ('                m.extra.naturalist_deer_variant = Some(variant.clone());\n            }',
         '                m.extra.naturalist_deer_variant = Some(variant.clone());\n            }\n            (19, Value::String(texture)) if m.kind.id == "naturalist:deer" => {\n                m.extra.naturalist_deer_texture = Some(texture.clone());\n            }\n            (20, Value::String(texture)) if m.kind.id == "naturalist:deer" => {\n                m.extra.naturalist_fawn_texture = Some(texture.clone());\n            }'),
    ])
    edit('game/creatures/spawn_eggs.rs', [
        ('        let target = self.mob_entity(&self.mobs[index]);',
         '        let deer_variant = parent.extra.naturalist_deer_variant.clone();\n        let target = self.mob_entity(&self.mobs[index]);'),
        ('            horizon_client::mobs::finalize::spawn_egg_offspring(child, species, variant, color);',
         '            child.extra.naturalist_deer_variant = deer_variant;\n            horizon_client::mobs::finalize::spawn_egg_offspring(child, species, variant, color);'),
    ])
    shutil.copyfile(ROOT / "src/deer.rs", engine / "crates/horizon-client/src/game/naturalist_deer.rs")
    shutil.copyfile(ROOT / "src/deer_models.rs", engine / "crates/horizon-client/src/game/deer_models.rs")
    shutil.copyfile(ROOT / "src/deer_animations.rs", engine / "crates/horizon-client/src/game/deer_animations.rs")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("jar", type=Path)
    parser.add_argument("pack", type=Path)
    parser.add_argument("--engine", type=Path)
    args = parser.parse_args()
    pack_deer(args.jar, args.pack)
    if args.engine:
        stage_engine(args.engine)
