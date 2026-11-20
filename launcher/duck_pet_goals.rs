use super::{Cx, FollowOwner, Goal, Mob, SitWhenOrdered};
use crate::mobs::nav::naturalist_duck_pet as pet;

pub fn add_goals(mob: &mut Mob) {
    mob.ai.goals.add(1, SitWhenOrdered);
    mob.ai.goals.add(4, FollowPetOwner(FollowOwner::new(1.1, 10.0, 2.0)));
}

pub fn add_bird_goals(mob: &mut Mob) {
    mob.ai.goals.add(3, SitWhenOrdered);
    mob.ai.goals.add(3, FollowPetOwner(FollowOwner::new(1.5, 5.0, 1.0)));
}

pub fn add_capybara_goals(mob: &mut Mob) {
    mob.ai.goals.add(1, CapybaraSitWhenOrdered(SitWhenOrdered));
    mob.ai.goals.add(4, FollowPetOwner(FollowOwner::new(1.2, 10.0, 3.0)));
}

pub fn set_capybara_owner_view(
    mob: &mut Mob,
    owner: Option<std::sync::Arc<crate::mobs::PlayerView>>,
) {
    *mob.ai.nav.__horizon_mixins_naturalist_capybara_owner_view_mut() = owner;
}

fn with_capybara_owner<T>(
    mob: &mut Mob,
    cx: &mut Cx,
    run: impl FnOnce(&mut Mob, &mut Cx) -> T,
) -> T {
    let Some(owner) = mob.ai.nav.__horizon_mixins_naturalist_capybara_owner_view().as_deref()
    else {
        return run(mob, cx);
    };
    if cx.w.player.as_ref().is_some_and(|player| player.uuid == owner.uuid) {
        return run(mob, cx);
    }
    let mut world = (*cx.w).clone();
    world.player = Some(owner.clone());
    let mut owner_context = Cx { w: &world, lr: &mut *cx.lr, events: &mut *cx.events };
    run(mob, &mut owner_context)
}

#[derive(Clone, Debug)]
struct CapybaraSitWhenOrdered(SitWhenOrdered);

impl Goal for CapybaraSitWhenOrdered {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { self.0.flags() }
    fn name(&self) -> &'static str { self.0.name() }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        with_capybara_owner(mob, cx, |mob, cx| self.0.can_use(mob, cx))
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        self.0.can_continue(mob, cx)
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.start(mob, cx); }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.stop(mob, cx); }
}

pub fn add_tortoise_goals(mob: &mut Mob) {
    mob.ai.goals.add(1, SitWhenOrdered);
    mob.ai.goals.add(3, FollowPetOwner(FollowOwner::new(1.0, 10.0, 5.0)));
}

pub fn add_lizard_goals(mob: &mut Mob) {
    mob.ai.goals.add(2, SitWhenOrdered);
    mob.ai.goals.add(6, FollowPetOwner(FollowOwner::new(1.0, 10.0, 2.0)));
}

pub fn add_bear_goals(mob: &mut Mob) {
    mob.ai.goals.add(1, SitWhenOrdered);
    mob.ai.goals.add(5, FollowPetOwner(FollowOwner::new(1.25, 10.0, 6.0)));
    mob.ai.targets.add(1, super::OwnerHurtBy { timestamp: 0 });
    mob.ai.targets.add(1, super::OwnerHurt { timestamp: 0 });
}

pub(in crate::mobs) fn add_hippo_goals(mob: &mut Mob) {
    mob.ai.goals.add(1, SitWhenOrdered);
    mob.ai.goals.add(6, FollowPetOwner(FollowOwner::new(1.25, 10.0, 5.0)));
    mob.ai.targets.add(2, super::OwnerHurtBy { timestamp: 0 });
    mob.ai.targets.add(3, super::OwnerHurt { timestamp: 0 });
}

pub(in crate::mobs) fn add_big_cat_goals(mob: &mut Mob) {
    mob.ai.goals.add(1, SitWhenOrdered);
    mob.ai.goals.add(5, FollowPetOwner(FollowOwner::new(1.3, 7.0, 2.0)));
    mob.ai.targets.add(2, super::OwnerHurtBy { timestamp: 0 });
    mob.ai.targets.add(3, super::OwnerHurt { timestamp: 0 });
}

pub(in crate::mobs) fn add_rat_goals(mob: &mut Mob) {
    mob.ai.goals.add(1, SitWhenOrdered);
    mob.ai.goals.add(7, FollowRatOwner(FollowPetOwner(FollowOwner::new(1.4, 10.0, 3.0))));
}

