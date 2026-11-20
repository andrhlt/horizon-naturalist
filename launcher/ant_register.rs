fn ant_register(mob: &mut Mob) -> std::ops::ControlFlow<()> {
    use super::{
        goals::naturalist_ant_goals::{ForageHill, Gather},
        naturalist_ant::ANT,
        nav::NavKind,
    };
    use std::ops::ControlFlow;
    if mob.kind.id != ANT {
        return ControlFlow::Continue(());
    }
    let kind = std::sync::Arc::make_mut(&mut mob.kind);
    kind.flags.climber = true;
    kind.food = Default::default();
    mob.ai.nav.kind = NavKind::WallClimber;
    mob.ai.goals.add(0, Float);
    mob.ai.goals.add(1, MeleeAttack::new(1.15, true));
    mob.ai.goals.add(2, ForageHill::new(true));
    mob.ai.goals.add(3, Gather);
    mob.ai.goals.add(4, ForageHill::new(false));
    mob.ai.goals.add(5, Stroll::new(1.0));
    mob.ai.goals.add(6, LookAtPlayer::new(6.0));
    mob.ai.goals.add(7, RandomLookAround::default());
    mob.ai.targets.add(0, OwnerHurtBy { timestamp: 0 });
    mob.ai.targets.add(1, OwnerHurt { timestamp: 0 });
    mob.ai.targets.add(
        2,
        HurtByTarget::new()
            .alert()
            .alert_if(|seen| seen.effects.contains(&ANT)),
    );
    mob.ai
        .targets
        .add(3, NearestTarget::player(true).when(angry_at_player));
    mob.ai
        .targets
        .add(4, super::golem::ResetUniversalAnger::default());
    ControlFlow::Break(())
}
