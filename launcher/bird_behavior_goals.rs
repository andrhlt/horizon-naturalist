use super::super::nav::naturalist_bird_navigation::{bird_look, foraging, section_order};
use super::super::path::PathType;
use super::{Cx, Goal, LOOK, MOVE, Mob, Stroll, goal_clone, reduced};
use horizon_core::geometry::Vec3;

pub fn register(mob: &mut Mob) {
    mob.ai.goals.add(1, BirdEatSeedsGoal::default());
    mob.ai.goals.add(4, BirdAvoidPlayerGoal::default());
    mob.ai.goals.add(5, BirdWanderGoal(Stroll::parrot(1.0)));
    mob.ai.goals.add(6, BirdFlockGoal::default());
}

#[derive(Clone, Debug, Default)]
struct BirdAvoidPlayerGoal {
    target: Option<([i32; 4], Vec3)>,
}

impl Goal for BirdAvoidPlayerGoal {
    goal_clone!();

    fn flags(&self) -> u8 {
        MOVE
    }

    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        let Some(player) = cx.w.player.as_ref() else { return false };
        if m.extra.tame || player.spectator || player.sneaking
            || !m.can_target_player(cx.w, 8.0, true)
        {
            return false;
        }

        let dx = m.position.x - player.position.x;
        let dz = m.position.z - player.position.z;
        let radians = f64::from(std::f32::consts::FRAC_PI_2);
        let Some(destination) = m.hover_random_pos(cx.w, 16, 7, dx, dz, radians, 3, 1)
            .or_else(|| m.air_and_water_random_pos(cx.w, 16, 4, -2, dx, dz))
        else {
            return false;
        };
        let path = m.create_path_to(cx.w, destination.map(|n| n.floor() as i32), 0);
        if path.is_none() {
            return false;
        }
        m.ai.pending_path = path;
        self.target = Some((player.uuid, player.position));
        true
    }

    fn can_continue(&mut self, m: &mut Mob, _: &mut Cx) -> bool {
        !m.extra.tame && !m.ai.nav.is_done()
    }

    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        let path = m.ai.pending_path.take();
        m.nav_move_to_path(cx.w, path, 1.4);
    }

    fn stop(&mut self, _: &mut Mob, _: &mut Cx) {
        self.target = None;
    }

    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        if let Some((id, position)) = &mut self.target {
            if let Some(player) = cx.w.player.as_ref().filter(|player| &player.uuid == id) {
                *position = player.position;
            }
            m.ai.nav.speed_modifier = if m.distance_sqr(*position) < 49.0 { 1.8 } else { 1.4 };
        }
    }
}

#[derive(Clone, Debug)]
struct BirdWanderGoal(Stroll);

impl Goal for BirdWanderGoal {
    goal_clone!();

    fn flags(&self) -> u8 {
        self.0.flags()
    }

    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        !m.extra.tame && self.0.can_use(m, cx)
    }

    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        !m.extra.tame && self.0.can_continue(m, cx)
    }

    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.start(m, cx);
    }

    fn stop(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.stop(m, cx);
    }
}

#[derive(Clone, Debug, Default)]
struct BirdEatSeedsGoal {
    cooldown: i32,
}

impl Goal for BirdEatSeedsGoal {
    goal_clone!();

    fn flags(&self) -> u8 {
        MOVE | LOOK
    }

    fn can_use(&mut self, m: &mut Mob, _: &mut Cx) -> bool {
        if m.extra.tame {
            return false;
        }
        let Some(state) = foraging(&mut m.ai.nav) else {
            return false;
        };
        state.selected = state.first;
        state.target = state.first.map(|seed| seed.id);
        state.target.is_some()
    }

    fn can_continue(&mut self, m: &mut Mob, _: &mut Cx) -> bool {
        !m.extra.tame && foraging(&mut m.ai.nav).as_ref().is_some_and(|state| state.selected.is_some())
    }

    fn stop(&mut self, m: &mut Mob, _: &mut Cx) {
        if let Some(state) = foraging(&mut m.ai.nav) {
            state.selected = None;
            state.target = None;
        }
        m.nav_stop();
    }

    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        let Some(seed) = foraging(&mut m.ai.nav).as_ref().and_then(|state| state.selected) else {
            return;
        };
        let at = [seed.position.x, seed.position.y + 0.2125, seed.position.z];
        m.look_at_speed(at, 30.0, 30.0);
        m.nav_move_to_entity(cx.w, [seed.position.x, seed.position.y, seed.position.z], 1, 1.0);
        if self.cooldown > 0 {
            self.cooldown -= 1;
        } else if m.distance_sqr(seed.position) < 1.5 {
            self.cooldown = 20;
            if let Some(state) = foraging(&mut m.ai.nav) {
                state.consume = Some(seed.id);
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
struct BirdFlockGoal {
    following: Option<u32>,
    recalc: i32,
    old_water: f32,
}

impl Goal for BirdFlockGoal {
    goal_clone!();

    fn flags(&self) -> u8 {
        MOVE | LOOK
    }

    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if m.extra.tame {
            return false;
        }
        let area = m.bounds().inflate(Vec3::new(12.0, 12.0, 12.0));
        self.following = cx.w.others_near(area).filter(|other| {
            bird_look(other).is_some()
                && !other.baby
                && !other.invisible
                && other.bounds.intersects(area)
        }).min_by_key(|other| section_order(other.section.0, other.section.1))
            .map(|other| other.id);
        self.following.is_some()
    }

    fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        !m.extra.tame && !m.ai.nav.is_done() && self.following
            .and_then(|id| cx.w.seen(id))
            .is_some_and(|other| m.distance_sqr(other.position) > 36.0)
    }

    fn start(&mut self, m: &mut Mob, _: &mut Cx) {
        self.recalc = 0;
        self.old_water = m.malus(PathType::Water);
        m.set_malus(PathType::Water, 0.0);
    }

    fn stop(&mut self, m: &mut Mob, _: &mut Cx) {
        self.following = None;
        m.nav_stop();
        m.set_malus(PathType::Water, self.old_water);
    }

    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        let Some(other) = self.following.and_then(|id| cx.w.seen(id)) else { return };
        if m.extra.leash.is_some() {
            return;
        }
        let p = other.position;
        m.look_at_speed([p.x, other.eye_y, p.z], 10.0, m.max_head_x_rot());
        self.recalc -= 1;
        if self.recalc > 0 {
            return;
        }
        self.recalc = reduced(10);
        let distance = m.distance_sqr(p);
        if distance > 36.0 {
            m.nav_move_to_entity(cx.w, [p.x, p.y, p.z], 1, 1.0);
        } else {
            m.nav_stop();
            if distance <= 6.0 || bird_look(other) == Some([m.position.x, m.position.y, m.position.z]) {
                m.nav_move_to(cx.w, [
                    2.0 * m.position.x - p.x,
                    m.position.y,
                    2.0 * m.position.z - p.z,
                ], 1.0);
            }
        }
    }
}