#[derive(Clone, Debug)]
struct FollowRatOwner(FollowPetOwner);
impl Goal for FollowRatOwner {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { self.0.flags() }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool { self.0.can_use(mob, cx) }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool { self.0.can_continue(mob, cx) }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.start(mob, cx);
        mob.extra.sprinting = true;
    }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.0.stop(mob, cx);
        mob.extra.sprinting = false;
    }
    fn every_tick(&self) -> bool { self.0.every_tick() }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.tick(mob, cx); }
}

pub(in crate::mobs) fn add_snake_goals(mob: &mut Mob) {
    mob.ai.goals.add(1, SitWhenOrdered);
    mob.ai.goals.add(4, FollowPetOwner(FollowOwner::new(1.2, 10.0, 2.0)));
    mob.ai.targets.add(1, super::OwnerHurtBy { timestamp: 0 });
    mob.ai.targets.add(2, super::OwnerHurt { timestamp: 0 });
}

pub(in crate::mobs) fn add_hedgehog_owner_goal(mob: &mut Mob) {
    mob.ai.goals.add(5, FollowPetOwner(FollowOwner::new(1.2, 10.0, 3.0)));
}

#[derive(Clone, Debug)]
struct FollowPetOwner(FollowOwner);

impl Goal for FollowPetOwner {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { self.0.flags() }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        pet::is_following(&mob.ai.nav) && hedgehog_can_follow(mob)
            && with_capybara_owner(mob, cx, |mob, cx| self.0.can_use(mob, cx))
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        pet::is_following(&mob.ai.nav) && hedgehog_can_follow(mob)
            && with_capybara_owner(mob, cx, |mob, cx| self.0.can_continue(mob, cx))
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.start(mob, cx); }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) { self.0.stop(mob, cx); }
    fn every_tick(&self) -> bool { self.0.every_tick() }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        with_capybara_owner(mob, cx, |mob, cx| self.0.tick(mob, cx));
    }
}

fn hedgehog_can_follow(mob: &Mob) -> bool {
    use crate::mobs::naturalist_hedgehog as hedgehog;
    if mob.kind.id != hedgehog::KIND {
        return true;
    }
    let pose = hedgehog::state(mob);
    !pose.rolling && pose.following_cooldown == 0
}

pub fn add_crab_goals(mob: &mut Mob) {
    mob.ai.goals.add(1, SitWhenOrdered);
    mob.ai.goals.add(4, CrabWeapon);
    mob.ai.goals.add(7, FollowPetOwner(FollowOwner::new(1.5, 5.0, 1.0)));
    mob.ai.targets.add(1, super::OwnerHurtBy { timestamp: 0 });
    mob.ai.targets.add(2, super::OwnerHurt { timestamp: 0 });
}

#[derive(Clone, Debug)]
struct CrabWeapon;

impl Goal for CrabWeapon {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { super::super::goals::MOVE | super::super::goals::LOOK }
    fn can_use(&mut self, mob: &mut Mob, _: &mut Cx) -> bool {
        use crate::mobs::nav::naturalist_crab as crab;
        !mob.extra.sitting_pose && !crab::holding_weapon(mob) && crab::state(&mob.ai.nav).nearest.is_some()
    }
    fn can_continue(&mut self, mob: &mut Mob, _: &mut Cx) -> bool {
        use crate::mobs::nav::naturalist_crab as crab;
        !crab::holding_weapon(mob) && crab::state(&mob.ai.nav).weapon.is_some()
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        use crate::mobs::nav::naturalist_crab as crab;
        let state = crab::state_mut(&mut mob.ai.nav);
        if let Some(weapon) = state.nearest {
            state.target = Some(weapon.id);
            state.weapon = Some(weapon);
            mob.nav_move_to(cx.w, weapon.at, 1.3);
        }
    }
    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        use crate::mobs::nav::naturalist_crab as crab;
        let state = crab::state_mut(&mut mob.ai.nav);
        state.target = None;
        state.weapon = None;
        mob.nav_stop();
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        use crate::mobs::nav::naturalist_crab as crab;
        if let Some(weapon) = crab::state(&mob.ai.nav).weapon {
            mob.look_at_speed(weapon.at, 30.0, 30.0);
            if mob.ai.nav.path.is_none() {
                mob.nav_move_to(cx.w, weapon.at, 1.3);
            }
        }
    }
}

pub(in crate::mobs) fn add_ostrich_goals(mob: &mut Mob) {
    mob.ai.goals.add(1, SitWhenOrdered);
    mob.ai.goals.add(6, FollowPetOwner(FollowOwner::new(1.2, 10.0, 3.0)));
}
