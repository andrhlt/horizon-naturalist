fn vulture_drop_equipment(mut result: (Vec<Drop>, bool), mob: &mut Mob) -> (Vec<Drop>, bool) {
    if mob.kind.id == "naturalist:vulture" {
        for slot in SLOTS {
            if let Some(stack) = slot_item(mob, slot) {
                result.0.push(slot_drop(mob, slot, stack));
                clear_slot(mob, slot);
            }
        }
    }
    result
}
