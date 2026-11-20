fn elephant_hit_mob(
    game: &mut Game<'_>,
    target: u32,
    amount: f32,
    source: horizon_client::mobs::Damage,
) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::nav::naturalist_elephants;
    use std::ops::ControlFlow;

    let Some(Who::Mob(attacker)) = source.attacker else { return ControlFlow::Continue(()) };
    let Some(animal) = game.mobs.iter().find(|mob| mob.id == attacker) else {
        return ControlFlow::Continue(());
    };
    if !naturalist_elephants::includes(&animal.kind.id) {
        return ControlFlow::Continue(());
    }
    let at = animal.position;
    let hurt = game.mob_hits_mob(target, amount, source);
    if hurt {
        let _attacker = game.knockback_scope_for(Some(Who::Mob(attacker)));
        if let Some(victim) = game.mobs.iter_mut().find(|mob| mob.id == target) {
            let x = at.x.floor() - victim.position.x;
            let z = at.z.floor() - victim.position.z;
            victim.knockback_effect(3.0, x, z, &source, amount);
            victim.velocity.y += 0.5 * (1.0 - victim.knockback_resistance()).max(0.0);
        }
    }
    ControlFlow::Break(hurt)
}
