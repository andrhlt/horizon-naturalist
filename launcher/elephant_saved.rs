fn elephant_saved(game: &Game<'_>, tag: &mut Tag, mob: &Mob) {
    use horizon_client::mobs::nav::naturalist_elephants as elephant;
    if !elephant::includes(&mob.kind.id) {
        return;
    }
    let Tag::Compound(fields) = tag else { return };
    if let Some(owner) = mob.extra.owner {
        fields.insert("Owner".into(), Tag::IntArray(owner.to_vec()));
    } else {
        fields.remove(&"Owner".into());
    }
    let status = elephant::state(mob);
    let end = status.anger_end.or_else(|| {
        status.legacy_anger_ticks.map(|ticks| game.world.game_time.saturating_add(ticks))
    }).unwrap_or(-1);
    fields.insert("anger_end_time".into(), Tag::Long(end));
    let actor = status.anger_uuid.or_else(|| {
        status.anger_target.and_then(|target| game.save_who_uuid(target))
    });
    if let Some(uuid) = actor {
        fields.insert("angry_at".into(), Tag::IntArray(uuid.to_vec()));
    } else {
        fields.remove(&"angry_at".into());
    }

    let mut items = mob.extra.items.clone();
    items.resize(elephant::INVENTORY_SLOTS, None);
    items[elephant::SADDLE_SLOT] = mob.saddle_item();
    fields.insert("Items".into(), horizon_client::save::encode_items(&items));
    fields.insert("Saddled".into(), Tag::Byte(i8::from(mob.extra.saddled)));
    fields.insert("Chested".into(), Tag::Byte(i8::from(mob.extra.chested)));
    fields.insert(
        "TamingFood".into(),
        Tag::Int(elephant::state(mob).cakes.min(i32::MAX as u32) as i32),
    );
}
