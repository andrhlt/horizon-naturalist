fn large_cat_death_post(game: &mut crate::game::Game<'_>, _: u32, _: &str, attacker: u32) {
    use horizon_core::nbt::Tag;
    let Some(mob) = game.mobs.iter_mut().find(|mob| {
        mob.id == attacker && matches!(mob.kind.id.as_ref(), "naturalist:lion" | "naturalist:tiger")
    }) else {
        return;
    };

    // Lion.killedEntity / Tiger.killedEntity pause the next hunt for 120 seconds.
    horizon_client::mobs::animal_brain::set_hunting_cooldown(mob, 2400);
    mob.ai.target = None;
    let raw = game
        .persist
        .raw
        .entry(attacker)
        .or_insert_with(|| Tag::Compound(Default::default()));
    if let Tag::Compound(fields) = raw {
        fields.insert("HuntingCooldown".into(), Tag::Int(2400));
    }
}
