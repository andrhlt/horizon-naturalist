use super::{Mob, MobWorld, nav::naturalist_deer_herd_turn::turn};
use horizon_core::{
    geometry::{Aabb, Vec3},
    random::RandomSource,
};
use std::sync::{Arc, Mutex};

pub const DEER: &str = "naturalist:deer";
// Extra::ints is species scratch storage and is unused by the cow template.
const PANIC_TIMER: usize = 0;
const PENDING_ROLL: usize = 1;

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    source: u32,
    reach: Aabb,
    ticks: i32,
}

impl Hit {
    fn at(mob: &Mob, ticks: i32) -> Self {
        Self {
            source: mob.id,
            reach: mob.bounds().inflate(Vec3::new(8.0, 4.0, 8.0)),
            ticks,
        }
    }

    pub fn share(self, mob: &mut Mob) {
        if mob.id != self.source
            && mob.kind.id == DEER
            && !mob.removed
            && self.reach.intersects(mob.bounds())
        {
            mob.extra.ints[PANIC_TIMER] = self.ticks;
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Round(Arc<Mutex<Vec<(usize, Hit)>>>);

#[derive(Clone, Debug)]
pub struct Turn {
    round: Round,
    group: Option<usize>,
    emitted: Vec<Hit>,
}

pub fn prepare_turn(mob: &mut Mob, round: Round) {
    *turn(&mut mob.ai.nav) = Some(Turn {
        round,
        group: None,
        emitted: Vec::new(),
    });
}

pub fn enter_turn(mob: &mut Mob, world: &MobWorld<'_>) {
    let Some(state) = turn(&mut mob.ai.nav).as_mut() else {
        return;
    };
    // Every deer within sharing range is in the same ordered mob island.
    // Use its snapshot's identity to keep separate islands and the optional
    // serial verification pass from observing one another's hits.
    let group = world.others.as_ptr() as usize;
    state.group = Some(group);
    let round = state.round.clone();
    let hits = round.0.lock().unwrap_or_else(|error| error.into_inner());
    for &(_, hit) in hits.iter().filter(|(key, _)| *key == group) {
        hit.share(mob);
    }
}

pub fn successful_hit(mob: &mut Mob) {
    if mob.kind.id != DEER {
        return;
    }
    let ticks = 100
        + mob
            .random
            .next_int_bound(100)
            .expect("positive fixed bound");
    mob.extra.ints[PANIC_TIMER] = ticks;
    let hit = Hit::at(mob, ticks);
    if let Some(state) = turn(&mut mob.ai.nav).as_mut()
        && let Some(group) = state.group
    {
        state
            .round
            .0
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .push((group, hit));
        state.emitted.push(hit);
    } else {
        mob.extra.ints[PENDING_ROLL] = ticks;
    }
}

pub fn finish_tick(mob: &mut Mob) {
    if let Some(state) = turn(&mut mob.ai.nav).as_mut() {
        state.group = None;
    }
    let timer = &mut mob.extra.ints[PANIC_TIMER];
    if *timer >= 0 {
        *timer -= 1;
    }
    if *timer == 0 {
        mob.ai.last_hurt_by = None;
        mob.ai.last_hurt_by_ref = None;
    }
}

pub fn take_turn_hits(mob: &mut Mob) -> Vec<Hit> {
    if mob.kind.id == DEER {
        turn(&mut mob.ai.nav).take().map_or_else(Vec::new, |state| state.emitted)
    } else {
        Vec::new()
    }
}

pub fn take_roll(mob: &mut Mob) -> Option<Hit> {
    if mob.kind.id != DEER {
        return None;
    }
    let ticks = std::mem::take(&mut mob.extra.ints[PENDING_ROLL]);
    (ticks > 0).then(|| Hit::at(mob, ticks))
}
