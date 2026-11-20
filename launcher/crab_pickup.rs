fn crab_pickup(game: &mut Game<'_>) {
    use horizon_client::mobs::nav::naturalist_crab as crab;

    if game.dropped.is_empty() || !game.ui.rules.bool("mob_griefing") {
        return;
    }
    for index in 0..game.mobs.len() {
        let mob = &game.mobs[index];
        if mob.kind.id != crab::KIND || !mob.alive() || mob.removed || !mob.can_pick_up_loot()
            || crab::holding_weapon(mob) || !game.creatures.ticked.contains(&mob.id)
        {
            continue;
        }
        let reach = mob.bounds().inflate(Vec3::new(1.0, 0.0, 1.0));
        let items = game.items();
        let Some(drop) = game.dropped.iter().position(|item| item.pickup_delay <= 0
            && reach.intersects(item.bounds()) && crab::WEAPON_TAGS.iter().any(|tag|
                items.item_tags.get(*tag).is_some_and(|entries| entries.contains(item.stack.item.as_ref()))))
        else {
            continue;
        };
        let before = game.dropped[drop].clone();
        let id = mob.id;
        let Some(taken) = game.equip_item_if_possible(index, &before.stack) else {
            continue;
        };
        game.add_pickup(Taken::Item(Box::new(before.clone())), Collector::Mob(id));
        game.mob_item_pickup_trigger(index, &before);
        game.dropped[drop].stack.count -= taken;
        if game.dropped[drop].stack.count == 0 {
            game.remove_dropped(drop);
        }
    }
}
