fn elephant_gear_receive(mob: &mut Mob, values: &[(u8, i32, Value)]) {
    use horizon_client::mobs::nav::naturalist_elephants as elephants;
    if !elephants::includes(&mob.kind.id) {
        return;
    }
    for (field, _, value) in values {
        match (*field, value) {
            (elephants::GEAR_META, Value::Byte(flags)) => {
                mob.extra.saddled = flags & 1 != 0;
                mob.extra.chested = flags & 2 != 0;
                mob.extra.tame = flags & 4 != 0;
            }
            (elephants::BANNER_META, Value::String(text)) => {
                let tag = horizon_client::command::snbt::parse_fully(text).ok();
                let mut items = horizon_client::save::decode_items(tag.as_ref(), 1);
                mob.extra.items.resize(elephants::INVENTORY_SLOTS, None);
                mob.extra.items[elephants::BANNER_SLOT] = items.pop().flatten();
            }
            _ => {}
        }
    }
}
