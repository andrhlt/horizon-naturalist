use super::{Mob, goals::{Cx, Goal, Tempt}, nav::Navigation};
use horizon_core::random::RandomSource;

pub fn missing_tail(nav: &Navigation) -> bool {
    nav.__horizon_mixins_naturalist_lizard_tail_regrow().is_some()
}

pub fn set_tail(nav: &mut Navigation, has_tail: bool) {
    *nav.__horizon_mixins_naturalist_lizard_tail_regrow_mut() = (!has_tail).then_some(0);
}

pub fn mark_tail_dropped(nav: &mut Navigation) {
    *nav.__horizon_mixins_naturalist_lizard_tail_regrow_mut() = Some(12_000);
    *nav.__horizon_mixins_naturalist_lizard_tail_pending_mut() = true;
}

pub fn take_tail_drop(nav: &mut Navigation) -> bool {
    std::mem::take(nav.__horizon_mixins_naturalist_lizard_tail_pending_mut())
}

pub fn tick_regrow(nav: &mut Navigation) -> bool {
    let state = nav.__horizon_mixins_naturalist_lizard_tail_regrow_mut();
    match state {
        Some(0) => {
            *state = None;
            true
        }
        Some(remaining) => {
            *remaining -= 1;
            false
        }
        None => false,
    }
}

pub fn tame_attributes(mob: &mut Mob, restore_health: bool) {
    use super::finalize::{Attr, set_base};
    set_base(mob, Attr::MaxHealth, 20.0);
    set_base(mob, Attr::AttackDamage, 4.0);
    if restore_health {
        mob.health = 20.0;
    }
}

#[derive(Clone, Debug)]
pub(super) struct LizardTempt {
    goal: Tempt,
    familiar: Option<[i32; 4]>,
}

impl LizardTempt {
    pub(super) fn new(items: std::sync::Arc<[String]>) -> Self {
        Self { goal: Tempt::new(0.6, items, true), familiar: None }
    }
}

impl Goal for LizardTempt {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }

    fn flags(&self) -> u8 { self.goal.flags() }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        self.goal.can_use(mob, cx) && !mob.extra.tame
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        self.goal.can_scare = self.familiar.is_none()
            || cx.w.player.as_ref().is_none_or(|player| Some(player.uuid) != self.familiar);
        !mob.extra.tame && self.goal.can_continue(mob, cx)
    }

    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) { self.goal.start(mob, cx); }

    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) { self.goal.stop(mob, cx); }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.goal.tick(mob, cx);
        if self.familiar.is_none() && mob.random.next_int_bound(300).unwrap_or(1) == 0 {
            self.familiar = cx.w.player.as_ref().map(|player| player.uuid);
        } else if mob.random.next_int_bound(250).unwrap_or(1) == 0 {
            self.familiar = None;
        }
    }
}
