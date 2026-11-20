fn duck_loaded_pet(mob: &mut Mob, tag: &Tag) {
    if mob.kind.id == "naturalist:bass" {
        use crate::mobs::nav::naturalist_bass_state as bass;
        let variant = text(tag.get("Variant"));
        mob.extra.variant = variant.as_deref().map_or_else(
            || number(tag.get("Variant")).unwrap_or(0).clamp(0, 2) as u8, bass::tier);
        bass::set_cooldown(&mut mob.ai.nav, number(tag.get("EatCooldown")).unwrap_or(0) as i32);
        return;
    }
    if mob.kind.id == "naturalist:desert_scorpion" {
        let from_hand = flag(tag, "FromHand");
        crate::mobs::naturalist_scorpion::set_from_hand(&mut mob.ai.nav, from_hand);
        mob.persistent |= from_hand;
        return;
    }
    if mob.kind.id == "naturalist:rhino" {
        crate::mobs::naturalist_rhino::put_stun(&mut mob.ai.nav, number(tag.get("StunTick")).unwrap_or(0) as i32);
        return;
    }
    if mob.kind.id == "naturalist:giraffe" {
        mob.extra.owner = crate::save::uuid_from_tag(tag.get("Owner"));
        mob.extra.tame = mob.extra.owner.is_some();
        crate::mobs::tame::restore_sitting(mob, flag(tag, "Sitting"));
        return;
    }
    let bear = crate::mobs::naturalist_bears::is_bear(&mob.kind.id);
    let bird = mob.kind.id == "naturalist:bird";
    if !matches!(mob.kind.id.as_str(), "naturalist:duck" | "naturalist:capybara" | "naturalist:lizard" | "naturalist:crab" | "naturalist:hippo" | "naturalist:tortoise" | "naturalist:tiger" | "naturalist:lion" | "naturalist:snake" | "naturalist:rat" | "naturalist:hedgehog") && !bird && !bear {
        return;
    }
    if mob.kind.id == "naturalist:rat" {
        use crate::mobs::goals::naturalist_rat_goals as rat;
        let packed = number(tag.get("Workstation"));
        rat::set_station(mob, packed.map(|p| [(p >> 38) as i32, (p << 52 >> 52) as i32, (p << 26 >> 38) as i32]));
        let status = rat::state_mut(mob);
        status.from_hand = flag(tag, "FromHand");
        status.cargo = match tag.get("CarriedItems") {
            Some(Tag::List(stacks)) => stacks.iter().filter_map(horizon_items::item_nbt::item_from_nbt)
                .take(rat::SLOTS).map(Some).collect(),
            _ => Vec::new(),
        };
    }
    if mob.kind.id == "naturalist:snake" {
        use crate::mobs::goals::naturalist_snake_goals as snake;
        let state = snake::state_mut(&mut mob.ai.nav);
        state.anger_uuid = crate::save::uuid_from_tag(tag.get("angry_at"));
        state.anger_end = Some(number(tag.get("anger_end_time")).unwrap_or(-1));
        state.anger_target = state.anger_uuid.map(|_| crate::mobs::Who::Mob(0));
        state.anger_load_pending = true;
        mob.extra.anger_end = state.anger_end.unwrap_or(-1);
        mob.extra.variant = text(tag.get("Variant")).as_deref().map_or(0, snake::variant_index);
    }
    if bear {
        let pose = crate::mobs::naturalist_bears::state_mut(&mut mob.ai.nav);
        pose.sheared = flag(tag, "Sheared");
        pose.hunting = number(tag.get("HuntingCooldown")).unwrap_or(0) as i32;
        pose.loaded_anger_target = crate::save::uuid_from_tag(tag.get("angry_at"));
        mob.extra.anger_end = number(tag.get("anger_end_time")).unwrap_or(-1);
    }
    mob.extra.owner = crate::save::uuid_from_tag(tag.get("Owner"));
    mob.extra.tame = mob.extra.owner.is_some();
    if mob.kind.id == "naturalist:lion" { mob.extra.flag = flag(tag, "Mane"); }
    if mob.kind.id == "naturalist:tortoise" {
        use crate::mobs::goals::naturalist_tortoise_goals as tortoise;
        mob.extra.flag = flag(tag, "HasEgg");
        mob.extra.variant = text(tag.get("Variant")).as_deref().map_or_else(
            || number(tag.get("Variant")).unwrap_or(0).rem_euclid(3) as u8,
            tortoise::variant_index,
        );
    }
    if mob.kind.id == "naturalist:lizard" {
        use crate::mobs::naturalist_lizard;
        naturalist_lizard::set_tail(&mut mob.ai.nav, flag(tag, "HasTail"));
        if mob.extra.tame {
            naturalist_lizard::tame_attributes(mob, false);
        }
    }
    crate::mobs::tame::restore_sitting(mob, flag(tag, "Sitting"));
    if mob.kind.id != "naturalist:hedgehog" {
        mob.extra.color = number(tag.get("DyeColor"))
            .filter(|id| *id >= 0)
            .map_or(u8::MAX, |id| u8::try_from(id).ok().filter(|id| *id <= 15).unwrap_or(0));
    }
    let following = tag.get("FollowingOwner").is_none() || flag(tag, "FollowingOwner");
    crate::mobs::nav::naturalist_duck_pet::set_following(&mut mob.ai.nav, following);
    mob.persistent |= flag(tag, "FromBucket");
}
