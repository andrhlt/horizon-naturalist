//! Naturalist current-nightly development port.
mod ant_hill;
mod appearance;
pub mod bass_fx;
pub mod bass_hunt;
mod bass_bucket_item;
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
}
impl Default for Naturalist {
    fn default() -> Self {
        Self {
            deer: Arc::new(renderer::DeerRenderer::new()),
        }
    }
}

#[mod_api::horizon_mod]
impl mod_api::Hooks for Naturalist {
    fn register(&self, registrar: &mut dyn mod_api::Registrar, _: &mod_api::RegistrationContext) {
        net::register(registrar);
        ant_hill::register(registrar);
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
    }
    fn entity_join_level(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EntityJoinLevelEvent<'_>,
    ) -> mod_api::Flow {
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
    fn register_entity_renderers(
        &self,
        _: &mod_api::RegistrationContext,
        event: &mut mod_api::RegisterEntityRenderersEvent<'_>,
    ) {
        event
            .renderers
            .register("naturalist:deer".into(), self.deer.clone());
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
        mod_api::Flow::Continue
    }

    fn client_tick_post(
        &self,
        host: &mut dyn mod_api::ClientHost,
        _: &mut mod_api::ClientTickPostEvent,
    ) {
        self.deer.retain_live(host);
    }

    fn client_level_unload(
        &self,
        _: &mut dyn mod_api::ClientHost,
        _: &mut mod_api::ClientLevelUnloadEvent<'_>,
    ) {
        self.deer.clear();
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
