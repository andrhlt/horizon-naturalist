use super::Game;
use horizon_client::mobs::{Species, goals};

/// Install the Java Anglerfish retaliation and cod-hunting goals once per mob.
///
/// The target selector deliberately does not require a path (`must_reach=false`),
/// matching Naturalist's extended hunt reach in water.  Hurt-by-target remains
/// the higher-priority selector so a damaged fish immediately retaliates.
pub(super) fn install(game: &mut Game<'_>, id: u32) {
    let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) else {
        return;
    };
    if !mob.ai.targets.has("hurt_by_target") {
        mob.ai.targets.add(1, goals::HurtByTarget::new());
    }
    if !mob.ai.targets.has("nearest_target") {
        mob.ai.targets.add(
            2,
            goals::NearestTarget::species(&[Species::Cod], false)
                .interval(10)
                .unseen_memory(300),
        );
    }
    if !mob.ai.goals.has("melee_attack") {
        mob.ai.goals.add(3, goals::MeleeAttack::new(1.2, true));
    }
}
