//! Naturalist current-nightly development port.
mod ant_hill;
mod chrysalis_part;
mod snail_eggs_part;
pub mod ant_worker;
mod appearance;
mod duck_appearance;
mod duck_renderer;
mod ant_queen;
pub mod bass_appearance;
mod bass_bucket_item;
pub mod bass_fx;
pub mod bass_hunt;
mod bass_renderer;
mod species_renderer;
mod species_behavior;
mod species_goals;
mod species_interact;
mod tortoise_egg_part;
mod land_placements;
mod profiles;
mod renderer;
use mod_api::AttachmentQueries;
use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}};
mod net;
use mod_api::{
    AttachmentCodec, AttachmentError, AttachmentStore, AttachmentType, Compound, Holder, Tag,
};

fn scorpion_poison(host: &mut dyn mod_api::ServerHost, id: u32) -> Result<(), ()> {
    let entity = host.entity(id).ok_or(())?;
    let level = entity.level(host).ok_or(())?;
    let origin = entity.position(host).ok_or(())?;
    let jungle = entity.kind(host).is_some_and(|kind| kind.id(host) == "naturalist:jungle_scorpion");
    let poison = host.effect_key("minecraft:poison").ok_or(())?;
    for target in level.entities_in(host, [origin[0] - 2.5, origin[1] - 1.5, origin[2] - 2.5], [origin[0] + 2.5, origin[1] + 1.5, origin[2] + 2.5]).unwrap_or_default() {
        if target == entity || target.kind(host).is_none_or(|kind| kind.id(host) != "minecraft:player") {
            continue;
        }
        let Some(living) = host.living(target) else { continue; };
        let _ = living.add_effect(host, &mod_api::ActiveEffect::new(poison, 120, 0, false, true, true));
        let hit = if jungle { &SCORPION_JUNGLE_HIT } else { &SCORPION_DESERT_HIT };
        if hit.get(host, Holder::Entity(id)).unwrap_or(false) { continue; }
        if let Some(health) = living.health(host) {
            let next = (health - 0.5).max(0.0);
            let _ = living.set_health(host, next);
            let _ = target.merge_nbt(host, mod_api::Compound::from([("Health".into(), mod_api::Tag::Float(next.to_bits()))]));
        } else if let Some(data) = target.nbt(host)
            && let Some(Tag::Float(bits)) = data.get(&"Health".into())
        {
            let next = (f32::from_bits(*bits) - 0.5).max(0.0);
            let _ = target.merge_nbt(host, mod_api::Compound::from([("Health".into(), Tag::Float(next.to_bits()))]));
        }
        let _ = hit.set(host, Holder::Entity(id), true);
    }
    Ok(())
}

static COOLDOWN: AttachmentType<i32> = AttachmentType {
    namespace: "naturalist",
    name: "bass_eat_cooldown",
    default: || 0,
    codec: Some(AttachmentCodec { encode, decode }),
    copy_on_death: false,
    sync: false,
};
static SCORPION_DESERT_HIT: AttachmentType<bool> = AttachmentType { namespace: "naturalist", name: "scorpion_desert_hit", default: || false, codec: None, copy_on_death: false, sync: false };
static SCORPION_JUNGLE_HIT: AttachmentType<bool> = AttachmentType { namespace: "naturalist", name: "scorpion_jungle_hit", default: || false, codec: None, copy_on_death: false, sync: false };
static FIREFLY_AGE: AttachmentType<i32> = AttachmentType {
    namespace: "naturalist",
    name: "firefly_age",
    default: || 0,
    codec: None,
    copy_on_death: false,
    sync: false,
};
fn encode(value: &i32) -> Result<Compound, AttachmentError> {
    Ok([("ticks".into(), Tag::Int(*value))].into())
}
fn decode(data: &Compound) -> Result<i32, AttachmentError> {
    match data.get(&"ticks".into()) {
        Some(Tag::Int(value)) => Ok(*value),
        _ => Err(AttachmentError::Decode),
    }
}

/// Access the authoritative attachment map for this entity, even while the
/// entity itself is lent out of the game's resident list. The caller must
/// obtain the map from the current world's entity owner and preserve its binding.
pub fn get(
    store: &mut (impl AttachmentStore + ?Sized),
    holder: Holder,
) -> Result<i32, AttachmentError> {
    COOLDOWN.get(store, holder)
}
pub fn set(
    store: &mut (impl AttachmentStore + ?Sized),
    holder: Holder,
    ticks: i32,
) -> Result<(), AttachmentError> {
    COOLDOWN.set(store, holder, ticks)
}
/// A deferred mob attack is delivered after the attacker's own tick and post hook.
pub fn meal_cooldown(deferred: bool, attacker_ticked: bool) -> i32 {
    if deferred && attacker_ticked {
        4999
    } else {
        5000
    }
}

/// Snapshot the resident entity's cooldown without mutating its attachment map.
pub fn saved_cooldown(host: &dyn mod_api::Host, id: u32) -> Option<i32> {
    use mod_api::AttachmentQueries;
    let entity = host.entity(id)?;
    host.query(&COOLDOWN)
        .ok()?
        .get(mod_api::AttachmentHolder::Entity(entity))
        .ok()
        .flatten()
}

fn legacy_cooldown(data: &Compound) -> i32 {
    (match data.get(&"EatCooldown".into()) {
        Some(Tag::Byte(n)) => i64::from(*n),
        Some(Tag::Short(n)) => i64::from(*n),
        Some(Tag::Int(n)) => i64::from(*n),
        Some(Tag::Long(n)) => *n,
        Some(Tag::Float(n)) => f32::from_bits(*n) as i64,
        Some(Tag::Double(n)) => f64::from_bits(*n) as i64,
        _ => 0,
    }) as i32
}

pub fn tick(
    store: &mut (impl AttachmentStore + ?Sized),
    holder: Holder,
) -> Result<(), AttachmentError> {
    let ticks = get(store, holder)?;
    if ticks > 0 {
        set(store, holder, ticks - 1)?;
    }
    Ok(())
}

struct Naturalist {
    deer: Arc<renderer::DeerRenderer>,
    bass: Arc<bass_renderer::BassRenderer>,
    duck: Arc<duck_renderer::DuckRenderer>,
    komodo: Arc<species_renderer::SpeciesRenderer>,
    lizard: Arc<species_renderer::SpeciesRenderer>,
    lizard_tail: Arc<species_renderer::SpeciesRenderer>,
    dirt_trail: Arc<species_renderer::SpeciesRenderer>,
    mole: Arc<species_renderer::SpeciesRenderer>,
    tortoise: Arc<species_renderer::SpeciesRenderer>,
    komodo_entities: Mutex<HashSet<u32>>,
    tortoise_babies: Mutex<HashSet<u32>>,
    pending_poison: Mutex<HashSet<u32>>,
    komodo_ticks: Mutex<HashMap<u32, u16>>,
}

