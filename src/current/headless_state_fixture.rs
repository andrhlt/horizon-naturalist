fn headless_state_fixture(
    game: &mut Game<'_>,
    command: &str,
) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;

    let Some(query) = command.strip_prefix("naturalist_fixture_mobs") else {
        return ControlFlow::Continue(());
    };
    if !game.options.headless {
        return ControlFlow::Continue(());
    }

    let query = query.trim();
    if query == "attack anglerfish cod" {
        use horizon_client::mobs::{Damage, DamageKind, Who};
        let Some(attacker) = game.mobs.iter().find(|mob| mob.kind.id == "naturalist:anglerfish" && mob.alive()).map(|mob| mob.id) else {
            game.feedback(Text::literal("Mob fixture no live Anglerfish attacker"));
            return ControlFlow::Break(true);
        };
        let Some(target) = game.mobs.iter().find(|mob| mob.kind.id == "minecraft:cod" && mob.alive()).map(|mob| mob.id) else {
            game.feedback(Text::literal("Mob fixture no live Cod target"));
            return ControlFlow::Break(true);
        };
        let landed = game.mob_attacks_mob(target, 1.0, Damage {
            kind: DamageKind::MobAttack,
            attacker: Some(Who::Mob(attacker)),
            from: None,
            attacker_species: None,
        });
        game.feedback(Text::literal(format!("Mob fixture attack landed={landed}")));
        return ControlFlow::Break(true);
    }
    if let Some(seed) = query.strip_prefix("seed anglerfish HuntingCooldown ") {
        let Ok(ticks) = seed.parse::<i32>() else {
            game.feedback(Text::literal("Mob fixture invalid cooldown seed"));
            return ControlFlow::Break(true);
        };
        let Some(id) = game.mobs.iter().find(|mob| mob.kind.id == "naturalist:anglerfish" && mob.alive()).map(|mob| mob.id) else {
            game.feedback(Text::literal("Mob fixture no live Anglerfish"));
            return ControlFlow::Break(true);
        };
        let raw = game.persist.raw.entry(id).or_insert_with(|| horizon_core::nbt::Tag::Compound(Default::default()));
        if let horizon_core::nbt::Tag::Compound(fields) = raw {
            fields.insert("HuntingCooldown".into(), horizon_core::nbt::Tag::Int(ticks));
        }
        game.feedback(Text::literal(format!("Mob fixture seeded HuntingCooldown={ticks}")));
        return ControlFlow::Break(true);
    }

    let mobs = game.mobs.iter().map(|mob| {
        let tags = game.tags_of(Target::Mob(mob.id));
        let raw = game.persist.raw.get(&mob.id);
        let string_field = |name: &str| raw.and_then(|tag| tag.get(name)).and_then(|tag| match tag {
            horizon_core::nbt::Tag::String(value) => Some(value.to_string_lossy()),
            _ => None,
        });
        let int_field = |name: &str| raw.and_then(|tag| tag.get(name)).and_then(|tag| match tag {
            horizon_core::nbt::Tag::Int(value) => Some(*value),
            _ => None,
        });
        serde_json::json!({
            "id": mob.id,
            "kind": mob.kind.id.as_str(),
            "alive": mob.alive(),
            "tags": tags,
            "health": mob.health,
            "baby": mob.is_baby(),
            "age": mob.extra.age,
            "air": mob.air,
            "position": mob.position.components(),
            "variant": string_field("Variant"),
            "gray": raw.and_then(|tag| tag.get("Gray")).map(|tag| format!("{tag:?}")),
            "conversion_time": int_field("ConversionTime"),
            "hunting_cooldown": int_field("HuntingCooldown"),
            "from_bucket": raw.and_then(|tag| tag.get("FromBucket")).is_some_and(|tag| {
                matches!(tag, horizon_core::nbt::Tag::Byte(value) if *value != 0)
            }),
            "raw": raw.map(|tag| format!("{tag:?}")),
            "effects": mob.extra.effects.iter().map(|effect| (effect.id, effect.duration, effect.amplifier)).collect::<Vec<_>>(),
            "target": format!("{:?}", mob.ai.target),
        })
    }).collect::<Vec<_>>();

    let held = game.held().map(|stack| {
        let component = |name: &str| stack.data()
            .and_then(|data| data.other(name))
            .and_then(|data| data.get("Variant"))
            .and_then(|tag| match tag {
                horizon_core::nbt::Tag::String(value) => Some(value.to_string_lossy()),
                _ => None,
            });
        serde_json::json!({
            "item": stack.item.to_string(),
            "bucket_variant": component("minecraft:bucket_entity_data"),
            "custom_variant": component("minecraft:custom_data"),
        })
    });
    let payload = serde_json::json!({"mobs": mobs, "held": held, "creative": game.creative});
    game.feedback(Text::literal(format!("Mob fixture {payload}")));
    ControlFlow::Break(true)
}
