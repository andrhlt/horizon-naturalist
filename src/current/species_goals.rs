//! Species-specific native goals for pets, moles and digging wolves.
use std::{collections::HashMap, sync::{LazyLock, Mutex}};
use mod_api::RandomSource;

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

pub(crate) static MOLE_STATES: LazyLock<Mutex<HashMap<u32, MoleRuntime>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub struct MolePart;
#[mod_api::horizon_part]
impl mod_api::EntityPart for MolePart {
    fn register_goals(&self, goals: &mut mod_api::GoalRegistrar<'_>) {
        goals.add_goal(2, MoleFlee::default());
        goals.add_goal(3, MoleScurry::default());
    }
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
        let _ = cx;
        false
    }
    fn can_continue(&mut self, cx: &mut mod_api::GoalContext<'_>) -> bool {
        let own = cx.mob().entity();
        let Some(owner) = self.owner else {
            return false;
        };
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
        if !mole_is_free(own.id()) {
            return false;
        }
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
                    && other
                        .kind(cx.host())
                        .is_some_and(|kind| kind.id(cx.host()) != "naturalist:mole")
            })
            .filter_map(|other| {
                let living = cx.host().living(other)?;
                if living.health(cx.host()).unwrap_or(0.0) <= 0.0 {
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
        if !mole_is_free(cx.mob().entity().id()) {
            return false;
        }
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
        if mob.target(cx.host()).is_some() {
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
        .ok()
        .and_then(|m| {
            m.get(&id)
                .map(|s| s.state == 2 && s.dug_out_hold_ticks <= 0)
        })
        .unwrap_or(false)
}
fn mole_can_be_dug(id: u32) -> bool {
    MOLE_STATES
        .lock()
        .ok()
        .and_then(|m| m.get(&id).map(|s| s.state == 2 && s.grace_ticks <= 0))
        .unwrap_or(false)
}