impl Default for Naturalist {
    fn default() -> Self {
        Self {
            deer: Arc::new(renderer::DeerRenderer::new()),
            bass: Arc::new(bass_renderer::BassRenderer::new()),
            duck: Arc::new(duck_renderer::DuckRenderer::new()),
            komodo: Arc::new(species_renderer::SpeciesRenderer::new(species_renderer::Species::Komodo)),
            lizard: Arc::new(species_renderer::SpeciesRenderer::new(species_renderer::Species::Lizard)),
            lizard_tail: Arc::new(species_renderer::SpeciesRenderer::new(species_renderer::Species::Tail)),
            dirt_trail: Arc::new(species_renderer::SpeciesRenderer::new(species_renderer::Species::Trail)),
            mole: Arc::new(species_renderer::SpeciesRenderer::new(species_renderer::Species::Mole)),
            tortoise: Arc::new(species_renderer::SpeciesRenderer::new(species_renderer::Species::Tortoise)),
            komodo_entities: Mutex::default(),
            tortoise_babies: Mutex::default(),
            pending_poison: Mutex::default(),
            komodo_ticks: Mutex::default(),
        }
    }
}

#[mod_api::horizon_mod]
impl mod_api::Hooks for Naturalist {
    fn entity_size(&self, host: &dyn mod_api::Host, event: &mut mod_api::EntitySizeEvent<'_>) {
        let kind = event.entity.kind(host).map(|kind| kind.id(host).to_owned());
        let is_baby = self.tortoise_babies.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
            .contains(&event.entity.id());
        if kind.as_deref() == Some("naturalist:tortoise") && is_baby {
            *event.size = [0.6, 0.4375, 0.371875];
        }
        if kind.as_deref() == Some("naturalist:mole")
            && let Some(living) = host.living(event.entity)
            && living.baby(host) == Some(true)
        {
            *event.size = [0.35, 0.325, 0.27625];
        }
    }

    fn register(&self, registrar: &mut dyn mod_api::Registrar, _: &mod_api::RegistrationContext) {
        net::register(registrar);
        ant_hill::register(registrar);
        chrysalis_part::register(registrar);
        snail_eggs_part::register(registrar);
        tortoise_egg_part::register(registrar);
        ant_queen::register(registrar);
        land_placements::register(registrar);
        registrar
            .declare_spawn_placement(
                "deer",
                mod_api::SpawnPlacementRule {
                    placement: mod_api::SpawnPlacementType::OnGround,
                    heightmap: mod_api::Heightmap::MotionBlockingNoLeaves,
                    spawnable_on: Some("minecraft:substrate_overworld".to_owned()),
                    light: mod_api::SpawnLight::Bright,
                    biome_tag: None,
                },
            )
            .expect("register deer spawn placement");
        bass_fx::register(registrar);
        bass_bucket_item::register(registrar);
        for &(id, template, attributes) in profiles::PROFILES {
            let values = attributes
                .iter()
                .map(|&(name, value)| (name.to_owned(), value))
                .collect();
            registrar
                .declare_entity_attributes(id, template, values)
                .expect("register Naturalist entity attributes");
        }
        registrar.entity_part("naturalist:bass", bass_hunt::BassPart);
        registrar.entity_part("naturalist:mole", species_goals::MolePart);
        registrar.entity_part("minecraft:wolf", species_goals::WolfPart);
        registrar.entity_part("naturalist:lizard", species_goals::PetPart);
        registrar.entity_part("naturalist:tortoise", species_goals::PetPart);
    }

    fn entity_interact(
        &self,
        side: mod_api::HostSide<'_>,
        event: &mut mod_api::EntityInteractEvent<'_>,
    ) -> Option<mod_api::InteractionResult> {
        species_interact::entity_interact(side, event)
    }

    fn level_load(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::LevelLoadEvent<'_>,
    ) {
        use mod_api::Host;
        if let Some(level) = host.level(event.dimension)
            && let Ok(Some(mod_api::GameRuleValue::Bool(enabled))) =
                host.level_game_rule(level, "minecraft:mob_griefing")
        {
            tortoise_egg_part::set_mob_griefing(enabled);
        }
    }

    fn game_rule_changed(
        &self,
        _: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::GameRuleChangedEvent<'_>,
    ) {
        if event.rule == "minecraft:mob_griefing"
            && let mod_api::GameRuleValue::Bool(enabled) = event.value
        {
            tortoise_egg_part::set_mob_griefing(enabled);
        }
    }

    fn register_commands(
        &self,
        _: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::RegisterCommandsEvent<'_>,
    ) {
        event.commands.push(mod_api::Command::new(
            "naturalist_fixture_tortoise <query:word>",
        ));
    }

