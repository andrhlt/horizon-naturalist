fn tortoise_egg_mined(
    _result: (),
    game: &mut Game<'_>,
    pos: [i32; 3],
    state: BlockStateId,
    canceled: &bool,
) {
    if !*canceled && game.pack.assets.registry.state(state).is_some_and(|row| row.name == "naturalist:ostrich_egg") {
        super::naturalist_ostrich_game::egg_destroyed(game, pos);
        return;
    }
    if *canceled || game.creative
        || !game.held().is_some_and(|stack| stack.item.as_ref() == "minecraft:command_block")
        || !game.pack.assets.registry.state(state).is_some_and(|row| row.name == "naturalist:tortoise_egg")
    {
        return;
    }
    game.level_events.push(horizon_world::level::LevelEvent::BlockEvent {
        pos: BlockPos::new(pos[0], pos[1], pos[2]),
        state,
        kind: "naturalist:tortoise_mined_hatch",
    });
}
