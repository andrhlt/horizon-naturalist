use mod_api::{EntityInteractEvent, HostSide, InteractionResult};

/// Server-authoritative Capybara pet interactions. Keep ownership and pet
/// settings in entity NBT so the ordinary save path carries them across reopen.
pub(crate) fn interact(
    side: HostSide<'_>,
    event: &mut EntityInteractEvent<'_>,
) -> Option<InteractionResult> {
    use mod_api::{Host, RandomSource, ServerHost, Tag, prelude::ServicesServerHost};

    if event.kind != "naturalist:capybara" {
        return None;
    }
    let HostSide::Server(host) = side else {
        return None;
    };
    let player = host.players().acting_player()?;
    let entity = host.entity(u32::try_from(event.entity).ok()?)?;
    let data = entity.nbt(host)?;
    let Some(living) = host.living(entity) else {
        return None;
    };
    let owner = uuid_tag(player.uuid());
    let item = event
        .stack
        .map(|stack| stack.item().id(host))
        .unwrap_or_default();
    let food = item == "minecraft:melon_slice";
    let empty_hand = event.stack.is_none() || item == "minecraft:air";
    let tamed = matches!(data.get(&"Tame".into()), Some(Tag::Byte(value)) if *value != 0);

    let consume = |host: &mut dyn ServerHost| -> bool {
        if player.is_creative(host).unwrap_or(false) {
            return true;
        }
        let slot = match event.hand {
            mod_api::Hand::Main => player.selected_slot(host).unwrap_or(0) as u32,
            mod_api::Hand::Off => 40,
        };
        let replacement = event.stack.and_then(|stack| {
            if stack.count() <= 1 {
                None
            } else {
                let mut one_less = stack.clone();
                one_less.set_count(stack.count() - 1).ok()?;
                Some(one_less)
            }
        });
        player.set_inventory_slot(host, slot, replacement).is_ok()
    };

    if !tamed && food {
        if !consume(host) {
            return None;
        }
        let success = entity
            .level(host)
            .and_then(|level| {
                level.with_random(host, |random| random.next_int_bound(3).unwrap_or(1) == 0)
            })
            .unwrap_or(false);
        if success {
            let mut fields = mod_api::Compound::default();
            fields.insert("Tame".into(), Tag::Byte(1));
            fields.insert("Owner".into(), owner);
            fields.insert("DyeColor".into(), Tag::Int(14));
            fields.insert("FollowingOwner".into(), Tag::Byte(1));
            fields.insert("Sitting".into(), Tag::Byte(0));
            let _ = entity.merge_nbt(host, fields);
        }
        return Some(InteractionResult::SuccessServer);
    }

    if !tamed {
        return None;
    }

    if food {
        let health = living.health(host)?;
        let max_health = living.max_health(host)?;
        let age = match data.get(&"Age".into()) {
            Some(Tag::Int(value)) => *value,
            _ => 0,
        };
        if health >= max_health && age < 0 {
            return None;
        }
        if !consume(host) {
            return None;
        }
        if health < max_health {
            let _ = living.set_health(host, (health + 2.0).min(max_health));
        }
        if age >= 0 {
            let mut fields = mod_api::Compound::default();
            fields.insert("InLove".into(), Tag::Int(600));
            fields.insert("LoveCause".into(), owner);
            let _ = entity.merge_nbt(host, fields);
        }
        return Some(InteractionResult::SuccessServer);
    }

    if item == "naturalist:whistle"
        && !player.is_secondary_use_active(host).unwrap_or(false)
        && !player
            .has_item_cooldown(host, "naturalist:whistle")
            .unwrap_or(false)
    {
        let following = matches!(
            data.get(&"FollowingOwner".into()),
            Some(Tag::Byte(value)) if *value != 0
        );
        let _ = entity.merge_nbt(
            host,
            [
                ("FollowingOwner".into(), Tag::Byte(i8::from(!following))),
                ("Sitting".into(), Tag::Byte(0)),
            ]
            .into(),
        );
        let _ = player.set_item_cooldown(host, "naturalist:whistle", 20);
        return Some(InteractionResult::SuccessServer);
    }

    if empty_hand {
        // The same physical interaction can arrive through more than one
        // input path. Match the client helper's short sit debounce so a
        // duplicate server event cannot toggle the order straight back.
        const SIT_ACTION: &str = "naturalist:capybara_sit";
        if player.has_item_cooldown(host, SIT_ACTION).unwrap_or(false) {
            return Some(InteractionResult::SuccessServer);
        }
        let sitting = matches!(data.get(&"Sitting".into()), Some(Tag::Byte(value)) if *value != 0);
        let _ = entity.merge_nbt(
            host,
            [("Sitting".into(), Tag::Byte(i8::from(!sitting)))].into(),
        );
        let _ = player.set_item_cooldown(host, SIT_ACTION, 5);
        return Some(InteractionResult::SuccessServer);
    }

    None
}

fn uuid_tag(uuid: u128) -> mod_api::Tag {
    mod_api::Tag::IntArray(vec![
        (uuid >> 96) as u32 as i32,
        (uuid >> 64) as u32 as i32,
        (uuid >> 32) as u32 as i32,
        uuid as u32 as i32,
    ])
}
