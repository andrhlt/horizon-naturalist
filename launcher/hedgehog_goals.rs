use super::{
    Breed, Cx, Float, Goal, JUMP, LOOK, LookAtPlayer, MOVE, Mob, Panic,
    RandomLookAround, Stroll, Tempt, goal_clone,
};
use crate::mobs::naturalist_hedgehog as hedgehog;

pub fn register(mob: &mut Mob) {
    let kind = std::sync::Arc::make_mut(&mut mob.kind);
    kind.food = hedgehog::FOOD.into_iter().map(str::to_owned).collect();
    kind.tempt = kind.food.clone();
    mob.ai.goals.add(0, Float);
    mob.ai.goals.add(1, GroundRoll(Panic::new(2.0)));
    mob.ai.goals.add(2, Hide);
    mob.ai.goals.add(3, Breed::new(1.0));
    mob.ai.goals.add(4, Tempt::new(1.5, mob.kind.food.clone(), false));
    mob.ai.goals.add(5, FollowYoung::default());
    crate::mobs::tame::naturalist_duck_pet_goals::add_hedgehog_owner_goal(mob);
    mob.ai.goals.add(6, Stroll::avoiding_water(1.0));
    mob.ai.goals.add(7, LookAtPlayer::new(6.0));
    mob.ai.goals.add(8, RandomLookAround::default());
}

#[derive(Clone, Debug)]
struct Hide;

impl Goal for Hide {
    goal_clone!();

    fn flags(&self) -> u8 {
        MOVE | LOOK | JUMP
    }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        hedgehog::can_hide(mob, cx.w)
    }

    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        mob.nav_stop();
        mob.ai.jump = false;
        mob.ai.mov.set_wanted(mob.position.x, mob.position.y, mob.position.z, 0.0);
    }
}

#[derive(Clone, Debug)]
struct GroundRoll(Panic);

impl Goal for GroundRoll {
    goal_clone!();

    fn flags(&self) -> u8 {
        self.0.flags()
    }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        !hedgehog::state(mob).rolling && self.0.can_use(mob, cx)
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        !hedgehog::state(mob).rolling && self.0.can_continue(mob, cx)
    }

    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.start(mob, cx);
        hedgehog::state_mut(mob).ground_roll = true;
    }

    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.stop(mob, cx);
        hedgehog::state_mut(mob).ground_roll = false;
    }
}

#[derive(Clone, Debug, Default)]
struct FollowYoung {
    parent: Option<u32>,
    recalc: i32,
}

impl Goal for FollowYoung {
    goal_clone!();

    fn flags(&self) -> u8 { 0 }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        use horizon_core::geometry::Vec3;
        if !mob.is_baby() || mob.extra.tame {
            return false;
        }
        let area = mob.bounds().inflate(Vec3::new(16.0, 8.0, 16.0));
        let adult = cx.w.others_near(area)
            .filter(|adult| !adult.baby && area.intersects(adult.bounds)
                && adult.effects.contains(&hedgehog::KIND))
            .reduce(|previous, adult| {
                if mob.distance_sqr(adult.position) <= mob.distance_sqr(previous.position) {
                    adult
                } else {
                    previous
                }
            });
        self.parent = adult.filter(|adult| mob.distance_sqr(adult.position) >= 25.0)
            .map(|adult| adult.id);
        self.parent.is_some()
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.is_baby() && !mob.extra.tame && self.parent.and_then(|id| cx.w.seen(id))
            .is_some_and(|adult| adult.alive && (25.0..=256.0).contains(&mob.distance_sqr(adult.position)))
    }

    fn start(&mut self, _: &mut Mob, _: &mut Cx) {
        self.recalc = 0;
    }

    fn stop(&mut self, _: &mut Mob, _: &mut Cx) {
        self.parent = None;
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.recalc -= 1;
        if self.recalc <= 0 {
            self.recalc = super::reduced(10);
            if let Some(adult) = self.parent.and_then(|id| cx.w.seen(id)) {
                mob.nav_move_to_entity(cx.w, adult.position.components(), 1, 1.1);
            }
        }
    }
}
