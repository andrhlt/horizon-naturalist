fn elephant_attack(mob: &mut Mob, target: Who, context: &mut Cx<'_, '_>) -> std::ops::ControlFlow<()> {
    use super::nav::naturalist_elephants as elephants;
    use std::ops::ControlFlow;

    if !elephants::includes(&mob.kind.id) {
        return ControlFlow::Continue(());
    }
    if mob.spares_target(context.w, target) {
        return ControlFlow::Break(());
    }
    let age = mob.age as u64;
    let swing = &mut elephants::state_mut(mob).swing_until;
    if *swing <= age {
        *swing = age + 25;
    }
    let from = mob.position.components();
    let amount = mob.attrs.attack_damage as f32;
    context.events.push(match target {
        Who::Player => MobEvent::AttackPlayer(amount, from),
        Who::Mob(target) => MobEvent::AttackMob {
            target,
            amount,
            source: Damage::mob(mob.id, from).with_species(mob.species()),
        },
    });
    if !mob.silent {
        context.events.push(MobEvent::Sound("entity.ravager.attack", from, 1.0, 1.0));
    }
    ControlFlow::Break(())
}