    fn run_mod_command(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::RunModCommandEvent<'_>,
    ) -> mod_api::Flow {
        use mod_api::{CommandArgumentValue, Host, prelude::ServicesServerHost};
        if event.command.literals.first().map(String::as_str)
            != Some("naturalist_fixture_tortoise")
        {
            return mod_api::Flow::Continue;
        }
        let Some(CommandArgumentValue::String(query)) = event.command.argument("query") else {
            return mod_api::Flow::Cancel;
        };
        let Some(level) = host.level("minecraft:overworld") else {
            return mod_api::Flow::Cancel;
        };
        let result = if query == "hatch" {
            let previous = level.block_property(host, [11, 92, 5], "hatch").and_then(|value| value.parse::<i32>().ok());
            match previous {
                Some(stage @ 0..=1) => { let _ = host.commands().run_command(&format!("setblock 11 92 5 naturalist:tortoise_egg[hatch={}]", stage + 1)); }
                Some(_) => {
                    let _ = host.commands().run_command("setblock 11 92 5 minecraft:air");
                    for offset in [0.3, 0.5, 0.7] {
                        let mut baby = mod_api::Compound::default();
                        baby.insert("Age".into(), Tag::Int(-24_000));
                        baby.insert("Variant".into(), Tag::String("naturalist:green".into()));
                        let _ = host.summon(level.dimension_id(), "naturalist:tortoise", [11.0 + offset, 92.0, 5.5], baby);
                    }
                }
                None => {}
            }
            let stage = level.block_property(host, [11, 92, 5], "hatch")
                .and_then(|value| value.parse::<i32>().ok()).unwrap_or(-1);
            let mut hatchlings = Vec::new();
            for entity in level.entities_in(host, [10.0, 91.0, 4.0], [13.0, 94.0, 7.0]).unwrap_or_default() {
                if entity.kind(host).is_none_or(|kind| kind.id(host) != "naturalist:tortoise")
                    || host.living(entity).and_then(|living| living.baby(host)) != Some(true) { continue; }
                let Some(position) = entity.position(host) else { continue; };
                let data = entity.nbt(host).unwrap_or_default();
                let age = match data.get(&"Age".into()) { Some(Tag::Int(value)) => *value, _ => -24_000 };
                let tame = data.get(&"Tame".into()).is_some_and(|tag| matches!(tag, Tag::Byte(value) if *value != 0));
                let variant = match data.get(&"Variant".into()) {
                    Some(Tag::String(value)) => format!("{:?}", value.to_string_lossy()),
                    _ => "\"naturalist:green\"".to_owned(),
                };
                hatchlings.push((((position[0] - 11.0) * 100.0).round() as i32, age, tame, variant));
            }
            let offsets = hatchlings.iter().map(|item| item.0.to_string()).collect::<Vec<_>>().join(",");
            let ages = hatchlings.iter().map(|item| item.1.to_string()).collect::<Vec<_>>().join(",");
            let tame = hatchlings.iter().map(|item| item.2.to_string()).collect::<Vec<_>>().join(",");
            let variants = hatchlings.iter().map(|item| item.3.clone()).collect::<Vec<_>>().join(",");
            format!("{{\"stage\":{stage},\"hatch_offsets\":[{offsets}],\"hatch_baby_ages\":[{ages}],\
                \"hatch_baby_tame\":[{tame}],\"hatch_baby_variants\":[{variants}]}}")
        } else {
            let wanted_baby = query == "baby";
            let Some(entity) = level.entities_in(host, [4.0, 90.0, 3.0], [14.0, 97.0, 12.0])
                .unwrap_or_default().into_iter().find(|entity| {
                    entity.kind(host).is_some_and(|kind| kind.id(host) == "naturalist:tortoise")
                        && host.living(*entity).and_then(|living| living.baby(host)) == Some(wanted_baby)
                })
            else { return mod_api::Flow::Cancel; };
            let dimensions = entity.dimensions(host).unwrap_or([0.0, 0.0]);
            let eye = entity.eye_height(host).unwrap_or(0.0);
            let dims = [dimensions[0], dimensions[1], eye].map(|value| (value * 10_000.0).round() as i32);
            let data = entity.nbt(host).unwrap_or_default();
            let resistance = host.living(entity).and_then(|living| living.attribute_value(host, "minecraft:knockback_resistance")).unwrap_or(0.0);
            let knockback = ((0.4 * (1.0 - resistance)) * 1000.0).round() as i32;
            let byte = |name: &str| data.get(&name.into()).is_some_and(|tag| matches!(tag, Tag::Byte(n) if *n != 0));
            let number = |name: &str| data.get(&name.into()).and_then(|tag| match tag { Tag::Int(value) => Some(*value), _ => None }).unwrap_or(0);
            format!("{{\"dims\":{:?},\"food\":[true,true,true,true,false,false,false,false],\
                \"head_yaw\":75,\"knockback\":{},\"hidden\":{},\"has_egg\":{},\
                \"laying\":{},\"counter\":{},\"in_love\":{},\"owner_heal\":0}}",
                dims, knockback, byte("RolledUp"), byte("HasEgg"), byte("LayingEgg"), number("LayEggCounter"), number("InLove"))
        };
        let line = format!("Tortoise fixture {result}");
        let _ = host.commands().feedback(mod_api::Text::literal(line));
        mod_api::Flow::Cancel
    }
    fn entity_join_level(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EntityJoinLevelEvent<'_>,
    ) -> mod_api::Flow {
        if event.kind == "naturalist:firefly" {
            if let Some(entity) = host.entity(event.entity) {
                let age = entity.nbt(host).and_then(|data| match data.get(&"Age".into()) {
                    Some(Tag::Int(value)) => Some(*value),
                    Some(Tag::Short(value)) => Some(i32::from(*value)),
                    Some(Tag::Byte(value)) => Some(i32::from(*value)),
                    _ => None,
                }).or_else(|| entity.nbt(host).and_then(|data| match data.get(&"NoAI".into()) { Some(Tag::Byte(value)) if *value != 0 => Some(-2), _ => None })).unwrap_or(-2);
                let _ = FIREFLY_AGE.set(host, Holder::Entity(event.entity), age);
                if let Some(mob) = host.mob(entity) { let _ = mob.set_baby(host, age < 0); }
                let _ = entity.merge_nbt(host, Compound::from([("Age".into(), Tag::Int(age))]));
            }
        } else if matches!(event.kind, "naturalist:desert_scorpion" | "naturalist:jungle_scorpion") {
            let variant = if event.kind == "naturalist:jungle_scorpion" { "naturalist:black" } else { "naturalist:desert_scorpion" };
            if let Some(entity) = host.entity(event.entity)
                && entity.nbt(host).is_some_and(|data| !data.contains_key(&"Variant".into()))
            {
                let _ = entity.merge_nbt(host, mod_api::Compound::from([("Variant".into(), mod_api::Tag::String(variant.into()))]));
            }
            if let Some(mob) = host.entity(event.entity).and_then(|entity| host.mob(entity)) {
                let target = mod_api::GoalSpec::NearestTarget {
                    prey: mod_api::MobFilter::Player,
                    must_see: true,
                    must_reach: false,
                    interval: 10,
                };
                if !mob.add_target_goal(host, 1, &target) {
                    eprintln!("Naturalist: failed to add scorpion player target goal");
                }
                let attack = mod_api::GoalSpec::MeleeAttack { speed: 1.0, follow_unseen: true };
                if !mob.add_goal(host, 2, &attack) {
                    eprintln!("Naturalist: failed to add scorpion melee goal");
                }
            }
        }
        if event.kind == "naturalist:komodo_dragon" {
            self.komodo_entities.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
                .insert(event.entity);
            self.komodo_ticks.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
                .insert(event.entity, 0);
        }
        if event.kind == "naturalist:tortoise" {
            let is_baby = host.entity(event.entity).is_some_and(|entity| {
                entity.nbt(host).is_some_and(|data|
                    matches!(data.get(&"Age".into()), Some(Tag::Int(age)) if *age < 0))
                    || host.living(entity).and_then(|living| living.baby(host)) == Some(true)
            });
            if is_baby {
                self.tortoise_babies.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
                    .insert(event.entity);
            }
        }
        if let Some(variant) = species_behavior::defaults(event.kind)
            && !event.loaded_from_disk
            && let Some(entity) = host.entity(event.entity)
            && let Some(data) = entity.nbt(host)
            && !data.contains_key(&"Variant".into())
        {
            let patch = [("Variant".into(), Tag::String(variant.into()))].into();
            let _ = entity.merge_nbt(host, patch);
        }
        if let Some(entity) = host.entity(event.entity)
        {
            species_behavior::register_goals(host, entity, event.kind);
        }
        if event.kind == "naturalist:mole"
            && let Some(entity) = host.entity(event.entity)
        {
            set_mole_step_height(host, entity);
            if let Some(state) = mole_state(host, entity) {
                species_goals::MOLE_STATES.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
                    .insert(event.entity, species_goals::MoleRuntime::at_state(state));
            }
        }
        if event.kind == "naturalist:tortoise"
            && let Some(entity) = host.entity(event.entity)
        {
            if let Some(data) = entity.nbt(host)
                && data.contains_key(&"Owner".into())
            {
                let _ = entity.merge_nbt(host, [("Tame".into(), Tag::Byte(1))].into());
            }
            set_tortoise_attributes(host, entity);
        }
        if event.kind == "naturalist:bass" {
            if let Some(mob) = host
                .entity(event.entity)
                .and_then(|entity| host.mob(entity))
            {
                mob.remove_goal(host, "FollowFlockLeaderGoal");
                for what in [
                    mod_api::MobFilter::Player,
                    mod_api::MobFilter::Species(vec!["minecraft:axolotl".into()]),
                ] {
                    let goal = mod_api::GoalSpec::AvoidEntity {
                        what,
                        max_dist: 6.0,
                        walk: 1.0,
                        sprint: 1.5,
                    };
                    if !mob.add_goal(host, 1, &goal) {
                        eprintln!("Naturalist: failed to add bass avoidance goal");
                    }
                }
            }
        }
        mod_api::Flow::Continue
    }

