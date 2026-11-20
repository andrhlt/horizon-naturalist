use super::{Cx, Goal, LOOK, MOVE, Mob, MobEvent, MobWorld, goal_clone, reduced};
use crate::mobs::{
    finalize::{Attr, Op, add_modifier, has_modifier, remove_modifier, set_age},
    nav::Navigation,
    path::PathType,
};
use horizon_core::{geometry::Vec3, random::RandomSource};

pub const KIND: &str = "naturalist:lion";
pub const PREY: &str = "naturalist:lion_hostiles";
pub const CUB_SPEED: &str = "naturalist:baby_speed_boost";
pub const SPAWN_BONUS: &str = "naturalist:random_spawn_bonus";

#[derive(Clone, Copy, Debug)]
enum PrideSpawn {
    Ordinary,
    Group(u32),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Pride {
    pub leader: Option<u32>,
    pub leader_look: [f64; 3],
    spawn: Option<PrideSpawn>,
}

pub fn pride(nav: &Navigation) -> Pride {
    *nav.__horizon_mixins_naturalist_lion_pride()
}

pub fn put_pride(nav: &mut Navigation, leader: Option<u32>, leader_look: [f64; 3]) {
    let state = nav.__horizon_mixins_naturalist_lion_pride_mut();
    state.leader = leader;
    state.leader_look = leader_look;
}

pub fn ranking(uuid: [i32; 4]) -> (i64, i64) {
    let half = |hi: i32, lo: i32| i64::from(hi) << 32 | i64::from(lo as u32);
    (half(uuid[0], uuid[1]), half(uuid[2], uuid[3]))
}

pub fn roll_mane(mob: &mut Mob) {
    mob.extra.flag = mob.random.next_boolean();
}

pub fn update_cub_speed(mob: &mut Mob) {
    let boosted = has_modifier(mob, Attr::MovementSpeed, CUB_SPEED);
    match (mob.is_baby(), boosted) {
        (true, false) => add_modifier(mob, Attr::MovementSpeed, CUB_SPEED, 0.05, Op::AddValue),
        (false, true) => remove_modifier(mob, Attr::MovementSpeed, CUB_SPEED),
        _ => {}
    }
}

pub fn natural_pride_member(mob: &mut Mob, index: u32, level_cub: bool) {
    if index == 0 {
        mob.extra.flag = false;
        return;
    }
    if level_cub {
        set_age(mob, -24000);
    }
    roll_mane(mob);
    if index > 1 && mob.random.next_float() < 0.7 {
        set_age(mob, -24000);
    }
}

pub fn mark_group_member(mob: &mut Mob, index: u32) {
    mob.ai
        .nav
        .__horizon_mixins_naturalist_lion_pride_mut()
        .spawn = Some(PrideSpawn::Group(index));
}

pub fn mark_ordinary_spawn(mob: &mut Mob) {
    mob.ai
        .nav
        .__horizon_mixins_naturalist_lion_pride_mut()
        .spawn = Some(PrideSpawn::Ordinary);
}

pub fn has_pending_spawn(mob: &Mob) -> bool {
    pride(&mob.ai.nav).spawn.is_some()
}

pub fn finish_spawn(mob: &mut Mob, level: &mut impl RandomSource) {
    let pending = mob
        .ai
        .nav
        .__horizon_mixins_naturalist_lion_pride_mut()
        .spawn
        .take();
    let Some(pending) = pending else { return };
    let level_cub =
        matches!(pending, PrideSpawn::Group(index) if index != 0) && level.next_float() <= 0.05;
    crate::mobs::finalize::mob_spawn(mob, level);
    match pending {
        PrideSpawn::Ordinary => roll_mane(mob),
        PrideSpawn::Group(index) => natural_pride_member(mob, index, level_cub),
    }
    apply_spawn_bonus(mob, level);
}

pub fn apply_spawn_bonus(mob: &mut Mob, random: &mut impl RandomSource) {
    let amount = random.triangle(0.0, 0.114_850_000_000_000_01);
    add_modifier(
        mob,
        Attr::FollowRange,
        SPAWN_BONUS,
        amount,
        Op::AddMultipliedBase,
    );
}

#[derive(Clone, Debug, Default)]
pub(super) struct FollowLeader {
    following: Option<u32>,
    recheck: i32,
    old_water_cost: f32,
}

impl Goal for FollowLeader {
    goal_clone!();

    fn flags(&self) -> u8 {
        MOVE | LOOK
    }

    fn can_use(&mut self, mob: &mut Mob, _: &mut Cx) -> bool {
        self.following = if mob.is_baby() {
            None
        } else {
            pride(&mob.ai.nav).leader
        };
        self.following.is_some()
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        !mob.ai.nav.is_done()
            && self
                .following
                .and_then(|id| cx.w.seen(id))
                .is_some_and(|leader| mob.distance_sqr(leader.position) > 64.0)
    }

    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        self.recheck = 0;
        self.old_water_cost = mob.malus(PathType::Water);
        mob.set_malus(PathType::Water, 0.0);
    }

    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        self.following = None;
        mob.nav_stop();
        mob.set_malus(PathType::Water, self.old_water_cost);
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(leader) = self.following.and_then(|id| cx.w.seen(id)) else {
            return;
        };
        if mob.extra.leash.is_some() {
            return;
        }
        let pos = leader.position;
        mob.look_at_speed([pos.x, leader.eye_y, pos.z], 10.0, mob.max_head_x_rot());
        self.recheck -= 1;
        if self.recheck > 0 {
            return;
        }
        self.recheck = reduced(10);
        let distance = mob.distance_sqr(pos);
        if distance <= 64.0 {
            mob.nav_stop();
            let selection = pride(&mob.ai.nav);
            if distance <= 8.0
                || selection.leader == self.following
                    && selection.leader_look == mob.position.components()
            {
                let away = Vec3::new(
                    2.0 * mob.position.x - pos.x,
                    mob.position.y,
                    2.0 * mob.position.z - pos.z,
                );
                mob.nav_move_to(cx.w, away.components(), 1.1);
            }
        } else {
            mob.nav_move_to(cx.w, pos.components(), 1.1);
        }
    }
}
