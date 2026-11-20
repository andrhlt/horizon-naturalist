fn bear_kill(game: &mut Game<'_>, victim: &Mob) {
    use horizon_client::mobs::naturalist_bears as bear;
    if let Some((source, _)) = victim.last_damage
        && let Some(Who::Mob(id)) = source.attacker
        && let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id && bear::is_bear(&mob.kind.id))
    {
        bear::state_mut(&mut mob.ai.nav).hunting = 2400;
    }
}
