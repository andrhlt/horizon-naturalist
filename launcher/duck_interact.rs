fn duck_interact(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::mobs::{nav::naturalist_duck_pet as pet, render::DYE_NAMES};
    use horizon_world::{attachments::AttachmentType, nbt::Tag};
    use horizon_client::text::Text;
    use std::ops::ControlFlow;

    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    if game.mobs[index].kind.id == "naturalist:ostrich" && game.mobs[index].alive() {
        let item = game.held().map(|stack| stack.item.as_ref());
        if !matches!(item, Some("minecraft:lead" | "minecraft:name_tag" | "minecraft:golden_dandelion")) {
            return ControlFlow::Break(Ok(super::naturalist_ostrich_game::interact(game, index)));
        }
    }
    let hedgehog = game.mobs[index].kind.id == "naturalist:hedgehog";
    let tortoise = game.mobs[index].kind.id == "naturalist:tortoise";
    let crab = game.mobs[index].kind.id == "naturalist:crab";
    let rat = game.mobs[index].kind.id == "naturalist:rat";
    let snake = game.mobs[index].kind.id == "naturalist:snake";
    let big_cat = horizon_client::mobs::goals::naturalist_cat_goals::is_big_cat(&game.mobs[index].kind.id);
    let big_cat_name = if game.mobs[index].kind.id == "naturalist:lion" { "Lion" } else { "Tiger" };
    let hippo = game.mobs[index].kind.id == "naturalist:hippo";
    let bird = game.mobs[index].kind.id == "naturalist:bird";
    let bear = horizon_client::mobs::naturalist_bears::is_bear(&game.mobs[index].kind.id);
    let lizard = game.mobs[index].kind.id == "naturalist:lizard";
    let capybara = game.mobs[index].kind.id == "naturalist:capybara";
    if (game.mobs[index].kind.id != "naturalist:duck" && !bird && !capybara && !lizard && !bear && !crab && !tortoise && !hippo && !hedgehog && !big_cat && !snake && !rat) || !game.mobs[index].alive() {
        return ControlFlow::Continue(());
    }
    let item = game.held().cloned();
    let id = item.as_ref().map_or("", |stack| stack.item.as_ref());
    if (crab || hedgehog) && matches!(id, "naturalist:capture_net" | "minecraft:lead" | "minecraft:name_tag") {
        return ControlFlow::Continue(());
    }
    const CONSUMED_MAIN_HAND: AttachmentType<([i32; 4], i64)> = AttachmentType {
        namespace: "naturalist",
        name: "consumed_main_hand",
        default: || ([0; 4], i64::MIN),
        codec: None,
        copy_on_death: false,
        sync: false,
    };
    let actor = game.acting_player_uuid();
    let mob = &game.mobs[index];
    let owner = mob.extra.tame && mob.extra.owner == Some(actor);
    let food = if bird {
        matches!(id, "minecraft:wheat_seeds" | "minecraft:pumpkin_seeds" | "minecraft:melon_seeds" | "minecraft:beetroot_seeds")
    } else if snake {
        horizon_client::mobs::goals::naturalist_snake_goals::TAME_FOOD.contains(&id)
    } else if lizard {
        id == "minecraft:spider_eye"
    } else {
        mob.kind.food.iter().any(|candidate| candidate == id)
    };
    let network_id = mob.id;
    let at = mob.center();
    let tick = game.world.game_time;
    let main = game.hand == Hand::Main;
    // Horizon can reissue a successful main-hand use as an empty off-hand
    // interaction, including on the following tick. An actual off-hand item
    // or an interaction without a successful main-hand use still reaches the pet.
    if !main && id.is_empty()
        && game.persist.uuids.get(&network_id)
            .and_then(|uuid| game.entity_attachments.get_mut(uuid))
            .and_then(|data| data.get_existing(&CONSUMED_MAIN_HAND).ok().flatten())
            .is_some_and(|(player, accepted)| player == actor && accepted <= tick && accepted >= tick - 1)
    {
        return ControlFlow::Break(Ok(false));
    }
    let mut consume = false;
    let mut sound = None;

    let secondary = game.keys.held(Action::Sneak) || game.keys.raw(&winit::keyboard::KeyCode::ShiftRight);
    let whistle_cooling = id == "naturalist:whistle" && game.cooldowns.is_on_cooldown(id);
    if owner && id == "naturalist:whistle" && !(rat && whistle_cooling) {
        if secondary || whistle_cooling {
            return ControlFlow::Continue(());
        }
        let mob = &mut game.mobs[index];
        let follow = !pet::is_following(&mob.ai.nav);
        pet::set_following(&mut mob.ai.nav, follow);
        mob.extra.sitting = false;
        if !follow {
            mob.ai.target = None;
        }
        game.add_cooldown(std::sync::Arc::from(id), 20);
        sound = Some(("naturalist:item.whistle", 0.8));
        let name = game.mobs[index].custom_name.as_deref().unwrap_or(if bird { "Bird" } else if capybara { "Capybara" } else if lizard { "Lizard" } else if bear { "Bear" } else if crab { "Crab" } else if tortoise { "Tortoise" } else if hippo { "Hippo" } else if hedgehog { "Hedgehog" } else if big_cat { big_cat_name } else if snake { "Snake" } else if rat { "Rat" } else { "Duck" }).to_owned();
        let mode = Text::translate(if follow { "naturalist.whistle.following" } else { "naturalist.whistle.wandering" }, &[]);
        game.send_overlay_message(Text::translate("naturalist.whistle.message", &[Text::literal(name), mode]));
    } else if !crab && !tortoise && !hippo && !big_cat && !snake && !rat && owner && id == "minecraft:shears" && mob.extra.color <= 15 {
        game.mobs[index].extra.color = u8::MAX;
        game.damage_held(1);
        sound = Some(("minecraft:entity.sheep.shear", 1.0));
    } else if let Some(dye) = id.strip_prefix("minecraft:").and_then(|id| id.strip_suffix("_dye"))
        .and_then(|color| DYE_NAMES.iter().position(|name| *name == color))
        .filter(|&dye| !crab && !tortoise && !hippo && !big_cat && !snake && !rat && owner && usize::from(mob.extra.color) != dye)
    {
        game.mobs[index].extra.color = dye as u8;
        consume = true;
        sound = Some(("minecraft:item.dye.use", 1.0));
    } else if !bird && !capybara && !lizard && !bear && !crab && !tortoise && !hippo && !hedgehog && !big_cat && !snake && !rat && id == "minecraft:bucket" && mob.is_baby() && (!mob.extra.tame || owner) {
        let mut filled = ItemStack::new("naturalist:duck_bucket", 1);
        let mut components = horizon_client::entity_nbt::bucket_components(mob);
        let variant = horizon_client_core::nbt_value::text(
            game.persist.raw.get(&network_id).and_then(|tag| tag.get("Variant"))
        ).unwrap_or_else(|| "naturalist:duck".to_owned());
        let variant = Tag::String(variant.as_str().into());
        let mut data = components.other("minecraft:bucket_entity_data").cloned()
            .unwrap_or_else(|| Tag::Compound(Default::default()));
        if let Tag::Compound(fields) = &mut data {
            fields.insert("Variant".into(), variant.clone());
            fields.insert("Age".into(), Tag::Int(mob.extra.age));
        }
        components.set_other("minecraft:bucket_entity_data", Some(data));
        components.set_other("minecraft:custom_data", Some(Tag::compound([("Variant", variant)])));
        filled.extra = Some(Box::new(components));
        let entity = game.mob_entity(&game.mobs[index]);
        if let Some(held) = &item {
            game.trigger_item_entity("minecraft:player_interacted_with_entity", held, &entity);
        }
        game.trigger_item("minecraft:filled_bucket", &filled);
        game.break_leash(network_id, true, false);
        game.mobs[index].removed = true;
        game.filled_result(filled);
        sound = Some(("minecraft:item.bucket.fill", 1.0));
    } else if big_cat && mob.extra.tame && mob.is_baby() && food {
        let age = i64::from(mob.extra.age);
        let seconds = ((-age / 20) as f32 * 0.1) as i32;
        horizon_client::mobs::ai::age_up(&mut game.mobs[index], seconds, true);
        consume = true;
    } else if !bird && !mob.extra.tame && (hedgehog || capybara || lizard || crab || tortoise || snake || rat || mob.is_baby()) && food {
        consume = true;
        let mob = &mut game.mobs[index];
        if tortoise { mob.persistent = true; }
        let tamed = if hippo {
            true
        } else if rat {
            mob.random.next_int_bound(10).unwrap_or(0) < 7
        } else if snake {
            mob.random.next_int_bound(2).unwrap_or(0) == 0
        } else if hedgehog {
            mob.random.next_boolean()
        } else {
            let roll = mob.random.next_int_bound(3).unwrap_or(0);
            if capybara { roll != 0 } else { roll == 0 }
        };
        if tamed && (capybara || hedgehog) && mob.extra.color > 15 {
            mob.extra.color = 14;
        }
        if tamed {
            mob.extra.tame = true;
            mob.extra.owner = Some(actor);
            if crab {
                pet::set_following(&mut mob.ai.nav, true);
                mob.ai.target = None;
            }
            if tortoise {
                horizon_client::mobs::goals::naturalist_tortoise_goals::tame_attributes(mob, true);
                mob.extra.sitting = true;
            }
            if rat {
                mob.extra.sitting = true;
                mob.nav_stop();
            }
            if snake {
                mob.extra.sitting = true;
                mob.nav_stop();
                horizon_client::mobs::goals::naturalist_snake_goals::stop_anger(mob);
            }
            if hippo || big_cat {
                mob.extra.sitting = true;
                mob.ai.target = None;
                mob.nav_stop();
            }
            if lizard {
                horizon_client::mobs::naturalist_lizard::tame_attributes(mob, true);
                mob.extra.sitting = true;
                mob.ai.target = None;
                mob.nav_stop();
            }
            let entity = game.mob_entity(&game.mobs[index]);
            game.trigger_entity("minecraft:tame_animal", "entity", &entity);
        }
        game.mob_event_particles(network_id, if tamed { 7 } else { 6 });
    } else if lizard && mob.extra.tame && food && mob.health < mob.attrs.max_health {
        use horizon_client::mobs::naturalist_lizard;
        let mob = &mut game.mobs[index];
        mob.heal(5.0);
        if mob.health >= mob.attrs.max_health && naturalist_lizard::missing_tail(&mob.ai.nav) {
            naturalist_lizard::set_tail(&mut mob.ai.nav, true);
            if !mob.silent {
                sound = Some(("minecraft:entity.slime.squish", 1.0));
            }
        }
        consume = true;
    } else if big_cat && mob.extra.tame && food && mob.health < mob.attrs.max_health {
        let nutrition = item.as_ref().and_then(|stack| {
            horizon_client::item_sync::food(stack, game.items().get(&stack.item).and_then(|info| info.food))
        }).map_or(4.0, |(nutrition, _)| nutrition as f32);
        game.mobs[index].heal(nutrition);
        consume = true;
    } else if rat && mob.extra.tame && food && mob.health < mob.attrs.max_health {
        game.mobs[index].heal(2.0);
        consume = true;
        sound = Some(("minecraft:entity.generic.eat", 1.0));
    } else if rat && owner && !food && !secondary
        && !matches!(id, "minecraft:lead" | "minecraft:name_tag" | "naturalist:capture_net" | "naturalist:rat_spawn_egg")
    {
        let mob = &mut game.mobs[index];
        mob.extra.sitting = !mob.extra.sitting;
        mob.ai.jumping = false;
        mob.nav_stop();
    } else if (hippo || snake) && mob.extra.tame && food && mob.health < mob.attrs.max_health {
        game.mobs[index].heal(4.0);
        consume = true;
    } else if hippo && food && mob.extra.age == 0 && mob.extra.in_love <= 0 {
        if let Some(stack) = item.clone() {
            game.naturalist_hippo_eat(index, stack, actor);
        }
        consume = true;
    } else if !snake && owner && food && mob.health < mob.attrs.max_health {
        game.mobs[index].heal(if bird { 1.0 } else if bear { 4.0 } else if tortoise { 3.0 } else { 2.0 });
        if bird && game.mobs[index].health == game.mobs[index].attrs.max_health {
            game.mob_event_particles(network_id, 7);
        }
        consume = true;
    } else if !rat && owner && (!hedgehog || !game.keys.held(Action::Sneak)) && (!(bear || hippo) || game.keys.held(Action::Sneak)
        || game.keys.raw(&winit::keyboard::KeyCode::ShiftRight))
        && (bird && mob.on_ground || !bird && id.is_empty() || crab && !food || (tortoise || big_cat || snake) && !food && !matches!(id, "minecraft:name_tag" | "minecraft:lead") || lizard && !matches!(id, "minecraft:name_tag" | "minecraft:lead")) {
        let mob = &mut game.mobs[index];
        mob.extra.sitting = !mob.extra.sitting;
        mob.ai.jump = false;
        if lizard || hippo || big_cat || snake {
            mob.ai.target = None;
        }
        mob.nav_stop();
    } else if big_cat && owner && food && !horizon_client::mobs::ai::takes_food(mob) {
        let mob = &mut game.mobs[index];
        mob.extra.sitting = !mob.extra.sitting;
        mob.ai.jump = false;
        mob.ai.target = None;
        mob.nav_stop();
    } else if tortoise && owner && food && (mob.is_baby() || !horizon_client::mobs::ai::takes_food(mob)) {
        let mob = &mut game.mobs[index];
        mob.extra.sitting = !mob.extra.sitting;
        return ControlFlow::Continue(());
    } else {
        if bird && (food || id == "minecraft:cookie") {
            return ControlFlow::Break(Ok(false));
        }
        return ControlFlow::Continue(());
    }

    if consume {
        if (hippo || big_cat) && food && let Some(stack) = &item {
            let target = game.mob_entity(&game.mobs[index]);
            game.trigger_item_entity("minecraft:player_interacted_with_entity", stack, &target);
        }
        game.consume_held();
    }
    if let Some((sound, volume)) = sound {
        game.audio.play(sound, Some(at), volume, 1.0);
    }
    let pos = game.mobs[index].position;
    if main && !game.mobs[index].removed {
        let uuid = game.uuid_for(network_id);
        let _ = game.entity_attachments.entry(uuid).or_default().set(&CONSUMED_MAIN_HAND, (actor, tick));
    }
    game.world.modified.insert((pos.x.floor() as i32 >> 4, pos.z.floor() as i32 >> 4));
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
