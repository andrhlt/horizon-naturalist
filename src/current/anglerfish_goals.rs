use super::Game;
use horizon_client::mobs::goals::{Cx, Goal, LOOK, MOVE};
use horizon_client::mobs::{Mob, MobWorld, Species, Who, animal_brain, goals};

fn acquire_cod(mob: &mut Mob, cx: &Cx) -> Option<Who> {
    if let Some(target) = mob.ai.target
        && cx.w.target_info(target).is_some_and(|t| t.alive)
    {
        return Some(target);
    }
    let pos = horizon_core::geometry::Vec3::new(mob.position.x, mob.eye_y(), mob.position.z);
    let id = cx
        .w
        .nearby_species(pos, 16.0, &[Species::Cod], |seen| seen.id != mob.id)
        .next()
        .map(|seen| seen.id)?;
    let target = Who::Mob(id);
    mob.set_ai_target(cx.w, Some(target));
    Some(target)
}

fn hunt_ready(mob: &Mob, _: &MobWorld<'_>) -> bool {
    animal_brain::hunting_cooldown(mob).is_none()
}

#[derive(Clone, Debug, Default)]
struct AnglerfishAttack {
    until_attack: i32,
}

impl Goal for AnglerfishAttack {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }

    fn flags(&self) -> u8 {
        MOVE | LOOK
    }

    fn name(&self) -> &'static str {
        "NaturalistAnglerfishAttack"
    }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        acquire_cod(mob, cx)
            .and_then(|target| cx.w.target_info(target))
            .is_some_and(|target| target.alive && mob.distance_sqr(target.position) <= 256.0)
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        self.can_use(mob, cx)
    }

    fn every_tick(&self) -> bool {
        true
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(who) = acquire_cod(mob, cx) else {
            return;
        };
        let Some(target) = cx.w.target_info(who) else {
            return;
        };
        mob.look_at_speed(
            [target.position.x, target.eye_y, target.position.z],
            30.0,
            30.0,
        );
        self.until_attack = (self.until_attack - 1).max(0);
        if self.until_attack == 0
            && target.alive
            && mob.distance_sqr(target.position) <= 256.0
        {
            self.until_attack = 20;
            mob.swing();
            horizon_client::mobs::ai::do_hurt_target(mob, who, cx);
        }
    }
}

/// Install the Java Anglerfish retaliation and cod-hunting goals once per mob.
///
/// The target selector deliberately does not require a path (`must_reach=false`),
/// matching Naturalist's extended hunt reach in water.  Hurt-by-target remains
/// the higher-priority selector so a damaged fish immediately retaliates.
pub(super) fn install(game: &mut Game<'_>, id: u32) {
    let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) else {
        return;
    };
    if mob.extra.animal_brain.is_none() {
        animal_brain::init(mob, i64::from(id));
    }
    if !mob.ai.targets.has("HurtByTarget") {
        mob.ai.targets.add(1, goals::HurtByTarget::new());
    }
    if !mob.ai.targets.has("NearestTarget") {
        mob.ai.targets.add(
            2,
            goals::NearestTarget::species(&[Species::Cod], false)
                .interval(10)
                .unseen_memory(300)
                .when(hunt_ready),
        );
    }
    if !mob.ai.goals.has("MeleeAttack") {
        mob.ai.goals.add(3, goals::MeleeAttack::new(1.2, true));
    }
    if !mob.ai.goals.has("NaturalistAnglerfishAttack") {
        mob.ai.goals.add(-1, AnglerfishAttack::default());
    }
}
