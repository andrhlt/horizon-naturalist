fn insect_release(game: &mut Game<'_>, stack: &ItemStack, claimed: &mut bool) {
    use horizon_world::nbt::Tag;

    if *claimed {
        return;
    }
    if stack.item.as_ref() == "naturalist:capture_net" {
        if let Some(player) = game.player.as_ref() {
            let at = [player.position.x, player.position.y, player.position.z];
            game.audio.play("minecraft:entity.player.attack.sweep", Some(at), 1.0, 1.0);
        }
        super::naturalist_capture_net_swing::particles(game);
        game.swing_arm();
        *claimed = true;
        return;
    }
    if stack.item.as_ref() == "naturalist:hedgehog" && game.pick().is_none() {
        *claimed = super::naturalist_hedgehog_throw::throw(game, stack);
        return;
    }
    let (kind, sound) = match stack.item.as_ref() {
        "naturalist:hedgehog" => ("naturalist:hedgehog", "naturalist:entity.hedgehog.ambient"),
        "naturalist:ant" => ("naturalist:ant", "naturalist:entity.ant.ambient"),
        "naturalist:snail" => ("naturalist:snail", "naturalist:entity.snail.forward"),
        "naturalist:butterfly" => ("naturalist:butterfly", "naturalist:entity.bird.fly"),
        "naturalist:caterpillar" => ("naturalist:caterpillar", "naturalist:entity.snail.forward"),
        "naturalist:scorpion" => ("naturalist:desert_scorpion", "naturalist:entity.scorpion.ambient"),
        "naturalist:dragonfly" => ("naturalist:dragonfly", "minecraft:item.bundle.drop_contents"),
        "naturalist:bee" => ("minecraft:bee", "minecraft:item.bundle.drop_contents"),
        "naturalist:silverfish" => ("minecraft:silverfish", "minecraft:item.bundle.drop_contents"),
        "naturalist:endermite" => ("minecraft:endermite", "minecraft:item.bundle.drop_contents"),
        "naturalist:firefly" => ("naturalist:firefly", "minecraft:item.bundle.drop_contents"),
        "naturalist:rat" => ("naturalist:rat", "naturalist:entity.rat.ambient"),
        "naturalist:crab" => ("naturalist:crab", "naturalist:entity.crab.ambient"),
        _ => return,
    };
    let Some(hit) = game.pick() else { return };
    let pos = hit.pos;
    let mut fields = stack.custom_data().cloned().unwrap_or_default();
    if kind == "naturalist:hedgehog" {
        fields = super::naturalist_hedgehog_throw::release_fields(stack, fields);
    }
    if matches!(kind, "naturalist:ant" | "naturalist:crab" | "naturalist:rat") && !fields.contains_key(&"id".into()) {
        fields.insert("id".into(), Tag::String(kind.into()));
        fields.insert("FromHand".into(), Tag::Byte(1));
        if kind == "naturalist:rat" && fields.get(&"Tame".into()) != Some(&Tag::Byte(1)) {
            fields.remove(&"Owner".into());
        }
        if let Some(weapon) = fields.remove(&"HeldItem".into()) {
            fields.insert("equipment".into(), Tag::compound([("mainhand", weapon)]));
        }
        if let Some(name) = stack.data().and_then(|data| data.other("minecraft:custom_name")).cloned() {
            fields.insert("CustomName".into(), name);
        }
    }
    let at = if fields.contains_key(&"id".into()) {
        let height = game.block_collision_shape(pos).bounds().map_or(0.0, |shape| shape.max_position().y);
        let at = [f64::from(pos[0]) + 0.5, f64::from(pos[1]) + height, f64::from(pos[2]) + 0.5];
        fields.insert("Pos".into(), Tag::List(at.into_iter().map(|p| Tag::Double(p.to_bits())).collect()));
        fields.insert("PersistenceRequired".into(), Tag::Byte(1));
        let before = game.mobs.len();
        let _ = game.spawn_entity_from_nbt(Tag::Compound(fields));
        if kind == "naturalist:rat" && let Some(mob) = game.mobs.get(before) {
            super::naturalist_rat_work::assign(game, mob.id, pos);
        }
        at
    } else {
        let Some(id) = game.spawn_mob_at(kind, pos.into(), true) else { return };
        let mut bucket = stack.clone();
        let variant = horizon_client_core::nbt_value::text(fields.get(&"Variant".into()));
        let snail_color = horizon_client_core::nbt_value::number(fields.get(&"Color".into()))
            .filter(|color| (0..=15).contains(color)).unwrap_or(12) as u8;
        bucket.data_mut().set_other("minecraft:bucket_entity_data", Some(Tag::Compound(fields)));
        let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) else { return };
        horizon_client::entity_nbt::apply_item_components(mob, &bucket);
        if kind == "naturalist:snail" {
            horizon_client::mobs::naturalist_snail::snail_mut(mob).color = snail_color;
        }
        let at = [mob.position.x, mob.position.y, mob.position.z];
        if let Some(variant) = variant {
            super::naturalist_deer_variants::put_variant(game, id, &variant);
        }
        if matches!(kind, "naturalist:butterfly" | "naturalist:caterpillar" | "naturalist:desert_scorpion" | "naturalist:snail") {
            let raw = game.persist.raw.entry(id).or_insert_with(|| Tag::Compound(Default::default()));
            if let Tag::Compound(fields) = raw {
                fields.insert("FromHand".into(), Tag::Byte(1));
            }
        }
        at
    };
    let who = game.who_context(Some(horizon_client::mobs::Who::Player), None);
    game.post_game_event("minecraft:entity_place", at, who);
    game.audio.play(sound, Some(at), 1.0, 1.0);
    game.award_stat("used", &stack.item, 1);
    game.consume_held();
    *claimed = true;
}
