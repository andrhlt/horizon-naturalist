fn catfish_attack(
    game: &mut Game<'_>,
    target: u32,
    source: Damage,
) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::DamageKind;
    use horizon_core::nbt::Tag;
    if source.kind != DamageKind::Drown
        && game.mobs.iter().any(|mob| mob.id == target && mob.kind.id == "naturalist:clam")
        && !game.persist.raw.get(&target).and_then(|tag| tag.get("NaturalistClamOpen"))
            .is_some_and(|tag| match tag { Tag::Byte(value) => *value != 0, Tag::Int(value) => *value != 0, _ => false })
    {
        return std::ops::ControlFlow::Break(false);
    }
    crate::game::naturalist_catfish::attack(game, target, source)
}
