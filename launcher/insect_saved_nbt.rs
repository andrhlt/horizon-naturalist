fn insect_saved_nbt(game: &Game<'_>, tag: &mut Tag, mob: &Mob) {
    use horizon_client_core::nbt_value::text;
    let Tag::Compound(fields) = tag else { return };
    let kind = mob.kind.id.as_str();
    if mob.kind.id == "naturalist:mole" {
        use horizon_client::mobs::goals::naturalist_mole_goals;
        fields.insert("MoleState".into(), Tag::Int(naturalist_mole_goals::phase(mob) as i32));
    }
    if matches!(kind, "naturalist:dragonfly" | "naturalist:lizard_tail" | "naturalist:vulture") {
        fields.remove(&"InLove".into());
        fields.remove(&"LoveCause".into());
    }
    let default_variant = match kind {
        "naturalist:ant" => {
            use horizon_client::mobs::{Who, naturalist_ant};
            let ant = naturalist_ant::state(mob);
            fields.insert("HillCooldown".into(), Tag::Int(ant.cooldown));
            if mob.extra.angry_at == Some(Who::Player)
                && let Some(uuid) = ant.anger_actor
            {
                fields.insert("angry_at".into(), Tag::IntArray(uuid.to_vec()));
            }
            "naturalist:ant"
        },
        "naturalist:hedgehog" => {
            for key in ["CollarColor", "variant", "sound_variant", "anger_end_time", "angry_at"] {
                fields.remove(&key.into());
            }
            if let Some(enchantments) = horizon_client::mobs::naturalist_hedgehog::save_enchantments(mob) {
                fields.insert("ThrowEnchantments".into(), enchantments);
            }
            "naturalist:brown"
        },
        "naturalist:snail" => {
            let snail = horizon_client::mobs::naturalist_snail::snail(mob);
            fields.insert("Color".into(), Tag::Byte(snail.color as i8));
            fields.insert("HasEgg".into(), Tag::Byte(i8::from(snail.has_egg)));
            if snail.surface.attached {
                let normal = snail.surface.normal;
                fields.insert("ClimbNormalX".into(), Tag::Float((normal.x as f32).to_bits()));
                fields.insert("ClimbNormalY".into(), Tag::Float((normal.y as f32).to_bits()));
                fields.insert("ClimbNormalZ".into(), Tag::Float((normal.z as f32).to_bits()));
            }
            "naturalist:snail"
        },
        "naturalist:butterfly" => "naturalist:monarch",
        "naturalist:caterpillar" => "naturalist:caterpillar",
        "naturalist:firefly" => "naturalist:firefly",
        "naturalist:dragonfly" => "naturalist:blue",
        _ => return,
    };
    let saved = game.persist.raw.get(&mob.id);
    let variant = text(saved.and_then(|raw| raw.get("Variant"))).unwrap_or_else(|| default_variant.to_owned());
    fields.insert("Variant".into(), Tag::String(variant.as_str().into()));
    if matches!(kind, "naturalist:ant" | "naturalist:hedgehog" | "naturalist:butterfly" | "naturalist:caterpillar" | "naturalist:snail") {
        let from_hand = saved.and_then(|raw| raw.get("FromHand")).cloned().unwrap_or(Tag::Byte(0));
        fields.insert("FromHand".into(), from_hand);
    }
    if matches!(kind, "naturalist:butterfly" | "naturalist:firefly") {
        fields.insert("Age".into(), Tag::Int(mob.extra.age));
    }
}
