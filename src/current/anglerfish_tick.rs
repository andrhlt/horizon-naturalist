fn anglerfish_tick(game: &mut Game<'_>, entity: u32, kind: &str) {
    if kind == "naturalist:anglerfish" {
        crate::game::naturalist_anglerfish_goals::install(game, entity);
    }
    if !matches!(kind, "naturalist:anglerfish" | "naturalist:great_white_shark") {
        return;
    }
    if let Some(horizon_core::nbt::Tag::Compound(fields)) = game.persist.raw.get_mut(&entity) {
        if let Some(horizon_core::nbt::Tag::Int(ticks)) = fields.get_mut(&"HuntingCooldown".into())
        {
            *ticks = if kind == "naturalist:great_white_shark" {
                naturalist::shark_behavior::next_hunting_cooldown(*ticks)
            } else {
                naturalist::anglerfish_behavior::next_hunting_cooldown(*ticks)
            };
        }
    }
}
