fn net_saved(game: &Game<'_>, mob: &Mob, tag: &mut Tag) {
    let kind = mob.kind.id.as_str();
    if kind == "naturalist:duck" {
        if let Tag::Compound(fields) = &mut *tag {
            if let Some(owner) = mob.extra.owner.filter(|_| mob.extra.tame) { fields.insert("Owner".into(), Tag::IntArray(owner.to_vec())); } else { fields.remove(&"Owner".into()); }
            fields.insert("FromBucket".into(), Tag::Byte(i8::from(mob.extra.from_bucket)));
            fields.insert("Sitting".into(), Tag::Byte(i8::from(mob.extra.sitting)));
        }
        return;
    }
    if !matches!(kind, "naturalist:ant" | "naturalist:crab" | "naturalist:butterfly" | "naturalist:caterpillar" | "naturalist:desert_scorpion" | "naturalist:snail" | "naturalist:dragonfly" | "naturalist:firefly") { return }
    let Tag::Compound(fields) = tag else { return };
    // Vanilla Parrot writes a numeric Variant; Naturalist owns a namespaced string instead.
    let variant = game.persist.raw.get(&mob.id).and_then(|raw| raw.get("Variant"));
    if kind == "naturalist:crab" {
        let name = match variant {
            Some(horizon_core::nbt::Tag::String(value)) => horizon_core::identifier::Identifier::parse(&value.to_string_lossy()).ok().map(|id| id.to_string()),
            Some(value) => naturalist::bass_hunt::legacy_numeric_index(value).map(|index| {
                let name = ["blue", "brown", "orange", "red", "yellow"][index.rem_euclid(5) as usize];
                format!("naturalist:{name}")
            }),
            None => None,
        }.unwrap_or_else(|| "naturalist:blue".to_owned());
        fields.insert("Variant".into(), horizon_core::nbt::Tag::String(name.as_str().into()));
    } else if let Some(horizon_core::nbt::Tag::String(variant)) = variant {
        fields.insert("Variant".into(), horizon_core::nbt::Tag::String(variant.clone()));
    }
    if matches!(kind, "naturalist:butterfly" | "naturalist:caterpillar" | "naturalist:crab") {
        fields.insert("Age".into(), horizon_core::nbt::Tag::Int(mob.extra.age));
    }
}