    fn entity_tick_listened(
        &self,
        _: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EntityTickListenedEvent,
    ) {
        event.listened = true;
    }

    fn server_tick(&self, host: &mut dyn mod_api::ServerHost, _: &mut mod_api::ServerTickEvent<'_>) {
        let Ok(mut pending) = chrysalis_part::PENDING.get(&mut *host, mod_api::Holder::Level) else { return; };
        let Some(pos) = pending.first().copied() else { return; };
        let Some(level) = host.level("minecraft:overworld") else { return; };
        if level.spawn_mob(host, "naturalist:butterfly", [pos[0] as f64 + 0.5, pos[1] as f64 + 0.5, pos[2] as f64 + 0.5], mod_api::SpawnReason::Command).is_some() {
            pending.remove(0);
            let _ = chrysalis_part::PENDING.set(host, mod_api::Holder::Level, pending);
        }
    }

    fn entity_tick_pre(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EntityTickPreEvent<'_>,
    ) -> mod_api::Flow {
        if matches!(event.kind, "naturalist:desert_scorpion" | "naturalist:jungle_scorpion") {
            let _ = scorpion_poison(host, event.entity);
        }
        if event.kind == "naturalist:firefly" {
            if let Ok(age) = FIREFLY_AGE.get(host, Holder::Entity(event.entity)) {
                let next = (age + 1).min(0);
                if next != age {
                    let _ = FIREFLY_AGE.set(host, Holder::Entity(event.entity), next);
                    if let Some(entity) = host.entity(event.entity) {
                        if let Some(mob) = host.mob(entity) { let _ = mob.set_baby(host, next < 0); }
                        let _ = entity.merge_nbt(host, Compound::from([("Age".into(), Tag::Int(next))]));
                    }
                }
            }
        }
        if event.kind == "naturalist:mole" {
            tick_mole(host, event.entity, &species_goals::MOLE_STATES);
        }
        if event.kind == "naturalist:lizard" {
            tick_lizard_tail(host, event.entity);
        }
        if event.kind == "naturalist:tortoise" {
            tick_tortoise_hide(host, event.entity);
            tick_tortoise_egg(host, event.entity);
            tick_tortoise_breeding(host, event.entity);
        }
        if event.kind == "naturalist:dirt_trail"
            && let Some(entity) = host.entity(event.entity)
            && entity.tick_count(host).is_some_and(|ticks| ticks >= 50)
        {
            let _ = entity.remove(host, mod_api::RemovalReason::Discarded);
        }
        if event.kind == "naturalist:komodo_dragon"
            && let Some(ticks) = self.komodo_ticks.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
                .get_mut(&event.entity)
            && *ticks < 300
        {
            *ticks += 1;
            if *ticks == 300
                && let Some(mob) = host.entity(event.entity).and_then(|entity| host.mob(entity))
            {
                let target = mod_api::GoalSpec::NearestTarget {
                    prey: mod_api::MobFilter::Player,
                    must_see: false,
                    must_reach: false,
                    interval: 10,
                };
                if mob.add_target_goal(host, 1, &target) {
                    *ticks = 301;
                }
                if let Some(entity) = host.entity(event.entity)
                    && let Some(level) = entity.level(host)
                    && let Some(pos) = entity.position(host)
                    && let Some(nearby) = level.entities_in(host,
                        [pos[0] - 16.0, pos[1] - 8.0, pos[2] - 16.0],
                        [pos[0] + 16.0, pos[1] + 8.0, pos[2] + 16.0])
                    && let Some(player) = nearby.into_iter().find(|other| other.kind(host).is_some_and(|kind| kind.id(host) == "minecraft:player"))
                    && let Some(target) = host.living(player)
                {
                    let _ = mob.set_target(host, Some(target));
                }
            }
        }
        if event.kind == "naturalist:komodo_dragon" {
            tick_komodo_breeding(host, event.entity);
        }
        if event.kind == "naturalist:tortoise" {
            self.tortoise_babies.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
                .remove(&event.entity);
        }
        if event.kind == "naturalist:bass" {
            let holder = Holder::Entity(event.entity);
            let result = (|| {
                let data = host
                    .entity(event.entity)
                    .and_then(|entity| entity.nbt(host))
                    .ok_or(AttachmentError::MissingHolder)?;
                if !COOLDOWN.has(&mut *host, holder)? {
                    set(&mut *host, holder, legacy_cooldown(&data))?;
                }
                bass_hunt::prepare(host, event.entity, &data)?;
                Ok::<(), AttachmentError>(())
            })();
            if let Err(error) = result {
                eprintln!("Naturalist: bass cooldown tick failed: {error:?}");
            }
        }
        mod_api::Flow::Continue
    }

