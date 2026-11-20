fn knapsack_release(game: &mut Game<'_>, stack: &ItemStack, pos: [i32; 3], face: [i32; 3], claimed: &mut bool) {
    use super::{commands::Target, naturalist_knapsack as sack, persist::PassengerSlot};
    use horizon_client::entity_nbt;
    use horizon_world::nbt::Tag;

    if *claimed || stack.item.as_ref() != sack::ITEM || !sack::filled(stack) {
        return;
    }
    let pos: [i32; 3] = std::array::from_fn(|i| pos[i] + face[i]);
    let at = [f64::from(pos[0]) + 0.5, f64::from(pos[1]), f64::from(pos[2]) + 0.5];
    let position = Tag::List(at.into_iter().map(|p| Tag::Double(p.to_bits())).collect());
    let mut fields = stack.custom_data().cloned().unwrap_or_default();
    fields.insert("Pos".into(), position.clone());
    // Java loads the passenger tree but only adds the root to the level.
    let passengers = fields.remove(&"Passengers".into());
    let root = Tag::Compound(fields);
    if !entity_nbt::uuid(&root).is_some_and(|uuid| game.target_with_uuid(uuid).is_some())
        && let Some(Target::Mob(id)) = game.spawn_entity_from_nbt(root)
        && let Some(Tag::List(passengers)) = passengers
    {
        fn place_tree(tag: &mut Tag, position: &Tag) {
            if let Tag::Compound(fields) = tag {
                fields.insert("Pos".into(), position.clone());
                if let Some(Tag::List(children)) = fields.get_mut(&"Passengers".into()) {
                    for child in children {
                        place_tree(child, position);
                    }
                }
            }
        }
        let mut slots = Vec::new();
        for mut passenger in passengers {
            if let Some(kind) = entity_nbt::kind(&passenger)
                && (game.res.mob_types().contains_key(&kind) || game.entity_type_exists(&kind))
            {
                place_tree(&mut passenger, &position);
                slots.push(PassengerSlot::Saved(passenger));
            }
        }
        if !slots.is_empty() {
            game.persist.passenger_slots.insert(id, slots);
        }
    }
    let sound = pos.map(|coordinate| f64::from(coordinate) + 0.5);
    game.native_play_sound_in_place("naturalist:item.knapsack.place", sound, 6, 0.6, 1.0);
    sack::replace_one(game, stack, ItemStack::new(sack::ITEM, 1));
    *claimed = true;
}
