//! Naturalist current-nightly development port.
mod ant_hill;
mod alligator_egg_part;
mod chrysalis_part;
mod snail_eggs_part;
mod scorpion;
mod capybara_interact;
mod piranha_schooling;
mod ray_retaliation;
pub mod anglerfish_behavior;
mod anglerfish_bucket_item;
mod anglerfish_bucket_pickup;
mod blobfish_behavior;
mod isopod_behavior;
mod jellyfish_behavior;
mod ocean_feeding;
mod ocean_placements;
mod ocean_renderer;
pub mod shark_behavior;
mod starfish_behavior;
mod whale_behavior;
mod aquatic_placements;
mod aquatic_spawn;
pub mod ant_worker;
mod bird;
mod vulture;
mod appearance;
mod duck_appearance;
mod duck_renderer;
mod large_renderer;
mod ant_queen;
pub mod bass_appearance;
mod bass_bucket_item;
mod ocean_bucket_item;
mod catfish_bucket_item;
pub mod bass_fx;
pub mod bass_hunt;
mod bass_renderer;
mod land_placements;
mod profiles;
mod renderer;
mod wildlife_renderer;
use mod_api::{AttachmentQueries, BlockMetadata, HostBlockStates, RandomSource};
use std::sync::Arc;
mod net;
mod rat_cargo_drops;
mod elephant_melon;
use mod_api::{
    AttachmentCodec, AttachmentError, AttachmentStore, AttachmentType, Compound, Holder, Tag,
};

