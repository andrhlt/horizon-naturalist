use super::{
    Mob, MobWorld,
    goals::{Cx, Goal, JUMP, LOOK, MOVE},
    nav::Navigation,
};
use horizon_core::random::RandomSource;

pub const KIND: &str = "naturalist:komodo_dragon";
pub const PREY: &str = "naturalist:komodo_dragon_hostiles";

#[derive(Clone, Debug, Default)]
pub struct PlayerProximity {
    ticks: u16,
    hunting: bool,
}

pub(super) fn player_targeting(mob: &Mob) -> bool {
    mob.ai.nav.__horizon_mixins_naturalist_komodo_proximity().hunting
}

pub(super) fn update_proximity(mob: &mut Mob, world: &MobWorld) {
    if mob.is_baby() {
        *mob.ai.nav.__horizon_mixins_naturalist_komodo_proximity_mut() = PlayerProximity::default();
        return;
    }
    if mob.age % 10 != 0 {
        return;
    }
    let nearby = world.player.as_ref().is_some_and(|player| {
        let size = mob.bounds();
        let low = size.min_position();
        let high = size.max_position();
        let p = player.bounds;
        let lo = p.min_position();
        let hi = p.max_position();
        !player.creative && !player.spectator
            && hi.x > low.x - 16.0 && lo.x < high.x + 16.0
            && hi.y > low.y - 16.0 && lo.y < high.y + 16.0
            && hi.z > low.z - 16.0 && lo.z < high.z + 16.0
    });
    let state = mob.ai.nav.__horizon_mixins_naturalist_komodo_proximity_mut();
    if nearby {
        state.ticks = (state.ticks + 10).min(400);
        state.hunting |= state.ticks == 400;
    } else {
        state.ticks = 0;
        if mob.ai.target.is_none() {
            state.hunting = false;
        }
    }
}

pub fn basking(nav: &Navigation) -> bool {
    *nav.__horizon_mixins_naturalist_komodo_basking()
}

pub fn is_prey(nav: &Navigation) -> bool {
    *nav.__horizon_mixins_naturalist_komodo_prey()
}

pub fn set_prey(nav: &mut Navigation, prey: bool) {
    *nav.__horizon_mixins_naturalist_komodo_prey_mut() = prey;
}

#[derive(Clone, Debug, Default)]
pub(super) struct Bask {
    remaining: i32,
}

impl Bask {
    fn allowed(mob: &Mob, world: &MobWorld) -> bool {
        let [x, y, z] = mob.position.components().map(|v| v.floor() as i32);
        world.daylight && mob.ai.target.is_none() && world.fluid(x, y, z) != crate::blocks::Fluid::Water
    }
}

impl Goal for Bask {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }

    fn flags(&self) -> u8 { MOVE | LOOK | JUMP }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        let asleep = basking(&mob.ai.nav);
        mob.ai.xxa == 0.0 && mob.ai.yya == 0.0 && mob.ai.zza == 0.0
            && (asleep || Self::allowed(mob, cx.w))
            && (asleep || mob.random.next_float() < 0.001)
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if !Self::allowed(mob, cx.w) {
            return false;
        }
        self.remaining -= 1;
        self.remaining > 0
    }

    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        *mob.ai.nav.__horizon_mixins_naturalist_komodo_basking_mut() = true;
        mob.ai.jump = false;
        mob.nav_stop();
        let p = mob.position;
        mob.ai.mov.set_wanted(p.x, p.y, p.z, 0.0);
        self.remaining = 160 + mob.random.next_int_bound(340).unwrap_or(0);
    }

    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        *mob.ai.nav.__horizon_mixins_naturalist_komodo_basking_mut() = false;
    }
}
