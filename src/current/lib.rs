//! Naturalist current-nightly development port.
mod ant_hill;
pub mod ant_worker;
mod appearance;
mod duck_appearance;
mod duck_renderer;
mod ant_queen;
mod aquatic_spawn;
pub mod anglerfish_behavior;
mod anglerfish_bucket_item;
pub mod bass_appearance;
mod bass_bucket_item;
pub mod bass_fx;
pub mod bass_hunt;
mod bass_renderer;
mod blobfish_behavior;
mod isopod_behavior;
mod jellyfish_behavior;
mod whale_behavior;
mod starfish_behavior;
pub mod shark_behavior;
mod ocean_renderer;
mod ocean_bucket_item;
mod ocean_placements;
mod land_placements;
mod profiles;
mod renderer;
use mod_api::AttachmentQueries;
use std::sync::Arc;
mod net;
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
    ocean: [Arc<ocean_renderer::OceanRenderer>; 8],
}

fn install_goal(
    mob: &mod_api::MobRef,
    host: &mut dyn mod_api::ServerHost,
    target: bool,
    priority: i32,
    goal: mod_api::GoalSpec,
) {
    let existing = if target {
        mob.target_goals(host)
    } else {
        mob.goals(host)
    };
    if existing.is_some_and(|goals| goals.iter().any(|installed| installed.id == goal.id())) {
        return;
    }
    let installed = if target {
        mob.add_target_goal(host, priority, &goal)
    } else {
        mob.add_goal(host, priority, &goal)
    };
    if !installed {
        eprintln!("Naturalist: failed to add {} goal", goal.id());
    }
}

