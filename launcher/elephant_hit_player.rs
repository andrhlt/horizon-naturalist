fn elephant_hit_player(
    game: &mut Game<'_>,
    amount: f32,
    from: [f64; 3],
    damage: &str,
) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::nav::naturalist_elephants;
    use std::ops::ControlFlow;

    if damage != "minecraft:mob_attack" {
        return ControlFlow::Continue(());
    }
    let animal = match game.combat.melee_mob {
        Some(id) => game.mobs.iter().find(|mob| mob.id == id),
        None => game.mobs.iter().find(|mob| mob.alive() && mob.position.components() == from),
    };
    let Some(animal) = animal.filter(|mob| naturalist_elephants::includes(&mob.kind.id)) else {
        return ControlFlow::Continue(());
    };
    let id = animal.id;
    let shielded = game.player_blocking(Some(from), damage, false)
        .is_some_and(|block| block.blocked(amount.max(1.0)) > 0.0);
    let hurt = game.hurt_player_from(amount, Some(from), damage, Some(Some(id)));
    if hurt && let Some(player) = game.player.as_mut() {
        let x = from[0].floor() - player.position.x;
        let z = from[2].floor() - player.position.z;
        let resistance = player.attribute("minecraft:knockback_resistance").unwrap_or(0.0);
        game.mob_pushes_player(
            if shielded { 1.5 } else { 3.0 }, x, z, Damage::mob(id, from), None,
        );
        if let Some(player) = game.player.as_mut() {
            player.velocity.y += 0.5 * (1.0 - resistance).max(0.0);
        }
    }
    ControlFlow::Break(hurt)
}