static COOLDOWN: AttachmentType<i32> = AttachmentType {
    namespace: "naturalist",
    name: "bass_eat_cooldown",
    default: || 0,
    codec: Some(AttachmentCodec { encode, decode }),
    copy_on_death: false,
    sync: false,
};
static FIREFLY_AGE: AttachmentType<i32> = AttachmentType {
    namespace: "naturalist",
    name: "firefly_age",
    default: || 0,
    codec: None,
    copy_on_death: false,
    sync: false,
};
static POISON_TARGETS: AttachmentType<Vec<[i32; 2]>> = AttachmentType {
    namespace: "naturalist",
    name: "pending_scorpion_poison_targets",
    default: Vec::new,
    codec: Some(AttachmentCodec {
        encode: encode_poison_targets,
        decode: decode_poison_targets,
    }),
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
fn encode_poison_targets(rows: &Vec<[i32; 2]>) -> Result<Compound, AttachmentError> {
    let flat: Vec<i32> = rows.iter().flat_map(|row| row.iter().copied()).collect();
    Ok(Compound::from([("rows".into(), Tag::IntArray(flat))]))
}
fn decode_poison_targets(data: &Compound) -> Result<Vec<[i32; 2]>, AttachmentError> {
    let Some(Tag::IntArray(flat)) = data.get(&"rows".into()) else {
        return Err(AttachmentError::Decode);
    };
    if flat.len() % 2 != 0 {
        return Err(AttachmentError::Decode);
    }
    Ok(flat.chunks_exact(2).map(|row| [row[0], row[1]]).collect())
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
    elephant: Arc<large_renderer::LargeRenderer>,
    mammoth: Arc<large_renderer::LargeRenderer>,
    giraffe: Arc<large_renderer::LargeRenderer>,
    lion: Arc<large_renderer::LargeRenderer>,
    rhino: Arc<large_renderer::LargeRenderer>,
    tiger: Arc<large_renderer::LargeRenderer>,
    zebra: Arc<large_renderer::LargeRenderer>,
    alligator: Arc<wildlife_renderer::WildlifeRenderer>,
    capybara: Arc<wildlife_renderer::WildlifeRenderer>,
    catfish: Arc<wildlife_renderer::WildlifeRenderer>,
    clam: Arc<wildlife_renderer::WildlifeRenderer>,
    hippo: Arc<wildlife_renderer::WildlifeRenderer>,
    piranha: Arc<wildlife_renderer::WildlifeRenderer>,
    ocean: [Arc<ocean_renderer::OceanRenderer>; 8],
}

fn add_ray_goals(mob: &mod_api::MobRef, host: &mut dyn mod_api::ServerHost) {
    let has_hurt_target = mob
        .target_goals(host)
        .is_some_and(|goals| goals.iter().any(|goal| goal.id == "hurt_by_target"));
    if !has_hurt_target {
        let _ = mob.add_target_goal(
            host,
            1,
            &mod_api::GoalSpec::HurtByTarget { alert: false },
        );
    }
    // Ray's inherited melee goal does not reliably pursue its hurt-by target.
    let _ = mob.remove_goal(host, "melee_attack");
    let _ = mob.add_goal(
        host,
        3,
        &mod_api::GoalSpec::MeleeAttack {
            speed: 1.0,
            follow_unseen: true,
        },
    );
}
impl Default for Naturalist {
    fn default() -> Self {
        Self {
            deer: Arc::new(renderer::DeerRenderer::new()),
            bass: Arc::new(bass_renderer::BassRenderer::new()),
            duck: Arc::new(duck_renderer::DuckRenderer::new()),
            elephant: Arc::new(large_renderer::LargeRenderer::elephant()),
            mammoth: Arc::new(large_renderer::LargeRenderer::mammoth()),
            giraffe: Arc::new(large_renderer::LargeRenderer::giraffe()),
            lion: Arc::new(large_renderer::LargeRenderer::lion()),
            rhino: Arc::new(large_renderer::LargeRenderer::rhino()),
            tiger: Arc::new(large_renderer::LargeRenderer::tiger()),
            zebra: Arc::new(large_renderer::LargeRenderer::zebra()),
            alligator: Arc::new(wildlife_renderer::WildlifeRenderer::new(wildlife_renderer::Species::Alligator)),
            capybara: Arc::new(wildlife_renderer::WildlifeRenderer::new(wildlife_renderer::Species::Capybara)),
            catfish: Arc::new(wildlife_renderer::WildlifeRenderer::new(wildlife_renderer::Species::Catfish)),
            clam: Arc::new(wildlife_renderer::WildlifeRenderer::new(wildlife_renderer::Species::Clam)),
            hippo: Arc::new(wildlife_renderer::WildlifeRenderer::new(wildlife_renderer::Species::Hippo)),
            piranha: Arc::new(wildlife_renderer::WildlifeRenderer::new(wildlife_renderer::Species::Piranha)),
            ocean: [
                Arc::new(ocean_renderer::OceanRenderer::anglerfish()),
                Arc::new(ocean_renderer::OceanRenderer::blobfish()),
                Arc::new(ocean_renderer::OceanRenderer::isopod()),
                Arc::new(ocean_renderer::OceanRenderer::shark()),
                Arc::new(ocean_renderer::OceanRenderer::jellyfish()),
                Arc::new(ocean_renderer::OceanRenderer::ray()),
                Arc::new(ocean_renderer::OceanRenderer::starfish()),
                Arc::new(ocean_renderer::OceanRenderer::whale()),
            ],
        }
    }
}

#[mod_api::horizon_mod]
impl mod_api::Hooks for Naturalist {
    fn register(&self, registrar: &mut dyn mod_api::Registrar, _: &mod_api::RegistrationContext) {
        net::register(registrar);
        bird::register(registrar);
        alligator_egg_part::register(registrar);
        elephant_melon::register(registrar);
        ant_hill::register(registrar);
        chrysalis_part::register(registrar);
        snail_eggs_part::register(registrar);
        ant_queen::register(registrar);
        land_placements::register(registrar);
        aquatic_placements::register(registrar);
        ocean_placements::register(registrar);
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
        anglerfish_bucket_item::register(registrar);
        ocean_bucket_item::register(registrar);
        catfish_bucket_item::register(registrar);
        for (kind, limit) in [
            ("naturalist:anglerfish", 8), ("naturalist:blobfish", 8),
            ("naturalist:giant_isopod", 4), ("naturalist:great_white_shark", 4),
            ("naturalist:jellyfish", 8), ("naturalist:ray", 8),
            ("naturalist:starfish", 4), ("naturalist:whale", 4),
        ] {
            registrar.entity_part(kind, aquatic_spawn::PackLimit(limit));
        }
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
        registrar.entity_part("naturalist:desert_scorpion", scorpion::ScorpionPart);
        registrar.entity_part("naturalist:jungle_scorpion", scorpion::ScorpionPart);
    }
    fn entity_join_level(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EntityJoinLevelEvent<'_>,
    ) -> mod_api::Flow {
        if let Some(mob) = host
            .entity(event.entity)
            .and_then(|entity| host.mob(entity))
        {
            let add_goal = |mob: &mod_api::MobRef, host: &mut dyn mod_api::ServerHost, priority, goal: mod_api::GoalSpec| {
                let id = goal.id();
                if !mob.goals(host).is_some_and(|goals| goals.iter().any(|entry| entry.id == id)) {
                    let _ = mob.add_goal(host, priority, &goal);
                }
            };
            let add_target = |mob: &mod_api::MobRef, host: &mut dyn mod_api::ServerHost, priority, goal: mod_api::GoalSpec| {
                let id = goal.id();
                if !mob.target_goals(host).is_some_and(|goals| goals.iter().any(|entry| entry.id == id)) {
                    let _ = mob.add_target_goal(host, priority, &goal);
                }
            };
            match event.kind {
                "naturalist:capybara" => {
                    add_goal(&mob, host, 2, mod_api::GoalSpec::Tempt { speed: 1.25, items: vec!["minecraft:melon".into()], can_scare: true });
                    add_goal(&mob, host, 4, mod_api::GoalSpec::Breed { speed: 1.0 });
                    add_goal(&mob, host, 5, mod_api::GoalSpec::FollowParent { speed: 1.1 });
                }
                "naturalist:piranha" => {
                    add_goal(&mob, host, 3, mod_api::GoalSpec::MeleeAttack { speed: 1.2, follow_unseen: true });
                    add_target(&mob, host, 1, mod_api::GoalSpec::NearestTarget { prey: mod_api::MobFilter::Species(vec!["minecraft:cod".into(), "minecraft:salmon".into(), "minecraft:tropical_fish".into(), "minecraft:pufferfish".into()]), must_see: false, must_reach: false, interval: 10 });
                    add_target(&mob, host, 2, mod_api::GoalSpec::HurtByTarget { alert: true });
                    piranha_schooling::reset(host, event.entity);
                }
                _ => {}
            }
        }
        if event.kind == "naturalist:ray" {
            if let Some(mob) = host
                .entity(event.entity)
                .and_then(|entity| host.mob(entity))
            {
                add_ray_goals(&mob, host);
            }
        }
        if matches!(event.kind, "naturalist:anglerfish" | "naturalist:blobfish" | "naturalist:giant_isopod" | "naturalist:great_white_shark" | "naturalist:jellyfish" | "naturalist:ray" | "naturalist:starfish" | "naturalist:whale") {
            if let Some(entity) = host.entity(event.entity) { ocean_renderer::record_variant(host, entity); }
        }
        if event.kind == "naturalist:giant_isopod" {
            isopod_behavior::joined(host, event.entity);
        }
        if event.kind == "naturalist:blobfish" {
            if let Some(mob) = host.entity(event.entity).and_then(|entity| host.mob(entity)) {
                let target = mod_api::GoalSpec::NearestTarget {
                    prey: mod_api::MobFilter::Species(vec!["naturalist:crab".into()]),
                    must_see: true, must_reach: false, interval: 10,
                };
                if !mob.target_goals(host).is_some_and(|goals| goals.iter().any(|g| g.id == "nearest_target")) {
                    let _ = mob.add_target_goal(host, 2, &target);
                }
                if !mob.goals(host).is_some_and(|goals| goals.iter().any(|g| g.id == "melee_attack")) {
                    let _ = mob.add_goal(host, 3, &mod_api::GoalSpec::MeleeAttack { speed: 1.0, follow_unseen: true });
                }
            }
        }
        if matches!(event.kind, "naturalist:anglerfish" | "naturalist:great_white_shark") {
            if let Some(mob) = host.entity(event.entity).and_then(|entity| host.mob(entity)) {
                let prey = if event.kind == "naturalist:anglerfish" {
                    vec!["minecraft:cod", "minecraft:salmon"]
                } else { vec!["minecraft:cod", "minecraft:salmon", "minecraft:turtle"] };
                if !mob.target_goals(host).is_some_and(|goals| goals.iter().any(|g| g.id == "nearest_target")) {
                    let _ = mob.add_target_goal(host, 1, &mod_api::GoalSpec::NearestTarget {
                        prey: mod_api::MobFilter::Species(prey.into_iter().map(str::to_owned).collect()),
                        must_see: true, must_reach: false, interval: 10,
                    });
                }
                if !mob.goals(host).is_some_and(|goals| goals.iter().any(|g| g.id == "melee_attack")) {
                    let _ = mob.add_goal(host, 3, &mod_api::GoalSpec::MeleeAttack { speed: 1.0, follow_unseen: true });
                }
            }
        }
        if matches!(event.kind, "naturalist:anglerfish" | "naturalist:blobfish" | "naturalist:giant_isopod" | "naturalist:great_white_shark" | "naturalist:jellyfish" | "naturalist:ray" | "naturalist:whale") {
            if let Some(mob) = host.entity(event.entity).and_then(|entity| host.mob(entity)) {
                let _ = mob.add_goal(host, 0, &mod_api::GoalSpec::Float);
                let _ = mob.remove_goal(host, "stroll");
                let _ = mob.add_goal(host, 5, &mod_api::GoalSpec::Stroll { speed: 1.0, interval: 40, water_avoiding: None, swim: true, flying: false });
                let _ = mob.add_goal(host, 6, &mod_api::GoalSpec::RandomLookAround);
                if event.kind == "naturalist:giant_isopod" {
                    let _ = mob.add_goal(host, 2, &mod_api::GoalSpec::Breed { speed: 1.0 });
                    let _ = mob.add_goal(host, 3, &mod_api::GoalSpec::FollowParent { speed: 1.1 });
                }
                if event.kind == "naturalist:whale" {
                    let _ = mob.add_goal(host, 1, &mod_api::GoalSpec::BreathAir);
                }
            }
        }
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
            let from_hand = host.entity(event.entity).and_then(|entity| entity.nbt(host)).is_some_and(|data| {
                matches!(data.get(&"FromHand".into()), Some(mod_api::Tag::Byte(value)) if *value != 0)
            });
            scorpion::set_from_hand(host, event.entity, from_hand);
            if let Some(mob) = host.entity(event.entity).and_then(|entity| host.mob(entity)) {
                let retaliation = mod_api::GoalSpec::HurtByTarget { alert: false };
                let _ = mob.add_target_goal(host, 0, &retaliation);
                let attack = mod_api::GoalSpec::MeleeAttack { speed: 1.0, follow_unseen: true };
                if !mob.add_goal(host, 2, &attack) {
                    eprintln!("Naturalist: failed to add scorpion melee goal");
                }
            }
        } else if event.kind == "naturalist:bass" {
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

    fn entity_interact(
        &self,
        side: mod_api::HostSide<'_>,
        event: &mut mod_api::EntityInteractEvent<'_>,
    ) -> Option<mod_api::InteractionResult> {
        if matches!(event.kind,
            "naturalist:anglerfish" | "naturalist:blobfish" | "naturalist:giant_isopod"
                | "naturalist:jellyfish" | "naturalist:ray" | "naturalist:starfish")
        {
            anglerfish_bucket_pickup::interact(side, event)
        } else if event.kind == "naturalist:great_white_shark" || event.kind == "naturalist:whale" {
            ocean_feeding::interact(side, event)
        } else {
            capybara_interact::interact(side, event)
        }
    }

    fn living_drops(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::LivingDropsEvent<'_>,
    ) -> mod_api::Flow {
        starfish_behavior::drops(host, event);
        rat_cargo_drops::drops(host, event);
        mod_api::Flow::Continue
    }

    fn mob_spawn_placement_check(
        &self,
        host: &dyn mod_api::Host,
        event: &mut mod_api::MobSpawnPlacementCheckEvent<'_>,
    ) {
        aquatic_spawn::placement_check(host, event);
        if event.reason != mod_api::SpawnReason::Natural
            || !matches!(event.kind, "naturalist:anglerfish" | "naturalist:blobfish" | "naturalist:piranha")
        {
            return;
        }
        let pos = [event.pos.x, event.pos.y, event.pos.z];
        let sea_level = event.level.sea_level(host);
        let fluid_water = host.fluid_key("minecraft:water");
        let water_fluid_at = |at| event.level.fluid_state(host, at)
            .is_some_and(|state| !state.is_empty() && Some(state.fluid) == fluid_water);
        let water_block_at = |at| event.level.block(host, at)
            .and_then(|block| host.block_states().name(block).ok().map(str::to_owned))
            .is_some_and(|name| name == "minecraft:water");
        let decision = match event.kind {
            "naturalist:anglerfish" => {
                let surface = aquatic_placements::anglerfish_surface_spawn_ok(
                    event.default_result, pos[1], sea_level,
                    water_fluid_at([pos[0], pos[1] - 1, pos[2]]),
                    water_block_at([pos[0], pos[1] + 1, pos[2]]),
                );
                aquatic_placements::anglerfish_spawn_ok(
                    surface, pos[1], sea_level, water_block_at(pos),
                    event.level.brightness(host, mod_api::LightLayer::Block, pos),
                    event.level.brightness(host, mod_api::LightLayer::Sky, pos),
                )
            }
            "naturalist:blobfish" => aquatic_placements::deep_water_spawn_ok(event.default_result, pos[1], sea_level, 33),
            "naturalist:piranha" => aquatic_placements::deep_water_spawn_ok(event.default_result, pos[1], sea_level, 0),
            _ => unreachable!(),
        };
        *event.result = if decision { mod_api::SpawnCheck::Succeed } else { mod_api::SpawnCheck::Fail };
    }

    fn living_damage_post(
        &self,
        host: &mut dyn mod_api::LevelRandomHost,
        event: &mut mod_api::LivingDamagePostEvent<'_>,
    ) {
        if event.container.health_damage() <= 0.0 || event.source.causing <= 0 {
            return;
        }
        let mut targets = POISON_TARGETS.get(host, Holder::Level).unwrap_or_default();
        targets.push([event.entity as i32, event.source.causing]);
        let _ = POISON_TARGETS.set(host, Holder::Level, targets);
    }

    fn entity_place(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EntityPlaceEvent,
    ) -> mod_api::Flow {
        let is_snail_egg = host
            .block_state_string(event.state)
            .is_some_and(|name| name.starts_with("naturalist:snail_eggs"));
        if !is_snail_egg {
            return mod_api::Flow::Continue;
        }
        let Some(level) = event.placer.and_then(|placer| placer.level(host)) else {
            return mod_api::Flow::Continue;
        };
        let support = [event.pos.x, event.pos.y - 1, event.pos.z];
        if level.block_face_sturdy(
            host,
            support,
            mod_api::BlockFace::Up,
            mod_api::SupportType::Full,
        ) != Some(true)
        {
            return mod_api::Flow::Cancel;
        }
        let Some(now) = level.game_time(host) else {
            return mod_api::Flow::Continue;
        };
        let Some(delay) = level.with_random(host, |random| {
            random.next_int_bound(1800).unwrap_or(0) + 600
        }) else {
            return mod_api::Flow::Continue;
        };
        snail_eggs_part::schedule_hatch(
            host,
            [event.pos.x, event.pos.y, event.pos.z],
            now + i64::from(delay),
        );
        mod_api::Flow::Continue
    }


    fn server_tick(&self, host: &mut dyn mod_api::ServerHost, event: &mut mod_api::ServerTickEvent<'_>) {
        if !matches!(event.phase, mod_api::ServerTickPhase::ServerTickEnd { .. }) {
            return;
        }
        alligator_egg_part::hatch_tick(host);
        let level = host.level("minecraft:overworld");
        if let Some(level) = level {
            let now = level.game_time(host).unwrap_or(i64::MIN);
            let mut future = Vec::new();
            for row in snail_eggs_part::hatch_queue(host) {
                if row[3] > now {
                    future.push(row);
                    continue;
                }
                let pos = [row[0] as i32, row[1] as i32, row[2] as i32];
                let still_eggs = level
                    .block_state(host, pos)
                    .and_then(|state| host.block_state_string(state))
                    .is_some_and(|name| name.starts_with("naturalist:snail_eggs"));
                if !still_eggs {
                    continue;
                }
                let count = level
                    .with_random(host, |random| random.next_int_bound(4).unwrap_or(0) + 2)
                    .unwrap_or(2)
                    .clamp(2, 5);
                if !level.remove_block(host, pos, false) {
                    continue;
                }
                for _ in 0..count {
                    if let Some(child) = level.spawn_mob(
                        host,
                        "naturalist:snail",
                        [pos[0] as f64 + 0.5, pos[1] as f64 + 0.5, pos[2] as f64 + 0.5],
                        mod_api::SpawnReason::Breeding,
                    ) {
                        if let Some(mob) = host.mob(child) {
                            let _ = mob.set_baby(host, true);
                        }
                        let _ = child.merge_nbt(host, Compound::from([("Age".into(), Tag::Int(-6000))]));
                    }
                }
            }
            snail_eggs_part::save_hatch_queue(host, future);
        }
        if let (Some(level), Ok(mut pending)) = (level, chrysalis_part::PENDING.get(&mut *host, mod_api::Holder::Level))
            && let Some(pos) = pending.first().copied()
            && level.spawn_mob(host, "naturalist:butterfly", [pos[0] as f64 + 0.5, pos[1] as f64 + 0.5, pos[2] as f64 + 0.5], mod_api::SpawnReason::Command).is_some()
        {
            pending.remove(0);
            let _ = chrysalis_part::PENDING.set(host, mod_api::Holder::Level, pending);
        }
        let poison_targets = POISON_TARGETS.get(host, Holder::Level).unwrap_or_default();
        let _ = POISON_TARGETS.set(host, Holder::Level, Vec::new());
        if let Some(poison) = host.effect_key("minecraft:poison") {
            for [target_id, attacker_id] in poison_targets {
                let is_scorpion = u32::try_from(attacker_id)
                    .ok()
                    .and_then(|id| host.entity(id))
                    .and_then(|entity| entity.kind(host))
                    .is_some_and(|kind| matches!(kind.id(host).as_str(), "naturalist:desert_scorpion" | "naturalist:jungle_scorpion"));
                if is_scorpion
                    && let Ok(target_id) = u32::try_from(target_id)
                    && let Some(living) = host.entity(target_id).and_then(|entity| host.living(entity))
                {
                    let effect = mod_api::ActiveEffect::new(poison, 120, 0, false, true, true);
                    let _ = living.add_effect(host, &effect);
                }
            }
        }
    }

    fn entity_tick_pre(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EntityTickPreEvent<'_>,
    ) -> mod_api::Flow {
        if event.kind == "naturalist:bird" {
            if let Some(bird) = host.entity(event.entity) {
                if let Err(error) = bird::eat_nearby_seeds(host, bird) {
                    eprintln!("Naturalist: bird seed feeding failed: {error:?}");
                }
            }
        }
        if event.kind == "naturalist:vulture" {
            if let Some(vulture) = host.entity(event.entity) {
                vulture::tick(host, vulture);
            }
        }
        if event.kind == "naturalist:great_white_shark" {
            shark_behavior::update_target_gate(host, event.entity);
        }
        if event.kind == "naturalist:whale" {
            whale_behavior::update_breeding_goal(host, event.entity);
        }
        if event.kind == "naturalist:piranha" {
            piranha_schooling::tick(host, event.entity);
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
        if event.kind == "naturalist:blobfish" { blobfish_behavior::tick(host, event.entity); }
        if event.kind == "naturalist:giant_isopod" { isopod_behavior::tick(host, event.entity); }
        if event.kind == "naturalist:jellyfish" { jellyfish_behavior::tick(host, event.entity); }
        if event.kind == "naturalist:great_white_shark" {
            shark_behavior::tick_air(host, event.entity);
            shark_behavior::tick_hunting_cooldown(host, event.entity);
        }
        if event.kind == "naturalist:ray" {
            ray_retaliation::tick(host, event.entity);
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
        for (kind, renderer) in [
            "naturalist:anglerfish", "naturalist:blobfish", "naturalist:giant_isopod",
            "naturalist:great_white_shark", "naturalist:jellyfish", "naturalist:ray",
            "naturalist:starfish", "naturalist:whale",
        ].into_iter().zip(self.ocean.iter()) {
            event.renderers.register(kind.into(), renderer.clone());
        }
        for (id, renderer) in [
            ("naturalist:elephant", self.elephant.clone()),
            ("naturalist:mammoth", self.mammoth.clone()),
            ("naturalist:giraffe", self.giraffe.clone()),
            ("naturalist:lion", self.lion.clone()),
            ("naturalist:rhino", self.rhino.clone()),
            ("naturalist:tiger", self.tiger.clone()),
            ("naturalist:zebra", self.zebra.clone()),
        ] {
            event.renderers.register(id.into(), renderer);
        }
        for (id, renderer) in [
            ("naturalist:alligator", self.alligator.clone()),
            ("naturalist:capybara", self.capybara.clone()),
            ("naturalist:catfish", self.catfish.clone()),
            ("naturalist:clam", self.clam.clone()),
            ("naturalist:hippo", self.hippo.clone()),
            ("naturalist:piranha", self.piranha.clone()),
        ] {
            event.renderers.register(id.into(), renderer);
        }
    }

    fn render_living_pre(
        &self,
        host: &dyn mod_api::ClientHost,
        event: &mut mod_api::RenderLivingPreEvent<'_>,
    ) -> mod_api::Flow {
        if let Some(index) = match event.entity.kind {
            "naturalist:anglerfish" => Some(0), "naturalist:blobfish" => Some(1),
            "naturalist:giant_isopod" => Some(2), "naturalist:great_white_shark" => Some(3),
            "naturalist:jellyfish" => Some(4), "naturalist:ray" => Some(5),
            "naturalist:starfish" => Some(6), "naturalist:whale" => Some(7), _ => None,
        } {
            if let Some(entity) = host.entity(event.entity.id) {
                self.ocean[index].observe_variant(entity.id(), ocean_renderer::synced_variant(host, entity));
            }
        }
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
        for renderer in &self.ocean { renderer.retain_live(host); }
    }

    fn client_level_unload(
        &self,
        _: &mut dyn mod_api::ClientHost,
        _: &mut mod_api::ClientLevelUnloadEvent<'_>,
    ) {
        self.deer.clear();
        self.bass.clear();
        self.duck.clear();
        for renderer in &self.ocean { renderer.clear(); }
    }

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
