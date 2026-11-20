fn bird_prepare_seeds(game: &mut Game<'_>) {
    use horizon_client::mobs::nav::naturalist_bird_navigation::{Seed, foraging, section_order};
    use horizon_core::geometry::Vec3;

    if !game.mobs.iter().any(|mob| mob.kind.id == "naturalist:bird" && !mob.extra.tame) {
        return;
    }
    let food = game.items().item_tags.get("naturalist:bird_food_items").cloned();
    let edible = |item: &str| {
        food.as_ref().map_or_else(
            || matches!(item, "minecraft:wheat_seeds" | "minecraft:pumpkin_seeds" | "minecraft:melon_seeds" | "minecraft:beetroot_seeds"),
            |tag| tag.contains(item),
        )
    };
    for mob in &mut game.mobs {
        if mob.kind.id != "naturalist:bird" || mob.extra.tame || !mob.alive() || mob.removed {
            continue;
        }
        let bounds = mob.bounds().inflate(Vec3::new(8.0, 4.0, 8.0));
        let target = foraging(&mut mob.ai.nav).as_ref().and_then(|state| state.target);
        let mut first = None;
        let mut selected = None;
        for drop in game.dropped.iter().filter(|drop| drop.stack.count > 0 && edible(&drop.stack.item)) {
            let seed = Seed { id: drop.id, position: drop.position };
            if bounds.intersects(drop.bounds()) {
                let section = [seed.position.x, seed.position.y, seed.position.z]
                    .map(|p| (p.floor() as i32) >> 4);
                let key = section_order(section, drop.join.get());
                if first.is_none_or(|(old, _)| key < old) {
                    first = Some((key, seed));
                }
            }
            if Some(seed.id) == target {
                selected = Some(seed);
            }
        }
        let state = foraging(&mut mob.ai.nav).get_or_insert_with(Default::default);
        state.first = first.map(|(_, seed)| seed);
        state.selected = selected;
    }
}
