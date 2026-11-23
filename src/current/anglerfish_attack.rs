fn anglerfish_attack(
    game: &mut Game<'_>,
    target: u32,
    amount: f32,
    source: horizon_client::mobs::Damage,
    original: impl FnOnce(&mut Game<'_>, u32, f32, horizon_client::mobs::Damage) -> bool,
) -> bool {
    use horizon_client::mobs::Who;
    use horizon_core::nbt::Tag;

    let Some(Who::Mob(attacker)) = source.attacker else {
        return original(game, target, amount, source);
    };
    let Some(index) = game.mobs.iter().position(|mob| {
        mob.id == attacker
            && mob.alive()
            && matches!(
                mob.kind.id.as_str(),
                "naturalist:anglerfish" | "naturalist:ray"
            )
    }) else {
        return original(game, target, amount, source);
    };
    let kind = game.mobs[index].kind.id.clone();
    let hit = original(game, target, amount, source);
    if !hit {
        return false;
    }

    match kind.as_str() {
        "naturalist:anglerfish" => {
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
        }
        "naturalist:ray" => {
            if let Some(mob) = game
                .mobs
                .iter_mut()
                .find(|mob| mob.id == target && mob.alive())
            {
                mob.add_effect("minecraft:poison", 100, 0);
            }
        }
        _ => {}
    }
    true
}
