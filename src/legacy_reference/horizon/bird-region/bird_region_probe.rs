fn bird_region_probe(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_core::random::{LegacyRandom, RandomSource, XoroshiroRandom};
    use horizon_world::nbt::Tag;
    use serde_json::{Value, json};
    use std::ops::ControlFlow;

    let Some((stream, seed)) = command.strip_prefix("naturalist_fixture_bird_region ")
        .and_then(|args| args.split_once(' '))
        .and_then(|(stream, seed)| Some((stream, seed.parse::<i64>().ok()?))) else {
        return ControlFlow::Continue(());
    };
    let Some(terrain) = game.end.locator() else {
        return ControlFlow::Break(false);
    };
    fn group(
        terrain: &horizon_client::vanillagen::VanillaTerrain,
        mut random: impl RandomSource + 'static,
    ) -> Option<Value> {
        let mut state = None;
        let mut members = Vec::new();
        for _ in 0..5 {
            let nbt = Tag::Compound(terrain.spawn_generated(
                "naturalist:bird",
                horizon_core::geometry::Vec3::new(8.5, 100.0, 8.5),
                0.0,
                "minecraft:taiga",
                &mut random,
                &mut state,
            )?);
            let Some(Tag::List(attrs)) = nbt.get("attributes") else { return None };
            let Some(Tag::List(modifiers)) = attrs.iter().find(|attr|
                horizon_client_core::nbt_value::text(attr.get("id")).as_deref() == Some("minecraft:follow_range"))?
                .get("modifiers") else { return None };
            let bonus = modifiers.iter().find(|modifier|
                horizon_client_core::nbt_value::text(modifier.get("id")).as_deref() == Some("minecraft:random_spawn_bonus"))?
                .get("amount")?;
            let Tag::Double(bonus) = bonus else { return None };
            let Some(Tag::Int(age)) = nbt.get("Age") else { return None };
            let Some(Tag::Byte(left)) = nbt.get("LeftHanded") else { return None };
            members.push(json!({
                "variant": horizon_client_core::nbt_value::text(nbt.get("Variant"))?,
                "age": age,
                "follow_bonus": f64::from_bits(*bonus),
                "left_handed": left != &0
            }));
        }
        Some(json!({ "members": members, "next": random.next_long() }))
    }

    use horizon_client::mobs::naturalist_spawn_variants as variants;
    let files = variants::generation_files(game.pack_files());
    let mut result = None;
    variants::with_generation_files(files, || {
        result = match stream {
            "legacy" => group(&terrain, LegacyRandom::new(seed)),
            "xoroshiro" => group(&terrain, XoroshiroRandom::new(seed)),
            _ => None,
        };
    });
    game.feedback(Text::literal(format!("Naturalist Bird region {}", json!(result))));
    ControlFlow::Break(true)
}
