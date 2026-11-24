//! Species-specific native goals for pets, moles and digging wolves.
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use mod_api::{AttachmentQueries, Host, RandomSource};

#[derive(Default, Clone, Copy)]
pub(crate) struct PetGoalState {
    pub owner: Option<u128>,
    pub tamed: bool,
    pub sitting: bool,
    pub following: bool,
}

static PET_GOAL_STATE: mod_api::AttachmentType<PetGoalState> = mod_api::AttachmentType {
    namespace: "naturalist",
    name: "pet_goal_state",
    default: PetGoalState::default,
    codec: None,
    copy_on_death: false,
    sync: false,
};
/// Refresh the immutable custom-goal view from authoritative entity NBT.
/// Join/load and interaction hooks call this whenever pet state can change.
pub(crate) fn sync_pet_goal_state(host: &mut dyn mod_api::ServerHost, id: u32) {
    let Some(entity) = host.entity(id) else {
        return;
    };
    let Some(data) = entity.nbt(host) else {
        return;
    };
    let kind = entity.kind(host).map(|kind| kind.id(host));
    let is_lizard = kind.as_deref() == Some("naturalist:lizard");
    let is_tortoise = kind.as_deref() == Some("naturalist:tortoise");
    let nbt_sitting = data
        .get(&if is_lizard { "NaturalistOrderedSitting" } else { "Sitting" }.into())
        .is_some_and(|tag| matches!(tag, mod_api::Tag::Byte(value) if *value != 0));
    let state = PetGoalState {
        owner: data.get(&"Owner".into()).and_then(tag_uuid),
        tamed: matches!(data.get(&"Tame".into()), Some(mod_api::Tag::Byte(value)) if *value != 0),
        sitting: nbt_sitting,
        following: match data.get(&"FollowingOwner".into()) {
            Some(mod_api::Tag::Byte(value)) => *value != 0,
            _ => is_tortoise,
        },
    };
    let _ = PET_GOAL_STATE.set(host, mod_api::Holder::Entity(id), state);
    if is_lizard && state.tamed {
        if let Some(mob) = host.mob(entity) {
            let _ = mob.set_ordered_sitting(host, nbt_sitting);
        }
    }
}

/// Apply an owner interaction's ordered sitting state to both the goal snapshot
/// and persisted entity data. Native ticking can rewrite the legacy NBT flag.
pub(crate) fn set_pet_sitting(host: &mut dyn mod_api::ServerHost, id: u32, sitting: bool) {
    use mod_api::{Attachments, Host, Tag};

    let holder = mod_api::Holder::Entity(id);
    let Some(entity) = host.entity(id) else {
        return;
    };
    let is_lizard = entity.kind(host).is_some_and(|kind| kind.id(host) == "naturalist:lizard");
    if let Some(mob) = host.mob(entity) {
        let _ = mob.set_ordered_sitting(host, sitting);
    }
    let mut state = PET_GOAL_STATE.get(host, holder).unwrap_or_default();
    state.sitting = sitting;
    let _ = PET_GOAL_STATE.set(host, holder, state);
    let mut patch = mod_api::Compound::default();
    patch.insert("Sitting".into(), Tag::Byte(i8::from(sitting)));
    if is_lizard {
        patch.insert(
            "NaturalistOrderedSitting".into(),
            Tag::Byte(i8::from(sitting)),
        );
    }
    let _ = entity.merge_nbt(host, patch);
}

/// The lizard's ordered-sitting state is attachment-backed during a live session;
/// its NBT bit is the durable value used when the entity is loaded again.
pub(crate) fn pet_sitting(host: &mut dyn mod_api::ServerHost, id: u32) -> Option<bool> {
    PET_GOAL_STATE
        .get(host, mod_api::Holder::Entity(id))
        .ok()
        .map(|state| state.sitting)
}

