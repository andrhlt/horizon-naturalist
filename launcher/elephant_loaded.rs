fn elephant_loaded(mob: &mut Mob, tag: &Tag) {
    use crate::mobs::nav::naturalist_elephants as elephant;
    if !elephant::includes(&mob.kind.id) {
        return;
    }
    let absolute_end = number(tag.get("anger_end_time"));
    let legacy_ticks = absolute_end.is_none().then(|| number(tag.get("AngerTime"))).flatten();
    let uuid = crate::save::uuid_from_tag(tag.get("angry_at"));
    let anger_end = absolute_end.unwrap_or(-1);
    let status = elephant::state_mut(mob);
    status.anger_uuid = uuid;
    status.anger_end = absolute_end;
    status.legacy_anger_ticks = legacy_ticks;
    status.restore_anger = anger_end > 0 || uuid.is_some() || legacy_ticks.is_some();

    mob.extra.owner = crate::save::uuid_from_tag(tag.get("Owner"));
    mob.extra.tame = mob.extra.owner.is_some();
    mob.extra.chested = flag(tag, "Chested");
    mob.extra.items = crate::save::decode_items(tag.get("Items"), elephant::INVENTORY_SLOTS);
    let saddle = mob
        .extra
        .items
        .get(elephant::SADDLE_SLOT)
        .cloned()
        .flatten();
    mob.set_saddle(saddle);
    elephant::state_mut(mob).cakes = number(tag.get("TamingFood"))
        .and_then(|count| u32::try_from(count).ok())
        .unwrap_or(0);
}
