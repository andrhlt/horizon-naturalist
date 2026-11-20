use super::{
    Mob,
    goals::{Cx, Goal, JUMP, LOOK, MOVE},
    nav::Navigation,
};
use horizon_core::random::RandomSource;

// 0 is awake, 1 and 2 are the two independent resting poses.
pub fn pose(navigation: &Navigation) -> u8 {
    *navigation.__horizon_mixins_naturalist_capybara_pose()
}

#[derive(Clone, Debug)]
pub(super) struct Sleep;

impl Goal for Sleep {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }

    fn flags(&self) -> u8 {
        MOVE | LOOK | JUMP
    }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.ai.xxa == 0.0 && mob.ai.yya == 0.0 && mob.ai.zza == 0.0
            && (self.can_continue(mob, cx) || pose(&mob.ai.nav) != 0)
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        let time = cx.w.day_time.rem_euclid(24_000);
        time > 6_000 && time < 13_000 && mob.on_ground && !mob.fluids.in_water
            && !mob.extra.sitting && mob.extra.in_love <= 0 && mob.ai.last_hurt_by.is_none()
    }

    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        if pose(&mob.ai.nav) == 0 {
            *mob.ai.nav.__horizon_mixins_naturalist_capybara_pose_mut() = 1 + u8::from(mob.random.next_boolean());
        }
        mob.ai.jump = false;
        mob.nav_stop();
    }

    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        *mob.ai.nav.__horizon_mixins_naturalist_capybara_pose_mut() = 0;
    }
}
