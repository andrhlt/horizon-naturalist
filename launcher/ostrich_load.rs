fn ostrich_load(mob: &mut Mob, tag: &Tag) {
    use horizon_client_core::nbt_value::number;
    use crate::entity_nbt::flag;
    use crate::mobs::naturalist_ostrich as ostrich;
    if mob.kind.id != ostrich::KIND {
        return;
    }
    mob.extra.owner = crate::save::uuid_from_tag(tag.get("Owner"));
    mob.extra.tame = mob.extra.owner.is_some();
    mob.extra.saddled = flag(tag, "Saddled");
    mob.extra.color = number(tag.get("DyeColor"))
        .filter(|color| (0..=15).contains(color)).map_or(u8::MAX, |color| color as u8);
    crate::mobs::tame::restore_sitting(mob, flag(tag, "Sitting"));
    let following = tag.get("FollowingOwner").is_none() || flag(tag, "FollowingOwner");
    crate::mobs::nav::naturalist_duck_pet::set_following(&mut mob.ai.nav, following);
    let pose = ostrich::state_mut(mob);
    pose.has_egg = flag(tag, "HasEgg");
    pose.anger_actor = crate::save::uuid_from_tag(tag.get("angry_at"));
    pose.anger_end = number(tag.get("anger_end_time")).filter(|end| *end > 0);
    if let Some(Tag::LongArray(eggs)) = tag.get("OwnedEggs") {
        pose.owned_eggs = eggs.iter().take(16).map(|&pos| {
            [(pos >> 38) as i32, (pos << 52 >> 52) as i32, (pos << 26 >> 38) as i32]
        }).collect();
    }
}
