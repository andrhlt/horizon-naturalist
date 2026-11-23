pub(crate) fn entity_interact(
    side: mod_api::HostSide<'_>,
    event: &mut mod_api::EntityInteractEvent<'_>,
) -> Option<mod_api::InteractionResult> {
    use mod_api::{Host, InteractionResult, ServerHost, Tag, prelude::ServicesServerHost};
    use mod_api::RandomSource;

    if event.kind != "naturalist:lizard" && event.kind != "naturalist:tortoise" {
        return None;
    }
    let mod_api::HostSide::Server(host) = side else {
        return None;
    };
    let stack = event.stack;
    let item = stack.map(|stack| stack.item().id(host)).unwrap_or_default();
    let player = host.players().acting_player()?;
    let entity_id = u32::try_from(event.entity).ok()?;
    let entity = host.entity(entity_id)?;
    let data = entity.nbt(host)?;
    let owner = uuid_tag(player.uuid());
    let living = host.living(entity)?;
    let tamed = data.get(&"Tame".into()).is_some_and(|tag| matches!(tag, Tag::Byte(value) if *value != 0));
    let owner_matches = data.get(&"Owner".into()) == Some(&owner);
    let owned = tamed && owner_matches;
    let (lizard, food) = if event.kind == "naturalist:lizard" {
        (true, item == "minecraft:spider_eye")
    } else {
        (false, item == "minecraft:cactus" || item == "minecraft:bamboo"
            || item == "minecraft:dandelion" || item == "minecraft:brown_mushroom")
    };
    if event.kind == "naturalist:tortoise" && owned && item == "naturalist:whistle" {
        let following = data.get(&"FollowingOwner".into()).is_none_or(|tag| !matches!(tag, Tag::Byte(0)));
        let _ = entity.merge_nbt(host, [
            ("FollowingOwner".into(), Tag::Byte(i8::from(!following))),
            ("Sitting".into(), Tag::Byte(0)),
        ].into());
        return Some(InteractionResult::SuccessServer);
    }
    if !food {
        if event.kind == "naturalist:tortoise" && owned && stack.is_none() {
            let sitting = data.get(&"Sitting".into()).is_some_and(|tag| matches!(tag, Tag::Byte(value) if *value != 0));
            let _ = entity.merge_nbt(host, [("Sitting".into(), Tag::Byte(i8::from(!sitting)))].into());
            return Some(InteractionResult::SuccessServer);
        }
        return None;
    }
    if lizard && tamed && !owned {
        return None;
    }
    let creative = player.is_creative(host).unwrap_or(false);
    let consume = |host: &mut dyn ServerHost| -> bool {
        if creative { return true; }
        let slot = match event.hand {
            mod_api::Hand::Main => player.selected_slot(host).unwrap_or(0) as u32,
            mod_api::Hand::Off => 40,
        };
        let replacement = if let Some(stack) = stack.filter(|stack| stack.count() > 1) {
            let mut one_less = stack.clone();
            if one_less.set_count(stack.count() - 1).is_err() { return false; }
            Some(one_less)
        } else {
            None
        };
        player.set_inventory_slot(host, slot, replacement).is_ok()
    };

    if lizard && !tamed {
        if !consume(host) { return None; }
        let tame = entity.level(host)
            .and_then(|level| level.with_random(host, |random| random.next_int_bound(3).unwrap_or(1) == 0))
            .unwrap_or(false);
        if tame {
            let mut patch = mod_api::Compound::default();
            patch.insert("Tame".into(), Tag::Byte(1));
            patch.insert("Owner".into(), owner);
            patch.insert("Sitting".into(), Tag::Byte(1));
            patch.insert("HasTail".into(), Tag::Byte(1));
            let _ = entity.merge_nbt(host, patch);
            let _ = living.set_attribute_base(host, "minecraft:max_health", 20.0);
            let _ = living.set_attribute_base(host, "minecraft:attack_damage", 4.0);
            let _ = living.set_health(host, 20.0);
        }
        return Some(InteractionResult::SuccessServer);
    }

    if !lizard && !tamed {
        if !consume(host) { return None; }
        let tame = entity.level(host)
            .and_then(|level| level.with_random(host, |random| random.next_int_bound(3).unwrap_or(1) == 0))
            .unwrap_or(false);
        if tame {
            let mut patch = mod_api::Compound::default();
            patch.insert("Tame".into(), Tag::Byte(1));
            patch.insert("Owner".into(), owner);
            patch.insert("Sitting".into(), Tag::Byte(1));
            patch.insert("FollowingOwner".into(), Tag::Byte(1));
            let _ = entity.merge_nbt(host, patch);
            let _ = living.set_attribute_base(host, "minecraft:max_health", 30.0);
            let _ = living.set_attribute_base(host, "minecraft:attack_damage", 4.0);
            let _ = living.set_health(host, 30.0);
        }
        return Some(InteractionResult::SuccessServer);
    }

    let health = living.health(host)?;
    let max_health = living.max_health(host)?;
    if health >= max_health {
        return None;
    }
    if !consume(host) { return None; }
    let amount = if lizard { 5.0 } else if event.kind == "naturalist:tortoise" { 3.0 } else { 2.0 };
    // Interaction events consume the held stack in the same server tick.  Use the
    // authoritative health setter here so the updated value is committed with that
    // interaction instead of being lost behind the entity's deferred heal path.
    let _ = living.set_health(host, (health + amount).min(max_health) as f32);
    if lizard
        && living.health(host).is_some_and(|after| after >= max_health)
        && data.get(&"HasTail".into()).is_some_and(|tag| matches!(tag, Tag::Byte(0)))
    {
        let _ = entity.merge_nbt(host, [("HasTail".into(), Tag::Byte(1))].into());
    }
    Some(InteractionResult::SuccessServer)
}

fn uuid_tag(uuid: u128) -> mod_api::Tag {
    mod_api::Tag::IntArray(vec![
        (uuid >> 96) as u32 as i32,
        (uuid >> 64) as u32 as i32,
        (uuid >> 32) as u32 as i32,
        uuid as u32 as i32,
    ])
}