/// Keep the persisted interaction flag aligned with the state used by pet goals.
/// Native mob ticking can rewrite the legacy NBT flag from its internal sit state;
/// this makes the successful tame/owner interaction durable until the owner toggles it.
pub(crate) fn persist_pet_sitting(host: &mut dyn mod_api::ServerHost, id: u32) {
    use mod_api::{Host, Tag};

    let Some(entity) = host.entity(id) else {
        return;
    };
    let Ok(state) = PET_GOAL_STATE.get(host, mod_api::Holder::Entity(id)) else {
        return;
    };
    if !state.tamed {
        return;
    }
    let sitting = state.sitting;
    if let Some(mob) = host.mob(entity) {
        let _ = mob.set_ordered_sitting(host, sitting);
    }
    let _ = entity.merge_nbt(
        host,
        [
            ("Sitting".into(), Tag::Byte(i8::from(sitting))),
            ("NaturalistOrderedSitting".into(), Tag::Byte(i8::from(sitting))),
        ].into(),
    );
}

fn pet_goal_state(
    cx: &mod_api::GoalContext<'_>,
    entity: mod_api::EntityRef,
) -> Option<PetGoalState> {
    cx.host()
        .query(&PET_GOAL_STATE)
        .ok()?
        .get(mod_api::AttachmentHolder::Entity(entity))
        .ok()?
}

#[derive(Default, Clone)]
pub(crate) struct MoleRuntime {
    pub state: i32,
    pub state_ticks: i32,
    pub calm_ticks: i32,
    pub peek_ticks: i32,
    pub grace_ticks: i32,
    pub lockout_ticks: i32,
    pub peek_saw_threat: bool,
    pub threat_detected: bool,
    pub attacker: Option<u32>,
    pub attacker_ticks: i32,
    pub last_mound: Option<[i32; 3]>,
    pub dug_out_hold_ticks: i32,
    pub pop_out_pending: bool,
}

impl MoleRuntime {
    pub(crate) fn at_state(state: i32) -> Self {
        let mut runtime = Self {
            state,
            ..Self::default()
        };
        match state {
            1 => runtime.state_ticks = 20,
            2 => {
                runtime.grace_ticks = 60;
                runtime.peek_ticks = 60;
            }
            3 => runtime.state_ticks = 57,
            4 => runtime.state_ticks = 25,
            _ => {}
        }
        runtime
    }
}

pub(crate) static MOLE_STATES: LazyLock<Mutex<HashMap<u32, MoleRuntime>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub struct MolePart;
#[mod_api::horizon_part]
impl mod_api::EntityPart for MolePart {
    fn register_goals(&self, goals: &mut mod_api::GoalRegistrar<'_>) {
        goals.add_goal(2, MoleFlee::default());
        goals.add_goal(3, MoleScurry::default());
    }
}

