fn net_saved(game: &Game<'_>, mob: &Mob, tag: &mut Tag) {
    let kind = mob.kind.id.as_str();
    if !matches!(kind, "naturalist:ant" | "naturalist:butterfly" | "naturalist:caterpillar" | "naturalist:desert_scorpion" | "naturalist:snail" | "naturalist:dragonfly" | "naturalist:firefly") { return }
    let Tag::Compound(fields) = tag else { return };
    // Vanilla Parrot writes a numeric Variant; Naturalist owns a namespaced string instead.
    if let Some(horizon_core::nbt::Tag::String(variant)) = game.persist.raw.get(&mob.id).and_then(|raw| raw.get("Variant")) {
        fields.insert("Variant".into(), horizon_core::nbt::Tag::String(variant.clone()));
    }
    if matches!(kind, "naturalist:butterfly" | "naturalist:caterpillar") {
        fields.insert("Age".into(), horizon_core::nbt::Tag::Int(mob.extra.age));
    }
}
