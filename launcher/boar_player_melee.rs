fn boar_player_melee(
    game: &mut Game<'_>, amount: f32, from: [f64; 3], damage_type: &str,
) -> std::ops::ControlFlow<bool> {
    if damage_type != "minecraft:mob_attack"
        || game.player.as_ref().is_none_or(|player| player.health.dead_or_dying())
    {
        return std::ops::ControlFlow::Continue(());
    }
    let _scope = super::naturalist_boar_death_scope::Scope::enter();
    std::ops::ControlFlow::Break(game.hurt_player_from(amount, Some(from), damage_type, None))
}
