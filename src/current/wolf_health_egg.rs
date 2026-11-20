fn wolf_health_egg(
    game: &mut Game<'_>,
    target: crate::game::commands::Target,
    tag: crate::game::naturalist_wolf_health::Nbt,
    original: impl FnOnce(&mut Game<'_>, crate::game::commands::Target, crate::game::naturalist_wolf_health::Nbt) -> crate::game::commands::R<()>,
) -> crate::game::commands::R<()> {
    let raw = tag.clone();
    let result = original(game, target, tag);
    if result.is_ok()
        && let crate::game::commands::Target::Mob(id) = target
    {
        let kind = game
            .mobs
            .iter()
            .find(|mob| mob.id == id)
            .map(|mob| mob.kind.id.to_string());
        if let Some(kind) = kind {
            crate::game::naturalist_wolf_health::restore(game, id, &kind, Some(&raw));
        }
    }
    result
}
