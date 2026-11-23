fn anglerfish_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    use horizon_client_lib::mobs::Who;
    let Some(tag) = command.strip_prefix("naturalist_fixture_anglerfish ") else {
        return ControlFlow::Continue(());
    };
    let text = if let Some(ticks) = tag.strip_prefix("seed ").and_then(|value| value.parse::<i32>().ok()) {
        let id = game.mobs.iter().find(|mob| mob.kind.id == "naturalist:anglerfish" && mob.alive()).map(|mob| mob.id);
        if let Some(id) = id {
            if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) {
                horizon_client::mobs::animal_brain::set_hunting_cooldown(mob, i64::from(ticks));
            }
            let raw = game.persist.raw.entry(id).or_insert_with(|| horizon_core::nbt::Tag::Compound(Default::default()));
            if let horizon_core::nbt::Tag::Compound(fields) = raw {
                fields.insert("HuntingCooldown".into(), horizon_core::nbt::Tag::Int(ticks));
            }
            format!("seeded cooldown {ticks}")
        } else {
            "no live Anglerfish".to_owned()
        }
    } else if tag == "inspect" {
        let mobs = game.mobs.iter().filter(|mob| mob.kind.id == "naturalist:anglerfish")
            .map(|mob| {
                let raw = game.persist.raw.get(&mob.id);
                let variant = raw.and_then(|tag| tag.get("Variant")).and_then(|tag| match tag {
                    horizon_core::nbt::Tag::String(value) => Some(value.clone()),
                    _ => None,
                });
                let cooldown = raw.and_then(|tag| tag.get("HuntingCooldown")).and_then(|tag| match tag {
                    horizon_core::nbt::Tag::Int(value) => Some(*value),
                    _ => None,
                });
                serde_json::json!({
                    "id": mob.id,
                    "alive": mob.alive(),
                    "health": mob.health,
                    "position": mob.position.components(),
                    "variant": variant,
                    "hunting_cooldown": cooldown,
                    "brain_cooldown": horizon_client::mobs::animal_brain::hunting_cooldown(mob),
                })
            }).collect::<Vec<_>>();
        let held = game.held().map(|stack| {
            let component = |name: &str| stack.data()
                .and_then(|data| data.other(name))
                .and_then(|data| data.get("Variant"))
                .and_then(|tag| match tag {
                    horizon_core::nbt::Tag::String(value) => Some(value.clone()),
                    _ => None,
                });
            serde_json::json!({"item": stack.item.as_ref(),
                "bucket_variant": component("minecraft:bucket_entity_data"),
                "custom_variant": component("minecraft:custom_data")})
        });
        serde_json::json!({"mobs": mobs, "held": held}).to_string()
    } else if tag == "deep" {
        game.naturalist_fixture_anglerfish_deep()
    } else if tag == "tables" {
        game.naturalist_fixture_aquatic_tables()
    } else if tag == "limits" {
        game.naturalist_fixture_aquatic_limits()
    } else if tag == "bass_termination" {
        game.naturalist_fixture_bass_termination()
    } else if tag == "placement" {
        let placement = |pos| super::natural_spawning::anglerfish_spawn::placement(game, pos);
        format!("{} {} {} {}", placement([8, 27, 8]), placement([20, 27, 8]), placement([8, 40, 8]), placement([8, 54, 8]))
    } else {
        let mob = game.mobs.iter().find(|mob| {
            game.tags_of(Target::Mob(mob.id)).iter().any(|candidate| candidate == tag)
        }).unwrap();
        match mob.ai.target {
            Some(Who::Mob(id)) => game.tags_of(Target::Mob(id)).into_iter().next().unwrap_or_default(),
            _ => "no target".to_owned(),
        }
    };
    game.feedback(Text::literal(format!("Anglerfish fixture {text}")));
    ControlFlow::Break(true)
}