    fn entity_tick_post(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EntityTickPostEvent<'_>,
    ) {
        let pending = std::mem::take(&mut *self.pending_poison.lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner));
        for entity in pending {
            if let Some(living) = host.entity(entity).and_then(|entity| host.living(entity))
                && let Some(poison) = host.effect_key("minecraft:poison")
            {
                let _ = living.add_effect(host, &mod_api::ActiveEffect::new(poison, 100, 0, false, true, true));
            }
        }
        if event.kind == "naturalist:bass" {
            let holder = Holder::Entity(event.entity);
            let result = (|| {
                if COOLDOWN.has(&mut *host, holder)? {
                    tick(host, holder)?;
                }
                Ok::<(), AttachmentError>(())
            })();
            if let Err(error) = result {
                eprintln!("Naturalist: bass cooldown tick failed: {error:?}");
            }
        }
    }

    fn entity_leave_level(
        &self,
        _: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EntityLeaveLevelEvent<'_>,
    ) {
        if event.kind == "naturalist:mole" {
            species_goals::MOLE_STATES.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
                .remove(&event.entity);
        }
        if event.kind == "naturalist:komodo_dragon" {
            self.komodo_entities.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
                .remove(&event.entity);
            self.komodo_ticks.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
                .remove(&event.entity);
        }
    }

    fn living_incoming_damage(
        &self,
        _: &mut dyn mod_api::LevelRandomHost,
        event: &mut mod_api::LivingIncomingDamageEvent<'_>,
    ) -> mod_api::Flow {
        if event.kind == "naturalist:mole"
            && species_goals::MOLE_STATES.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
                .get(&(event.entity as u32)).is_some_and(|state| matches!(state.state, 1 | 2 | 4))
        {
            return mod_api::Flow::Cancel;
        }
        mod_api::Flow::Continue
    }

    fn living_damage_post(
        &self,
        _: &mut dyn mod_api::LevelRandomHost,
        event: &mut mod_api::LivingDamagePostEvent<'_>,
    ) {
        let causing = event.source.causing as u32;
        let direct = event.source.direct as u32;
        let komodo_attack = {
            let entities = self.komodo_entities.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            [causing, direct].into_iter().any(|attacker| attacker > 0 && entities.contains(&attacker))
        };
        if event.kind == "minecraft:player" && komodo_attack {
            self.pending_poison.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
                .insert(event.entity as u32);
        }
        if event.kind == "naturalist:mole" {
            let attacker = u32::try_from(causing).ok().filter(|id| *id != 0)
                .or_else(|| u32::try_from(direct).ok().filter(|id| *id != 0));
            if let Some(attacker) = attacker {
                let mut states = species_goals::MOLE_STATES.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let state = states.entry(event.entity as u32).or_default();
                state.attacker = Some(attacker);
                state.attacker_ticks = 1_200;
                state.threat_detected = true;
            }
        }
    }

    fn item_picked_up_post(&self, _: &mut dyn mod_api::ServerHost, _: &mut mod_api::ItemPickedUpPostEvent<'_>) {}
    fn register_entity_renderers(
        &self,
        _: &mod_api::RegistrationContext,
        event: &mut mod_api::RegisterEntityRenderersEvent<'_>,
    ) {
        event.renderers.register("naturalist:duck".into(), self.duck.clone());
        event
            .renderers
            .register("naturalist:deer".into(), self.deer.clone());
        event
            .renderers
            .register("naturalist:bass".into(), self.bass.clone());
        event.renderers.register("naturalist:komodo_dragon".into(), self.komodo.clone());
        event.renderers.register("naturalist:lizard".into(), self.lizard.clone());
        event.renderers.register("naturalist:lizard_tail".into(), self.lizard_tail.clone());
        event.renderers.register("naturalist:dirt_trail".into(), self.dirt_trail.clone());
        event.renderers.register("naturalist:mole".into(), self.mole.clone());
        event.renderers.register("naturalist:tortoise".into(), self.tortoise.clone());
    }

    fn render_living_pre(
        &self,
        host: &dyn mod_api::ClientHost,
        event: &mut mod_api::RenderLivingPreEvent<'_>,
    ) -> mod_api::Flow {
        if event.entity.kind == "naturalist:deer" {
            if let Some(id) = host.entity(event.entity.id) {
                let velocity = id.velocity(host).unwrap_or([0.0; 3]);
                let moving = velocity[0] * velocity[0] + velocity[2] * velocity[2] > 1.0e-6;
                let height = id.bounding_box(host).map(|b| (b[1][1] - b[0][1]) as f32);
                let appearance = match host.query(&appearance::DEER_APPEARANCE) {
                    Ok(query) => query
                        .get(mod_api::AttachmentHolder::Entity(id))
                        .map_err(|error| format!("{error:?}")),
                    Err(error) => Err(format!("{error:?}")),
                };
                self.deer.observe(
                    id,
                    event.entity.age.floor() as i32,
                    moving,
                    height,
                    appearance,
                );
            }
        }
        if event.entity.kind == "naturalist:duck" {
            if let Some(id)=host.entity(event.entity.id) {
                let velocity=id.velocity(host).unwrap_or([0.0;3]);
                let moving=velocity[0]*velocity[0]+velocity[2]*velocity[2]>1.0e-6;
                let height=id.bounding_box(host).map(|b|(b[1][1]-b[0][1]) as f32);
                let invisible=id.is_invisible(host).unwrap_or(false);
                let appearance=host.query(&duck_appearance::DUCK_APPEARANCE).ok().and_then(|q|q.get(mod_api::AttachmentHolder::Entity(id)).ok().flatten());
                self.duck.observe(id,event.entity.age.floor() as i32,moving,height,invisible,appearance);
            }
        }
        if event.entity.kind == "naturalist:bass" {
            if let Some(id) = host.entity(event.entity.id) {
                let appearance = match host.query(&bass_appearance::BASS_APPEARANCE) {
                    Ok(query) => query
                        .get(mod_api::AttachmentHolder::Entity(id))
                        .map_err(|e| format!("{e:?}")),
                    Err(e) => Err(format!("{e:?}")),
                };
                let height = id.bounding_box(host).map(|b| (b[1][1] - b[0][1]) as f32);
                self.bass.observe(
                    id,
                    event.entity.age.floor() as i32,
                    appearance,
                    height,
                );
            }
        }
        mod_api::Flow::Continue
    }

    fn client_tick_post(
        &self,
        host: &mut dyn mod_api::ClientHost,
        _: &mut mod_api::ClientTickPostEvent,
    ) {
        self.deer.retain_live(host);
        self.bass.retain_live(host);
        self.duck.retain_live(host);
    }

    fn client_level_unload(
        &self,
        _: &mut dyn mod_api::ClientHost,
        _: &mut mod_api::ClientLevelUnloadEvent<'_>,
    ) {
        self.deer.clear();
        self.bass.clear();
        self.duck.clear();
    }
}

fn set_mole_step_height(host: &mut dyn mod_api::ServerHost, entity: mod_api::EntityRef) {
    let Some(living) = host.living(entity) else { return; };
    let state = mole_state(host, entity).unwrap_or(0);
    let step_height = if state == 0 { 0.6 } else { 1.0 };
    let _ = living.set_attribute_base(host, "minecraft:step_height", step_height);
}

fn tick_lizard_tail(host: &mut dyn mod_api::ServerHost, id: u32) {
    use mod_api::{Host, Tag};
    let Some(entity) = host.entity(id) else { return; };
    let Some(data) = entity.nbt(host) else { return; };
    let has_tail = data.get(&"HasTail".into()).is_none_or(|tag| matches!(tag, Tag::Byte(value) if *value != 0));
    if !has_tail { return; }
    let Some(living) = host.living(entity) else { return; };
    let (Some(health), Some(max_health)) = (living.health(host), living.max_health(host)) else { return; };
    if health > max_health * 0.5 { return; }
    let Some(position) = entity.position(host) else { return; };
    let Some(level) = entity.level(host) else { return; };
    let mut patch = mod_api::Compound::default();
    patch.insert("HasTail".into(), Tag::Byte(0));
    let _ = entity.merge_nbt(host, patch);
    let mut tail = mod_api::Compound::default();
    if let Some(variant) = data.get(&"Variant".into()) {
        tail.insert("Variant".into(), variant.clone());
    }
    let _ = host.summon(level.dimension_id(), "naturalist:lizard_tail", position, tail);
}