/// Add a native fallback flee goal during the join hook as well as the custom
/// part goal. This keeps Mole escape working if the part's goal callback is not
/// included by a host's mob construction path.
pub(crate) fn register_mole_escape(host: &mut dyn mod_api::ServerHost, entity: mod_api::EntityRef) {
    let Some(mob) = host.mob(entity) else {
        return;
    };
    let goal = mod_api::GoalSpec::AvoidEntity {
        what: mod_api::MobFilter::Species(
            [
                "minecraft:zombie",
                "minecraft:husk",
                "minecraft:drowned",
                "minecraft:zombie_villager",
                "minecraft:skeleton",
                "minecraft:stray",
                "minecraft:bogged",
                "minecraft:wither_skeleton",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        ),
        max_dist: 6.0,
        walk: 1.0,
        sprint: 1.25,
    };
    let _ = mob.add_goal(host, 2, &goal);
}

pub struct WolfPart;
#[mod_api::horizon_part]
impl mod_api::EntityPart for WolfPart {
    fn register_goals(&self, goals: &mut mod_api::GoalRegistrar<'_>) {
        goals.add_goal(3, WolfDig::default());
        goals.add_target_goal(4, WolfMoleTarget::default());
    }
}

pub struct PetPart;
#[mod_api::horizon_part]
impl mod_api::EntityPart for PetPart {
    fn register_goals(&self, goals: &mut mod_api::GoalRegistrar<'_>) {
        match goals.mob().id() {
            "naturalist:lizard" => goals.add_goal(2, PetFollow::default()),
            "naturalist:tortoise" => goals.add_goal(3, PetFollow::default()),
            _ => {}
        }
    }
}

#[derive(Default)]
struct PetFollow {
    owner: Option<mod_api::LivingRef>,
    recalc: u8,
}
impl mod_api::Goal for PetFollow {
    fn flags(&self) -> mod_api::GoalFlags {
        mod_api::GoalFlags::MOVE.union(mod_api::GoalFlags::LOOK)
    }
    fn can_use(&mut self, cx: &mut mod_api::GoalContext<'_>) -> bool {
        let own = cx.mob().entity();
        let Some(state) = pet_goal_state(cx, own) else {
            return false;
        };
        if !state.following || state.sitting || !state.tamed {
            return false;
        }
        let Some(uuid) = state.owner else {
            return false;
        };
        let Some(owner) = cx
            .host()
            .player(uuid)
            .and_then(|p| cx.host().living(p.entity()))
        else {
            return false;
        };
        let (Some(a), Some(b)) = (own.position(cx.host()), owner.entity().position(cx.host()))
        else {
            return false;
        };
        if (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f64>() < 100.0 {
            return false;
        }
        self.owner = Some(owner);
        true
    }
    fn can_continue(&mut self, cx: &mut mod_api::GoalContext<'_>) -> bool {
        let own = cx.mob().entity();
        let Some(owner) = self.owner else {
            return false;
        };
        let Some(state) = pet_goal_state(cx, own) else {
            return false;
        };
        if state.sitting || !state.following {
            return false;
        }
        own.position(cx.host())
            .zip(owner.entity().position(cx.host()))
            .is_some_and(|(a, b)| (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f64>() > 25.0)
    }
    fn start(&mut self, cx: &mut mod_api::GoalContext<'_>) {
        self.recalc = 0;
    }
    fn tick(&mut self, cx: &mut mod_api::GoalContext<'_>) {
        let Some(owner) = self.owner else {
            return;
        };
        let Some(at) = owner.entity().position(cx.host()) else {
            return;
        };
        cx.look_at(at);
        self.recalc = self.recalc.saturating_sub(1);
        if self.recalc == 0 {
            self.recalc = 10;
            let _ = cx.move_to(at, 1.0);
        }
    }
    fn stop(&mut self, cx: &mut mod_api::GoalContext<'_>) {
        self.owner = None;
        cx.stop_navigation();
    }
}
fn tag_uuid(tag: &mod_api::Tag) -> Option<u128> {
    let mod_api::Tag::IntArray(parts) = tag else {
        return None;
    };
    if parts.len() != 4 {
        return None;
    }
    Some(
        (u128::from(parts[0] as u32) << 96)
            | (u128::from(parts[1] as u32) << 64)
            | (u128::from(parts[2] as u32) << 32)
            | u128::from(parts[3] as u32),
    )
}

#[derive(Default)]
struct MoleFlee {
    threat: Option<([f64; 3], u32)>,
    recalc: u8,
}
impl mod_api::Goal for MoleFlee {
    fn flags(&self) -> mod_api::GoalFlags {
        mod_api::GoalFlags::MOVE
    }
    fn can_use(&mut self, cx: &mut mod_api::GoalContext<'_>) -> bool {
        let mob = cx.mob();
        let own = mob.entity();
        let (Some(level), Some(pos)) = (own.level(cx.host()), own.position(cx.host())) else {
            return false;
        };
        let nearby = level
            .entities_in(
                cx.host(),
                [pos[0] - 6.0, pos[1] - 6.0, pos[2] - 6.0],
                [pos[0] + 6.0, pos[1] + 6.0, pos[2] + 6.0],
            )
            .unwrap_or_default();
        self.threat = nearby
            .into_iter()
            .filter(|other| {
                *other != own
                    && other.kind(cx.host()).is_some_and(|kind| {
                        let kind = kind.id(cx.host());
                        kind == "minecraft:player"
                            || (kind != "naturalist:mole"
                                && (cx.host().mob(*other).is_some()
                                    || super::is_undead_kind(&kind)))
                    })
            })
            .filter_map(|other| {
                let kind = other.kind(cx.host())?.id(cx.host());
                if let Some(living) = cx.host().living(other)
                    && living.health(cx.host()).is_some_and(|health| health <= 0.0)
                {
                    return None;
                }
                // A mob can be temporarily unavailable through Host while its own
                // tick runs; its kind and level position are still enough to flee.
                if cx.host().living(other).is_none() && !super::is_undead_kind(&kind) {
                    return None;
                }
                let at = other.position(cx.host())?;
                let distance = (0..3).map(|i| (pos[i] - at[i]).powi(2)).sum::<f64>();
                (distance <= 36.0).then_some((distance, at, other.id()))
            })
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .map(|(_, at, id)| (at, id));
        self.threat.is_some()
    }
    fn can_continue(&mut self, cx: &mut mod_api::GoalContext<'_>) -> bool {
        let Some((_, target)) = self.threat else {
            return false;
        };
        let Some(entity) = cx.host().entity(target) else {
            return false;
        };
        let Some(living) = cx.host().living(entity) else {
            return false;
        };
        if living.alive(cx.host()) != Some(true) {
            return false;
        }
        let Some(at) = entity.position(cx.host()) else {
            return false;
        };
        self.threat = Some((at, target));
        self.threat.is_some_and(|(at, _)| {
            cx.mob()
                .entity()
                .position(cx.host())
                .is_some_and(|pos| (0..3).map(|i| (pos[i] - at[i]).powi(2)).sum::<f64>() <= 36.0)
        })
    }
    fn start(&mut self, cx: &mut mod_api::GoalContext<'_>) {
        self.recalc = 0;
        self.flee(cx);
    }
    fn tick(&mut self, cx: &mut mod_api::GoalContext<'_>) {
        self.recalc = self.recalc.saturating_sub(1);
        if self.recalc == 0 {
            self.recalc = 10;
            self.flee(cx);
        }
    }
    fn stop(&mut self, cx: &mut mod_api::GoalContext<'_>) {
        self.threat = None;
        cx.stop_navigation();
    }
}
impl MoleFlee {
    fn flee(&self, cx: &mut mod_api::GoalContext<'_>) {
        let (Some((threat, _)), Some(pos)) = (self.threat, cx.mob().entity().position(cx.host()))
        else {
            return;
        };
        let dx = pos[0] - threat[0];
        let dz = pos[2] - threat[2];
        let len = (dx * dx + dz * dz).sqrt().max(0.001);
        let distance = (0..3).map(|i| (pos[i] - threat[i]).powi(2)).sum::<f64>();
        let speed = if distance < 9.0 { 1.5 } else { 1.0 };
        let _ = cx.move_to(
            [pos[0] + dx / len * 8.0, pos[1], pos[2] + dz / len * 8.0],
            speed,
        );
    }
}

#[derive(Default)]
struct MoleScurry {
    destination: Option<[f64; 3]>,
}
impl mod_api::Goal for MoleScurry {
    fn flags(&self) -> mod_api::GoalFlags {
        mod_api::GoalFlags::MOVE
    }
    fn can_use(&mut self, cx: &mut mod_api::GoalContext<'_>) -> bool {
        if !mole_is_free(cx.mob().entity().id()) {
            return false;
        }
        let own = cx.mob().entity();
        let (Some(level), Some(pos)) = (own.level(cx.host()), own.position(cx.host())) else {
            return false;
        };
        for _ in 0..10 {
            let x = cx.random().next_int_bound(11).unwrap_or(5) - 5;
            let y = cx.random().next_int_bound(9).unwrap_or(4) - 4;
            let z = cx.random().next_int_bound(11).unwrap_or(5) - 5;
            let block = [
                pos[0].floor() as i32 + x,
                pos[1].floor() as i32 + y,
                pos[2].floor() as i32 + z,
            ];
            if !level
                .block_face_sturdy(
                    cx.host(),
                    [block[0], block[1] - 1, block[2]],
                    mod_api::BlockFace::Up,
                    mod_api::SupportType::Full,
                )
                .unwrap_or(false)
            {
                continue;
            }
            let clear = level.block_state(cx.host(), block).is_some_and(|s| {
                level
                    .block_shape(cx.host(), s, block, mod_api::BlockShapeKind::Collision)
                    .is_some_and(|boxes| boxes.is_empty())
            });
            if clear {
                self.destination = Some([
                    block[0] as f64 + 0.5,
                    block[1] as f64,
                    block[2] as f64 + 0.5,
                ]);
                return true;
            }
        }
        false
    }
    fn can_continue(&mut self, cx: &mut mod_api::GoalContext<'_>) -> bool {
        mole_is_free(cx.mob().entity().id()) && cx.mob().is_done(cx.host()) == Some(false)
    }
    fn start(&mut self, cx: &mut mod_api::GoalContext<'_>) {
        if let Some(at) = self.destination {
            let _ = cx.move_to(at, 1.5);
        }
    }
    fn stop(&mut self, cx: &mut mod_api::GoalContext<'_>) {
        self.destination = None;
        cx.stop_navigation();
    }
}

#[derive(Default)]
struct WolfMoleTarget {
    target: Option<mod_api::LivingRef>,
    cooldown: u8,
}
impl mod_api::Goal for WolfMoleTarget {
    fn flags(&self) -> mod_api::GoalFlags {
        mod_api::GoalFlags::TARGET
    }
    fn can_use(&mut self, cx: &mut mod_api::GoalContext<'_>) -> bool {
        let mob = cx.mob();
        let wolf = mob.entity();
        if mob.target(cx.host()).is_some()
            || pet_goal_state(cx, wolf).is_some_and(|state| state.tamed)
        {
            return false;
        }
        self.cooldown = self.cooldown.saturating_sub(1);
        if self.cooldown > 0 {
            return false;
        }
        self.cooldown = 10;
        let (Some(level), Some(pos), Some(follow)) = (
            wolf.level(cx.host()),
            wolf.position(cx.host()),
            cx.host()
                .living(wolf)
                .and_then(|l| l.attribute_value(cx.host(), "minecraft:follow_range")),
        ) else {
            return false;
        };
        let nearby = level
            .entities_in(
                cx.host(),
                [pos[0] - follow, pos[1] - follow, pos[2] - follow],
                [pos[0] + follow, pos[1] + follow, pos[2] + follow],
            )
            .unwrap_or_default();
        self.target = nearby
            .into_iter()
            .filter(|e| {
                e.kind(cx.host())
                    .is_some_and(|k| k.id(cx.host()) == "naturalist:mole")
                    && mole_can_be_dug(e.id())
            })
            .filter_map(|e| cx.host().living(e))
            .find(|l| l.alive(cx.host()) == Some(true));
        self.target.is_some()
    }
    fn start(&mut self, cx: &mut mod_api::GoalContext<'_>) {
        let _ = cx.set_target(self.target);
    }
    fn can_continue(&mut self, cx: &mut mod_api::GoalContext<'_>) -> bool {
        self.target.is_some_and(|t| {
            t.alive(cx.host()) == Some(true) && cx.mob().target(cx.host()) == Some(t)
        })
    }
    fn stop(&mut self, cx: &mut mod_api::GoalContext<'_>) {
        if self.target.is_some() && cx.mob().target(cx.host()) == self.target {
            let _ = cx.set_target(None);
        }
        self.target = None;
    }
}

#[derive(Default)]
struct WolfDig {
    mole: Option<u32>,
    ticks: u8,
}
impl mod_api::Goal for WolfDig {
    fn flags(&self) -> mod_api::GoalFlags {
        mod_api::GoalFlags::MOVE
            .union(mod_api::GoalFlags::LOOK)
            .union(mod_api::GoalFlags::JUMP)
    }
    fn can_use(&mut self, cx: &mut mod_api::GoalContext<'_>) -> bool {
        let Some(target) = cx.mob().target(cx.host()) else {
            return false;
        };
        let entity = target.entity();
        let (Some(wolf), Some(mole)) = (
            cx.mob().entity().position(cx.host()),
            entity.position(cx.host()),
        ) else {
            return false;
        };
        let d = (0..3).map(|i| (wolf[i] - mole[i]).powi(2)).sum::<f64>();
        if entity
            .kind(cx.host())
            .is_some_and(|k| k.id(cx.host()) == "naturalist:mole")
            && mole_can_be_dug(entity.id())
            && cx.mob().entity().on_ground(cx.host()) == Some(true)
            && d <= 4.0
        {
            self.mole = Some(entity.id());
            true
        } else {
            false
        }
    }
    fn can_continue(&mut self, cx: &mut mod_api::GoalContext<'_>) -> bool {
        let (Some(id), Some(target)) = (self.mole, cx.mob().target(cx.host())) else {
            return false;
        };
        let (Some(wolf), Some(mole)) = (
            cx.mob().entity().position(cx.host()),
            target.entity().position(cx.host()),
        ) else {
            return false;
        };
        target.entity().id() == id
            && mole_can_be_dug(id)
            && self.ticks < 50
            && (0..3).map(|i| (wolf[i] - mole[i]).powi(2)).sum::<f64>() <= 6.25
    }
    fn start(&mut self, cx: &mut mod_api::GoalContext<'_>) {
        self.ticks = 0;
        cx.stop_navigation();
    }
    fn tick(&mut self, cx: &mut mod_api::GoalContext<'_>) {
        let Some(id) = self.mole else {
            return;
        };
        let Some(target) = cx.mob().target(cx.host()) else {
            return;
        };
        let Some(mole) = target.entity().position(cx.host()) else {
            return;
        };
        cx.stop_navigation();
        cx.look_at(mole);
        self.ticks = self.ticks.saturating_add(1);
        if let Ok(mut map) = MOLE_STATES.lock() {
            if let Some(s) = map.get_mut(&id) {
                s.dug_out_hold_ticks = 3;
                if self.ticks >= 50 {
                    s.pop_out_pending = true;
                    s.attacker = Some(cx.mob().entity().id());
                    s.attacker_ticks = 1200;
                    s.threat_detected = true;
                }
            }
        }
    }
    fn stop(&mut self, cx: &mut mod_api::GoalContext<'_>) {
        if let Some(id) = self.mole {
            if let Ok(mut map) = MOLE_STATES.lock() {
                if let Some(s) = map.get_mut(&id) {
                    if !s.pop_out_pending {
                        s.dug_out_hold_ticks = 0;
                    }
                }
            }
        }
        self.mole = None;
    }
    fn requires_update_every_tick(&self) -> bool {
        true
    }
}

fn mole_is_free(id: u32) -> bool {
    MOLE_STATES
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(&id)
        .is_some_and(|state| state.state == 2 && state.dug_out_hold_ticks <= 0)
}
fn mole_can_be_dug(id: u32) -> bool {
    MOLE_STATES
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(&id)
        .is_some_and(|state| state.state == 2 && state.grace_ticks <= 0)
}