impl Default for Naturalist {
    fn default() -> Self {
        Self {
            deer: Arc::new(renderer::DeerRenderer::new()),
            bass: Arc::new(bass_renderer::BassRenderer::new()),
            duck: Arc::new(duck_renderer::DuckRenderer::new()),
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
    fn mob_spawn_placement_check(
        &self,
        host: &dyn mod_api::Host,
        event: &mut mod_api::MobSpawnPlacementCheckEvent<'_>,
    ) {
        aquatic_spawn::placement_check(host, event);
    }

    fn register(&self, registrar: &mut dyn mod_api::Registrar, _: &mod_api::RegistrationContext) {
        net::register(registrar);
        ant_hill::register(registrar);
        ant_queen::register(registrar);
        land_placements::register(registrar);
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
        for (kind, limit) in [
            ("naturalist:anglerfish", 8),
            ("naturalist:blobfish", 8),
            ("naturalist:giant_isopod", 4),
            ("naturalist:great_white_shark", 4),
            ("naturalist:jellyfish", 8),
            ("naturalist:ray", 8),
            ("naturalist:starfish", 4),
            ("naturalist:whale", 4),
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
    }
    fn entity_join_level(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EntityJoinLevelEvent<'_>,
    ) -> mod_api::Flow {
        if matches!(event.kind, "naturalist:anglerfish" | "naturalist:blobfish" | "naturalist:giant_isopod" | "naturalist:great_white_shark" | "naturalist:jellyfish" | "naturalist:ray" | "naturalist:starfish" | "naturalist:whale") {
            if let Some(entity) = host.entity(event.entity) { ocean_renderer::record_variant(host, entity); }
        }
        if event.kind == "naturalist:giant_isopod" {
            isopod_behavior::joined(host, event.entity);
        }
        if event.kind == "naturalist:blobfish" {
            if let Some(mob) = host.entity(event.entity).and_then(|entity| host.mob(entity)) {
                let has_crab_target = mob
                    .target_goals(host)
                    .is_some_and(|goals| goals.iter().any(|goal| goal.id == "nearest_target"));
                if !has_crab_target {
                    let goal = mod_api::GoalSpec::NearestTarget {
                        prey: mod_api::MobFilter::Species(vec!["naturalist:crab".into()]),
                        must_see: true,
                        must_reach: false,
                        interval: 10,
                    };
                    if !mob.add_target_goal(host, 2, &goal) {
                        eprintln!("Naturalist: failed to add Blobfish Crab target goal");
                    }
                }
                let has_melee = mob
                    .goals(host)
                    .is_some_and(|goals| goals.iter().any(|goal| goal.id == "melee_attack"));
                if !has_melee
                    && !mob.add_goal(
                        host,
                        3,
                        &mod_api::GoalSpec::MeleeAttack {
                            speed: 1.0,
                            follow_unseen: true,
                        },
                    )
                {
                    eprintln!("Naturalist: failed to add Blobfish melee goal");
                }
            }
        }
        if matches!(event.kind, "naturalist:anglerfish" | "naturalist:great_white_shark") {
            if let Some(mob) = host.entity(event.entity).and_then(|entity| host.mob(entity)) {
                let prey = if event.kind == "naturalist:anglerfish" {
                    vec!["minecraft:cod", "minecraft:salmon"]
                } else {
                    vec!["minecraft:cod", "minecraft:salmon", "minecraft:turtle"]
                };
                install_goal(&mob, host, true, 1, mod_api::GoalSpec::NearestTarget {
                    prey: mod_api::MobFilter::Species(prey.into_iter().map(str::to_owned).collect()),
                    must_see: true,
                    must_reach: false,
                    interval: 10,
                });
                install_goal(&mob, host, false, 3, mod_api::GoalSpec::MeleeAttack {
                    speed: 1.0,
                    follow_unseen: true,
                });
            }
        }
        if matches!(event.kind, "naturalist:anglerfish" | "naturalist:blobfish" | "naturalist:giant_isopod" | "naturalist:great_white_shark" | "naturalist:jellyfish" | "naturalist:ray" | "naturalist:whale") {
            if let Some(mob) = host.entity(event.entity).and_then(|entity| host.mob(entity)) {
                install_goal(&mob, host, false, 0, mod_api::GoalSpec::Float);
                // The declared entity template may already have a walking
                // stroll goal. Replace it so this species gets RandomSwimmingGoal.
                let _ = mob.remove_goal(host, "stroll");
                install_goal(&mob, host, false, 5, mod_api::GoalSpec::Stroll {
                    speed: 1.0,
                    interval: 40,
                    water_avoiding: None,
                    swim: true,
                    flying: false,
                });
                install_goal(&mob, host, false, 6, mod_api::GoalSpec::RandomLookAround);
                if event.kind == "naturalist:giant_isopod" {
                    install_goal(&mob, host, false, 2, mod_api::GoalSpec::Breed { speed: 1.0 });
                    install_goal(&mob, host, false, 3, mod_api::GoalSpec::FollowParent { speed: 1.1 });
                }
                if event.kind == "naturalist:whale" {
                    install_goal(&mob, host, false, 1, mod_api::GoalSpec::BreathAir);
                }
            }
        }
        if event.kind == "naturalist:ray" {
            if let Some(mob) = host.entity(event.entity).and_then(|entity| host.mob(entity)) {
                let has_hurt_target = mob
                    .target_goals(host)
                    .is_some_and(|goals| goals.iter().any(|goal| goal.id == "hurt_by_target"));
                if !has_hurt_target
                    && !mob.add_target_goal(
                        host,
                        1,
                        &mod_api::GoalSpec::HurtByTarget { alert: false },
                    )
                {
                    eprintln!("Naturalist: failed to add Ray retaliation goal");
                }
                let has_melee = mob
                    .goals(host)
                    .is_some_and(|goals| goals.iter().any(|goal| goal.id == "melee_attack"));
                if !has_melee
                    && !mob.add_goal(
                        host,
                        3,
                        &mod_api::GoalSpec::MeleeAttack {
                            speed: 1.0,
                            follow_unseen: true,
                        },
                    )
                {
                    eprintln!("Naturalist: failed to add Ray melee goal");
                }
            }
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

    fn entity_tick_pre(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EntityTickPreEvent<'_>,
    ) -> mod_api::Flow {
        if event.kind == "naturalist:great_white_shark" {
            shark_behavior::update_target_gate(host, event.entity);
        }
        if event.kind == "naturalist:whale" {
            whale_behavior::update_breeding_goal(host, event.entity);
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
        if event.kind == "naturalist:blobfish" {
            blobfish_behavior::tick(host, event.entity);
        }
        if event.kind == "naturalist:giant_isopod" {
            isopod_behavior::tick(host, event.entity);
        }
        if event.kind == "naturalist:jellyfish" {
            jellyfish_behavior::tick(host, event.entity);
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
    fn living_damage_pre(
        &self,
        _: &mut dyn mod_api::LevelRandomHost,
        event: &mut mod_api::LivingDamagePreEvent<'_>,
    ) {
        if event.kind == "naturalist:giant_isopod"
            && event.container.reduction(mod_api::Reduction::Armor) > 0.0
        {
            isopod_behavior::cap_shell_damage(event.container);
        }
    }
    fn living_drops(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::LivingDropsEvent<'_>,
    ) -> mod_api::Flow {
        starfish_behavior::drops(host, event);
        mod_api::Flow::Continue
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
