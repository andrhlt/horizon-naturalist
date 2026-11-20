fn bear_shear(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::mobs::naturalist_bears as bear;
    use std::ops::ControlFlow;
    let Some(tool) = game.held().filter(|stack| stack.item.as_ref() == "minecraft:shears").cloned() else {
        return ControlFlow::Continue(());
    };
    let Some(index) = game.mob_interaction_target(target) else { return ControlFlow::Continue(()) };
    let actor = game.acting_player_uuid();
    let mob = &mut game.mobs[index];
    if !bear::is_bear(&mob.kind.id) || !bear::can_shear(mob)
        || (mob.extra.owner == Some(actor) && mob.extra.color <= 15)
    {
        return ControlFlow::Continue(());
    }
    let pose = bear::state_mut(&mut mob.ai.nav);
    if !pose.sleeping {
        mob.ai.last_hurt_by = Some(Who::Player);
        mob.ai.last_hurt_by_time = mob.age as i32;
    }
    pose.sheared = true;
    let drops = (0..(1 + mob.random.next_int_bound(2).unwrap_or(0))).map(|_| {
        let x = f64::from((mob.random.next_float() - mob.random.next_float()) * 0.1);
        let y = f64::from(mob.random.next_float() * 0.05);
        let z = f64::from((mob.random.next_float() - mob.random.next_float()) * 0.1);
        [x, y, z]
    }).collect::<Vec<_>>();
    let [x, y, z] = mob.position.components();
    let creature = game.mob_entity(&game.mobs[index]);
    game.trigger_item_entity("minecraft:player_interacted_with_entity", &tool, &creature);
    for velocity in drops {
        game.spawn_item(ItemStack::new("naturalist:fur", 1), [x.floor(), y.floor() + 1.0, z.floor()], velocity, 0);
    }
    game.damage_held(1);
    game.audio.play("minecraft:entity.sheep.shear", Some([x, y, z]), 1.0, 1.0);
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
