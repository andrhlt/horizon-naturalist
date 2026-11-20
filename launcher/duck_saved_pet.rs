fn duck_saved_pet(game: &Game<'_>, tag: &mut Tag, mob: &Mob) {
    if mob.kind.id == "naturalist:bass" {
        if let Tag::Compound(fields) = tag {
            use horizon_client::mobs::nav::naturalist_bass_state as bass;
            fields.insert("EatCooldown".into(), Tag::Int(bass::cooldown(&mob.ai.nav)));
        }
        return;
    }
    if mob.kind.id == "naturalist:rhino" {
        if let Tag::Compound(fields) = tag {
            let stun = horizon_client::mobs::naturalist_rhino::stun_ticks(&mob.ai.nav);
            fields.insert("StunTick".into(), Tag::Int(stun));
        }
        return;
    }
    if mob.kind.id == "naturalist:giraffe" {
        if let Tag::Compound(fields) = tag {
            if let Some(owner) = mob.extra.owner {
                fields.insert("Owner".into(), Tag::IntArray(owner.to_vec()));
            } else {
                fields.remove(&"Owner".into());
            }
        }
        return;
    }
    let ostrich = mob.kind.id == "naturalist:ostrich";
    let bear = horizon_client::mobs::naturalist_bears::is_bear(&mob.kind.id);
    let bird = mob.kind.id == "naturalist:bird";
    if !matches!(mob.kind.id.as_str(), "naturalist:duck" | "naturalist:capybara" | "naturalist:lizard" | "naturalist:crab" | "naturalist:hippo" | "naturalist:tortoise" | "naturalist:tiger" | "naturalist:lion" | "naturalist:snake" | "naturalist:rat" | "naturalist:hedgehog") && !bird && !bear && !ostrich {
        return;
    }
    let Tag::Compound(fields) = tag else { return };
    if let Some(owner) = mob.extra.owner {
        fields.insert("Owner".into(), Tag::IntArray(owner.to_vec()));
    } else {
        fields.remove(&"Owner".into());
    }
    if mob.kind.id == "naturalist:rat" {
        let status = horizon_client::mobs::goals::naturalist_rat_goals::state(mob);
        fields.insert("FromHand".into(), Tag::Byte(i8::from(status.from_hand)));
        if let Some([x,y,z]) = status.station {
            let packed = (i64::from(x) & 0x3ffffff) << 38
                | (i64::from(z) & 0x3ffffff) << 12 | i64::from(y) & 0xfff;
            fields.insert("Workstation".into(), Tag::Long(packed));
        } else {
            fields.remove(&"Workstation".into());
        }
        let cargo: Vec<_> = status.cargo.iter().flatten()
            .map(horizon_items::item_nbt::item_to_nbt).collect();
        fields.insert("CarriedItems".into(), Tag::List(cargo));
    }
    if ostrich {
        let state = horizon_client::mobs::naturalist_ostrich::state(mob);
        let eggs: Vec<_> = state.owned_eggs.iter().map(|&[x, y, z]| {
            ((i64::from(x) & 0x3ffffff) << 38)
                | ((i64::from(z) & 0x3ffffff) << 12) | (i64::from(y) & 0xfff)
        }).collect();
        fields.insert("OwnedEggs".into(), Tag::LongArray(eggs));
        fields.insert("Saddled".into(), Tag::Byte(i8::from(mob.extra.saddled)));
        fields.insert("HasEgg".into(), Tag::Byte(i8::from(state.has_egg)));
        fields.insert("anger_end_time".into(), Tag::Long(state.anger_end.unwrap_or(-1)));
        if let Some(angry) = state.anger_actor {
            fields.insert("angry_at".into(), Tag::IntArray(angry.to_vec()));
        } else {
            fields.remove(&"angry_at".into());
        }
        for key in ["EatingHaystack", "Temper", "Bred", "ChestedHorse", "Tame"] {
            fields.remove(&key.into());
        }
    }
    if bird {
        // Bird cannot be a baby; AgeableMob does not save these fields.
        for key in ["Age", "ForcedAge", "AgeLocked"] { fields.remove(&key.into()); }
        // The Parrot template writes its legacy integer over the raw name.
        let variant = horizon_client_core::nbt_value::text(
            game.persist.raw.get(&mob.id).and_then(|raw| raw.get("Variant"))
        ).unwrap_or_else(|| "naturalist:american_robin".to_owned());
        fields.insert("Variant".into(), Tag::String(variant.as_str().into()));
    }
    if mob.kind.id == "naturalist:lizard" {
        let has_tail = !horizon_client::mobs::naturalist_lizard::missing_tail(&mob.ai.nav);
        fields.insert("HasTail".into(), Tag::Byte(i8::from(has_tail)));
    }
    if bear {
        let pose = horizon_client::mobs::naturalist_bears::state(&mob.ai.nav);
        fields.insert("Sheared".into(), Tag::Byte(i8::from(pose.sheared)));
        fields.insert("HuntingCooldown".into(), Tag::Int(pose.hunting));
    }
    let snake = (mob.kind.id == "naturalist:snake")
        .then(|| horizon_client::mobs::goals::naturalist_snake_goals::state(&mob.ai.nav));
    if bear || snake.is_some() {
        let (end, target) = if let Some(state) = snake {
            (state.anger_end.unwrap_or(-1), state.anger_target)
        } else {
            (mob.extra.anger_end, (mob.extra.anger > 0).then_some(mob.extra.angry_at.or(mob.ai.target)).flatten())
        };
        fields.insert("anger_end_time".into(), Tag::Long(end));
        if let Some(target) = target {
            let uuid = match target {
                horizon_client::mobs::Who::Player => {
                    let guest = game.lan.as_ref()
                        .and_then(|host| host.mob_players.get(&mob.id).copied());
                    game.player_uuid_of(guest).or_else(|| game.save_who_uuid(target))
                }
                horizon_client::mobs::Who::Mob(_) => game.save_who_uuid(target),
            };
            if let Some(uuid) = uuid.or_else(|| snake.and_then(|state| state.anger_uuid)) {
                fields.insert("angry_at".into(), Tag::IntArray(uuid.to_vec()));
            }
        } else if snake.is_some() {
            fields.remove(&"angry_at".into());
        }
    }
    fields.insert("Sitting".into(), Tag::Byte(i8::from(mob.extra.sitting)));
    if mob.kind.id == "naturalist:crab" {
        fields.insert("FromHand".into(), Tag::Byte(i8::from(horizon_client::mobs::nav::naturalist_crab::state(&mob.ai.nav).from_hand)));
    }
    if mob.kind.id == "naturalist:lion" {
        fields.insert("Mane".into(), Tag::Byte(i8::from(mob.extra.flag)));
        if let Some(Tag::List(attributes)) = fields.get_mut(&"attributes".into()) {
            for attribute in attributes {
                if let Tag::Compound(row) = attribute
                    && let Some(Tag::List(modifiers)) = row.get_mut(&"modifiers".into())
                {
                    modifiers.retain(|modifier| {
                        horizon_client_core::nbt_value::text(modifier.get("id")).as_deref()
                            != Some(horizon_client::mobs::goals::naturalist_lion_goals::CUB_SPEED)
                    });
                    if modifiers.is_empty() { row.remove(&"modifiers".into()); }
                }
            }
        }
    }
    let color = if mob.extra.color <= 15 { i32::from(mob.extra.color) } else { -1 };
    if mob.kind.id == "naturalist:tortoise" {
        fields.insert("HasEgg".into(), Tag::Byte(i8::from(mob.extra.flag)));
    } else if !matches!(mob.kind.id.as_str(), "naturalist:hippo" | "naturalist:tiger" | "naturalist:lion" | "naturalist:snake" | "naturalist:rat") {
        fields.insert("DyeColor".into(), Tag::Int(color));
    }
    let following = horizon_client::mobs::nav::naturalist_duck_pet::is_following(&mob.ai.nav);
    fields.insert("FollowingOwner".into(), Tag::Byte(i8::from(following)));
}
