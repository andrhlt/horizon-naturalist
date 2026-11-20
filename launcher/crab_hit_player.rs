fn crab_hit_player(game: &mut Game<'_>, amount: f32, from: [f64; 3], damage: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::nav::naturalist_crab as crab;
    use std::ops::ControlFlow;

    if damage != "minecraft:mob_attack" {
        return ControlFlow::Continue(());
    }
    let id = match game.combat.melee_mob {
        Some(id) => game.mobs.iter().find(|mob| mob.id == id),
        None => game.mobs.iter().find(|mob| mob.alive() && mob.position.components() == from),
    }.filter(|mob| mob.kind.id == crab::KIND).map(|mob| mob.id);
    let Some(id) = id else { return ControlFlow::Continue(()) };
    let landed = game.hurt_player_from(amount, Some(from), damage, None);
    if landed { game.naturalist_crab_hit(id); }
    ControlFlow::Break(landed)
}
