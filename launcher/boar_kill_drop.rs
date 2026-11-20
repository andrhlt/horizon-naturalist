fn boar_kill_drop(game: &mut Game<'_>, damage: &adv::DamageView) {
    use horizon_core::random::RandomSource;

    if damage.kind != "minecraft:mob_attack" || !damage.is_direct || damage.actual <= 0.0
        || !game.ui.rules.bool("mob_drops")
        || damage.source.as_ref().is_none_or(|source| source.kind != "naturalist:boar")
    {
        return;
    }
    let Some(player) = game.player.as_ref().filter(|player| player.health.dead_or_dying()) else {
        return;
    };
    if !super::naturalist_boar_death_scope::take() {
        return;
    }
    let at = player.position.components();
    let attacker = horizon_client::health::knockback_attacker() as u32;
    if let Some(boar) = game.mobs.iter_mut().find(|mob| mob.id == attacker && mob.kind.id == "naturalist:boar")
        && boar.random.next_float() < 0.1
    {
        game.spawn_item_own(horizon_client::items::ItemStack::new(
            "naturalist:music_disc_death_by_hogs", 1), at, 10);
    }
}
