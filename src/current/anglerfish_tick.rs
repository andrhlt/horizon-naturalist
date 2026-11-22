fn anglerfish_tick(game: &mut Game<'_>, entity: u32, kind: &str) {
    if kind != "naturalist:anglerfish" {
        return;
    }
    if let Some(horizon_core::nbt::Tag::Compound(fields)) = game.persist.raw.get_mut(&entity) {
        if let Some(horizon_core::nbt::Tag::Int(ticks)) = fields.get_mut(&"HuntingCooldown".into())
        {
            *ticks = naturalist::anglerfish_behavior::next_hunting_cooldown(*ticks);
        }
    }
}
