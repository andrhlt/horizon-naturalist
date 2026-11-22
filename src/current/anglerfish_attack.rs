fn anglerfish_attack(
    game: &mut Game<'_>,
    target: u32,
    source: Damage,
) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::Who;
    use horizon_core::nbt::Tag;

    let _ = target;
    let Some(Who::Mob(attacker)) = source.attacker else {
        return std::ops::ControlFlow::Continue(());
    };
    let Some(index) = game.mobs.iter().position(|mob| mob.id == attacker) else {
        return std::ops::ControlFlow::Continue(());
    };
    if game.mobs[index].kind.id != "naturalist:anglerfish" || !game.mobs[index].alive() {
        return std::ops::ControlFlow::Continue(());
    }
    horizon_client::mobs::animal_brain::set_hunting_cooldown(
        &mut game.mobs[index],
        i64::from(naturalist::anglerfish_behavior::HUNTING_COOLDOWN),
    );
    let raw = game
        .persist
        .raw
        .entry(attacker)
        .or_insert_with(|| Tag::Compound(Default::default()));
    if let Tag::Compound(fields) = raw {
        fields.insert(
            "HuntingCooldown".into(),
            Tag::Int(naturalist::anglerfish_behavior::HUNTING_COOLDOWN),
        );
    }
    std::ops::ControlFlow::Continue(())
}