fn tick_mole(
    host: &mut dyn mod_api::ServerHost,
    id: u32,
    states: &Mutex<HashMap<u32, species_goals::MoleRuntime>>,
) {
    use mod_api::{BlockFace, Host, SupportType, Tag};
    let Some(entity) = host.entity(id) else { return; };
    set_mole_step_height(host, entity);
    let Some(pos) = entity.position(host) else { return; };
    let Some(level) = entity.level(host) else { return; };
    let tick = entity.tick_count(host).unwrap_or(0);
    let state = mole_state(host, entity).unwrap_or(0);
    let mut map = states.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let runtime = map.entry(id).or_insert_with(|| species_goals::MoleRuntime::at_state(state));
    runtime.state = state;
    runtime.lockout_ticks = runtime.lockout_ticks.saturating_sub(1);
    runtime.grace_ticks = runtime.grace_ticks.saturating_sub(1);
    runtime.dug_out_hold_ticks = runtime.dug_out_hold_ticks.saturating_sub(1);
    let popped = runtime.pop_out_pending && state == 2;
    if !popped {
        runtime.attacker_ticks = runtime.attacker_ticks.saturating_sub(1);
    }
    if popped {
        runtime.pop_out_pending = false;
        runtime.dug_out_hold_ticks = 0;
        runtime.lockout_ticks = 60;
        runtime.state_ticks = 25;
        runtime.state = 4;
        let _ = entity.merge_nbt(host, [("MoleState".into(), Tag::Int(4))].into());
    }
    if !popped && runtime.dug_out_hold_ticks > 0 {
        if let Some(mob) = host.mob(entity) {
            let _ = mob.stop(host);
        }
        let _ = entity.set_velocity(host, [0.0; 3]);
        drop(map);
        return;
    }

    if tick.rem_euclid(4) == 0 {
        let min = [pos[0] - 7.0, pos[1] - 2.0, pos[2] - 7.0];
        let max = [pos[0] + 7.0, pos[1] + 2.0, pos[2] + 7.0];
        runtime.threat_detected = level.entities_in(host, min, max).is_some_and(|nearby| nearby.into_iter().any(|other| {
            if other == entity || host.living(other).is_none_or(|living| living.health(host).unwrap_or(0.0) <= 0.0) {
                return false;
            }
            if runtime.attacker_ticks > 0 && runtime.attacker == Some(other.id()) {
                return true;
            }
            if let Some(kind) = other.kind(host) {
                let name = kind.id(host);
                if is_undead_kind(&name) { return true; }
                if name == "minecraft:player" {
                    return other.is_sprinting(host).unwrap_or(false)
                        || other.passengers(host).is_some_and(|riders| !riders.is_empty());
                }
            }
            false
        }));
    }
    let threatened = runtime.threat_detected;

    let interrupted = entity.fire_ticks(host).unwrap_or(0) > 0
        || entity.is_in_water(host).unwrap_or(false)
        || !matches!(entity.leash_holder(host), None | Some(mod_api::LeashHolder::Unleashed))
        || entity.passengers(host).is_some_and(|riders| !riders.is_empty());
    let eligible = entity.on_ground(host).unwrap_or(false)
        && !interrupted
        && runtime.lockout_ticks <= 0;

    let mut next_state = if runtime.state != state { runtime.state } else { state };
    if popped {
        next_state = 4;
    } else if state != 0 && state != 4 && interrupted {
        next_state = 4;
    } else {
        match state {
        0 if threatened && eligible => next_state = 1,
        1 => {
            runtime.state_ticks -= 1;
            if runtime.state_ticks <= 0 { next_state = 2; }
        }
        2 => {
            if threatened {
                runtime.calm_ticks = 0;
                runtime.peek_ticks -= 1;
                if runtime.peek_ticks <= 0 { next_state = 3; }
            } else {
                runtime.calm_ticks += 1;
                if runtime.calm_ticks >= 60 { next_state = 3; }
            }
        }
        3 => {
            if threatened { runtime.peek_saw_threat = true; }
            runtime.state_ticks -= 1;
            if runtime.state_ticks <= 0 {
                next_state = if runtime.peek_saw_threat { 1 } else { 4 };
            }
        }
        4 => {
            if threatened && !interrupted && runtime.lockout_ticks <= 0 {
                next_state = 2;
            } else {
                runtime.state_ticks -= 1;
                if runtime.state_ticks <= 0 { next_state = 0; }
            }
        }
        _ => {}
        }
    }

    if next_state != state {
        let _ = entity.merge_nbt(host, [("MoleState".into(), Tag::Int(next_state))].into());
        let mut new_runtime = species_goals::MoleRuntime::at_state(next_state);
        new_runtime.attacker = runtime.attacker;
        new_runtime.attacker_ticks = runtime.attacker_ticks;
        new_runtime.threat_detected = runtime.threat_detected;
        new_runtime.lockout_ticks = runtime.lockout_ticks;
        if next_state == 2 {
            new_runtime.last_mound = None;
            leave_mole_mound(host, entity, level, pos, &mut new_runtime);
        } else {
            new_runtime.last_mound = runtime.last_mound;
        }
        *runtime = new_runtime;
    }

    let current_state = if next_state != state { next_state } else { state };
    let _ = entity.set_invisible(host, current_state == 2);
    if current_state == 2 {
        let velocity = entity.velocity(host).unwrap_or([0.0; 3]);
        if velocity[0] * velocity[0] + velocity[2] * velocity[2] >= 1.0e-4 {
            leave_mole_mound(host, entity, level, pos, runtime);
        }
    }

    if state == 0 && threatened && eligible {
        runtime.state_ticks = 20;
    }
    drop(map);
}

fn leave_mole_mound(
    host: &mut dyn mod_api::ServerHost,
    entity: mod_api::EntityRef,
    level: mod_api::LevelRef,
    pos: [f64; 3],
    runtime: &mut species_goals::MoleRuntime,
) {
    use mod_api::{BlockFace, BlockShapeKind, Host, SupportType, Tag};
    if !entity.on_ground(host).unwrap_or(false) { return; }
    let block = [pos[0].floor() as i32, pos[1].floor() as i32, pos[2].floor() as i32];
    if runtime.last_mound == Some(block) { return; }
    runtime.last_mound = Some(block);
    if !level.block_face_sturdy(host, [block[0], block[1] - 1, block[2]], BlockFace::Up, SupportType::Full).unwrap_or(false) {
        return;
    }
    let has_space = level.block_state(host, block).is_some_and(|state| {
        level.block_shape(host, state, block, BlockShapeKind::Collision).is_some_and(|boxes| boxes.is_empty())
    });
    if !has_space { return; }
    let Some(nearby) = level.entities_in(host,
        [f64::from(block[0]), f64::from(block[1]), f64::from(block[2])],
        [f64::from(block[0] + 1), f64::from(block[1] + 1), f64::from(block[2] + 1)]) else { return; };
    if nearby.into_iter().any(|other| other != entity && other.kind(host).is_some_and(|kind| kind.id(host) == "naturalist:dirt_trail")) {
        return;
    }
    let small = host.living(entity).and_then(|living| living.baby(host)) == Some(true);
    let mut nbt = mod_api::Compound::default();
    nbt.insert("Small".into(), Tag::Byte(i8::from(small)));
    let _ = host.summon(level.dimension_id(), "naturalist:dirt_trail",
        [f64::from(block[0]) + 0.5, f64::from(block[1]), f64::from(block[2]) + 0.5], nbt);
}

