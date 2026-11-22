fn bass_movement(game: &mut Game<'_>, entity: u32, kind: &str) {
    use horizon_client::mobs::control::{LookKind, MoveKind};
    use mod_api::Host;
    if kind != "naturalist:bass" {
        return;
    }
    // Normalize legacy NBT before the entity's gameplay and render snapshots.
    if let Some(horizon_core::nbt::Tag::Compound(fields)) = game.persist.raw.get_mut(&entity) {
        if let Some(value) = fields.get(&"Variant".into()) {
            if naturalist::bass_hunt::legacy_numeric_index(value).is_some() {
                if let Some(name) = naturalist::bass_hunt::variant(Some(value)) {
                    fields.insert(
                        "Variant".into(),
                        horizon_core::nbt::Tag::String(name.as_str().into()),
                    );
                }
            }
        }
    }
    let tier = game
        .entity(entity)
        .and_then(|entity| entity.nbt(game))
        .map_or(0, |data| naturalist::bass_hunt::tier(&data));
    let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) else {
        return;
    };
    mob.extra.variant = tier;
    if !mob.ai.goals.has("NaturalistBassSchool") {
        let goal = crate::game::naturalist_bass_school::BassSchool::new(mob);
        mob.ai.goals.add(5, goal);
    }
    if !mob.ai.goals.has("NaturalistBassAvoidCatfish") {
        mob.ai
            .goals
            .add(1, crate::game::naturalist_bass_avoid::BassAvoid::catfish());
    }
    if !mob.ai.goals.has("NaturalistBassAvoidLarger") {
        mob.ai.goals.add(
            1,
            crate::game::naturalist_bass_avoid::BassAvoid::larger_bass(),
        );
    }
    if !matches!(mob.ai.mov.kind, MoveKind::SmoothSwimming { .. }) {
        mob.ai.mov.kind = MoveKind::SmoothSwimming {
            max_turn_x: 1000.0,
            max_turn_y: 5.0,
            in_water: 0.02,
            outside: 0.1,
            gravity: false,
        };
    }
    if let MoveKind::SmoothSwimming { max_turn_y, .. } = &mut mob.ai.mov.kind {
        *max_turn_y = if tier == 2 { 10.0 } else { 5.0 };
    }
    mob.ai.look.kind = LookKind::SmoothSwimming(5.0);
}