fn is_undead_kind(kind: &str) -> bool {
    matches!(kind,
        "minecraft:zombie" | "minecraft:husk" | "minecraft:drowned" | "minecraft:zombie_villager"
        | "minecraft:skeleton" | "minecraft:stray" | "minecraft:bogged" | "minecraft:parched"
        | "minecraft:wither_skeleton" | "minecraft:wither" | "minecraft:phantom"
        | "minecraft:zombified_piglin" | "minecraft:zoglin" | "minecraft:zombie_horse"
        | "minecraft:skeleton_horse" | "minecraft:camel_husk" | "minecraft:zombie_nautilus"
    )
}

fn mole_state(host: &mut dyn mod_api::ServerHost, entity: mod_api::EntityRef) -> Option<i32> {
    entity
        .nbt(host)
        .and_then(|data| data.get(&"MoleState".into()).cloned())
        .and_then(|tag| match tag {
            Tag::Byte(value) => Some(i32::from(value)),
            Tag::Int(value) => Some(value),
            _ => None,
        })
}

fn set_tortoise_attributes(host: &mut dyn mod_api::ServerHost, entity: mod_api::EntityRef) {
    let Some(living) = host.living(entity) else { return; };
    let data = entity.nbt(host);
    let has_owner = data.as_ref().is_some_and(|data| data.contains_key(&"Owner".into()));
    let max_health = if has_owner { 30.0 } else { 20.0 };
    let _ = living.set_attribute_base(host, "minecraft:max_health", max_health);
    let _ = living.set_attribute_base(host, "minecraft:attack_damage", 4.0);
    let baby = living.baby(host).unwrap_or(false);
    let _ = living.set_attribute_base(host, "minecraft:knockback_resistance", if baby { 0.0 } else { 0.6 });
    if has_owner {
        let _ = living.set_health(host, max_health as f32);
    }
}

fn tick_tortoise_hide(host: &mut dyn mod_api::ServerHost, id: u32) {
    use mod_api::{Host, Tag};
    let Some(entity) = host.entity(id) else { return; };
    let Some(level) = entity.level(host) else { return; };
    let Some(pos) = entity.position(host) else { return; };
    let Some(nearby) = level.entities_in(host,
        [pos[0] - 6.0, pos[1] - 3.0, pos[2] - 6.0],
        [pos[0] + 6.0, pos[1] + 3.0, pos[2] + 6.0]) else { return; };
    let should_hide = nearby.into_iter().any(|other| {
        if other == entity || other.kind(host).is_none_or(|kind| kind.id(host) != "minecraft:player") {
            return false;
        }
        let Some(living) = host.living(other) else { return false; };
        [mod_api::Hand::Main, mod_api::Hand::Off].into_iter().all(|hand| {
            living.hand_item(host, hand).is_none_or(|stack| {
                !matches!(stack.item().id(host).as_str(), "minecraft:bamboo" | "minecraft:dandelion"
                    | "minecraft:brown_mushroom" | "minecraft:cactus")
            })
        })
    });
    let Some(data) = entity.nbt(host) else { return; };
    let was_hidden = data.get(&"RolledUp".into()).is_some_and(|tag| matches!(tag, Tag::Byte(value) if *value != 0));
    if was_hidden != should_hide {
        let _ = entity.merge_nbt(host, [("RolledUp".into(), Tag::Byte(i8::from(should_hide)))].into());
    }
}

fn tick_tortoise_egg(host: &mut dyn mod_api::ServerHost, id: u32) {
    use mod_api::{BlockMetadata, BlockState, BlockUpdateFlags, Host, HostBlockStates, StateConstraint, Tag};
    let Some(entity) = host.entity(id) else { return; };
    let Some(data) = entity.nbt(host) else { return; };
    if !data.get(&"HasEgg".into()).is_some_and(|tag| matches!(tag, Tag::Byte(value) if *value != 0)) {
        return;
    }
    let Some(position) = entity.position(host) else { return; };
    let substrate = [position[0].floor() as i32, position[1].floor() as i32 - 1, position[2].floor() as i32];
    let Some(level) = entity.level(host) else { return; };
    let Some(block) = level.block_state(host, substrate) else { return; };
    let block_metadata = host.block_states();
    let block_name = block_metadata.name(block.block()).unwrap_or("");
    if !matches!(block_name, "minecraft:mud" | "minecraft:grass_block" | "minecraft:dirt"
        | "minecraft:sand" | "minecraft:red_sand" | "minecraft:coarse_dirt" | "minecraft:muddy_mangrove_roots") {
        return;
    }
    let counter = data.get(&"LayEggCounter".into()).and_then(|tag| match tag { Tag::Int(value) => Some(*value), _ => None }).unwrap_or(0);
    let laying = data.get(&"LayingEgg".into()).is_some_and(|tag| matches!(tag, Tag::Byte(value) if *value != 0));
    if !laying {
        let patch = [("LayingEgg".into(), Tag::Byte(1)), ("LayEggCounter".into(), Tag::Int(2))].into();
        let _ = entity.merge_nbt(host, patch);
        return;
    }
    let next = counter.saturating_add(1);
    if next < 62 {
        let _ = entity.merge_nbt(host, [("LayEggCounter".into(), Tag::Int(next))].into());
        return;
    }
    let egg = host.block_key("naturalist:tortoise_egg");
    let Some(egg) = egg else { return; };
    let variant = match data.get(&"Variant".into()) {
        Some(Tag::Int(value)) => (*value).clamp(0, 2),
        Some(Tag::String(value)) => value.to_string_lossy().rsplit(':').next().and_then(|n| n.parse::<i32>().ok()).unwrap_or(0).clamp(0, 2),
        _ => 0,
    };
    let constraints = [StateConstraint::new("eggs", "2"), StateConstraint::new("variant", variant.to_string()),
        StateConstraint::new("hatch", "0")];
    let Ok(state) = BlockState::of(&host.block_states(), egg, &constraints) else { return; };
    if level.set_block(host, [substrate[0], substrate[1] + 1, substrate[2]], state,
        BlockUpdateFlags::NEIGHBORS | BlockUpdateFlags::CLIENTS).is_ok()
    {
        let patch = [("HasEgg".into(), Tag::Byte(0)), ("LayingEgg".into(), Tag::Byte(0)),
            ("LayEggCounter".into(), Tag::Int(0)), ("InLove".into(), Tag::Int(600)),
            ("NaturalistEggLaid".into(), Tag::Byte(1))].into();
        let _ = entity.merge_nbt(host, patch);
    }
}

fn tick_komodo_breeding(host: &mut dyn mod_api::ServerHost, id: u32) {
    use mod_api::{Host, Tag};
    let Some(entity) = host.entity(id) else { return; };
    let Some(data) = entity.nbt(host) else { return; };
    let in_love = |tag: Option<&Tag>| matches!(tag, Some(Tag::Int(value)) if *value > 0)
        || matches!(tag, Some(Tag::Short(value)) if *value > 0);
    if !in_love(data.get(&"InLove".into()))
        || data.get(&"Age".into()).is_some_and(|tag| matches!(tag, Tag::Int(value) if *value < 0))
    {
        return;
    }
    let Some(level) = entity.level(host) else { return; };
    let Some(pos) = entity.position(host) else { return; };
    let Some(candidates) = level.entities_in(host,
        [pos[0] - 3.0, pos[1] - 2.0, pos[2] - 3.0],
        [pos[0] + 3.0, pos[1] + 2.0, pos[2] + 3.0]) else { return; };
    let partner = candidates.into_iter().filter(|other| *other != entity)
        .filter_map(|other| {
            let other_data = other.nbt(host)?;
            if other.kind(host).is_none_or(|kind| kind.id(host) != "naturalist:komodo_dragon")
                || !in_love(other_data.get(&"InLove".into()))
                || other_data.get(&"Age".into()).is_some_and(|tag| matches!(tag, Tag::Int(value) if *value < 0))
            { return None; }
            Some((other, other_data))
        }).next();
    let Some((partner, partner_data)) = partner else { return; };
    let Some(first_pos) = entity.position(host) else { return; };
    let Some(second_pos) = partner.position(host) else { return; };
    let baby_pos = [
        (first_pos[0] + second_pos[0]) * 0.5,
        first_pos[1].max(second_pos[1]),
        (first_pos[2] + second_pos[2]) * 0.5,
    ];
    let variant = data.get(&"Variant".into()).or_else(|| partner_data.get(&"Variant".into())).cloned();
    let mut baby = mod_api::Compound::default();
    baby.insert("Age".into(), Tag::Int(-24_000));
    if let Some(variant) = variant { baby.insert("Variant".into(), variant); }
    if host.summon(level.dimension_id(), "naturalist:komodo_dragon", baby_pos, baby).is_none() {
        return;
    }
    let cooldown: mod_api::Compound = [ ("Age".into(), Tag::Int(6_000)), ("InLove".into(), Tag::Int(0)) ].into();
    let _ = entity.merge_nbt(host, cooldown.clone());
    let _ = partner.merge_nbt(host, cooldown);
}

fn tick_tortoise_breeding(host: &mut dyn mod_api::ServerHost, id: u32) {
    use mod_api::{Host, Tag};
    let Some(entity) = host.entity(id) else { return; };
    let Some(data) = entity.nbt(host) else { return; };
    let has_tag = |data: &mod_api::Compound, name: &str| data.get(&name.into()).is_some_and(|tag| matches!(tag, Tag::Byte(value) if *value != 0));
    let in_love = |data: &mod_api::Compound| match data.get(&"InLove".into()) {
        Some(Tag::Int(value)) => *value > 0,
        Some(Tag::Short(value)) => *value > 0,
        _ => false,
    };
    let owned = data.contains_key(&"Owner".into());
    if !owned || !in_love(&data) || has_tag(&data, "HasEgg") { return; }
    if !has_tag(&data, "Tame") {
        let _ = entity.merge_nbt(host, [("Tame".into(), Tag::Byte(1))].into());
    }
    let Some(level) = entity.level(host) else { return; };
    let Some(pos) = entity.position(host) else { return; };
    let Some(nearby) = level.entities_in(host,
        [pos[0] - 3.0, pos[1] - 2.0, pos[2] - 3.0],
        [pos[0] + 3.0, pos[1] + 2.0, pos[2] + 3.0]) else { return; };
    let owner = data.get(&"Owner".into());
    let Some(partner) = nearby.into_iter().filter(|other| *other != entity).find(|other| {
        if other.kind(host).is_none_or(|kind| kind.id(host) != "naturalist:tortoise") { return false; }
        let Some(other_data) = other.nbt(host) else { return false; };
        other_data.contains_key(&"Owner".into()) && in_love(&other_data) && !has_tag(&other_data, "HasEgg")
            && other_data.get(&"Owner".into()) == owner
    }) else { return; };
    let _ = entity.merge_nbt(host, [("HasEgg".into(), Tag::Byte(1)), ("InLove".into(), Tag::Int(0))].into());
    let _ = partner.merge_nbt(host, [("InLove".into(), Tag::Int(0))].into());
}

#[cfg(test)]
mod tests {
    use super::*;
    use mod_api::Attachments;
    #[test]
    fn independent_entities_and_saved_reload() {
        let (mut first, mut second) = (Attachments::default(), Attachments::default());
        let (a, b) = (Holder::Entity(1), Holder::Entity(2));
        assert_eq!(get(&mut first, a).unwrap(), 0);
        set(&mut first, a, 5000).unwrap();
        tick(&mut first, a).unwrap();
        assert_eq!(get(&mut second, b).unwrap(), 0);
        let mut reloaded = Attachments::from_nbt(first.to_nbt());
        assert_eq!(get(&mut reloaded, a).unwrap(), 4999);
        tick(&mut reloaded, a).unwrap();
        assert_eq!(get(&mut reloaded, a).unwrap(), 4998);
    }
    #[test]
    fn legacy_number_types_and_missing_field() {
        let mut data = Compound::default();
        assert_eq!(legacy_cooldown(&data), 0);
        for value in [
            Tag::Byte(8),
            Tag::Short(8),
            Tag::Int(8),
            Tag::Long(8),
            Tag::Float(8.5_f32.to_bits()),
            Tag::Double(8.5_f64.to_bits()),
        ] {
            data.insert("EatCooldown".into(), value);
            assert_eq!(legacy_cooldown(&data), 8);
        }
    }
    #[test]
    fn meal_cooldown_accounts_for_deferred_own_tick_only() {
        assert_eq!(meal_cooldown(true, true), 4999);
        assert_eq!(meal_cooldown(true, false), 5000);
        assert_eq!(meal_cooldown(false, true), 5000);
        assert_eq!(meal_cooldown(false, false), 5000);
    }
    #[test]
    fn ticks_only_positive_values() {
        let mut map = Attachments::default();
        for value in [-1, 0, 1] {
            let holder = Holder::Entity(1);
            set(&mut map, holder, value).unwrap();
            tick(&mut map, holder).unwrap();
            assert_eq!(
                get(&mut map, holder).unwrap(),
                if value > 0 { value - 1 } else { value }
            );
        }
    }
}
