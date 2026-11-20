fn alligator_egg_crack(game: &mut Game<'_>, at: [i32; 3]) -> std::ops::ControlFlow<Result<()>> {
    use horizon_client::mobs::goals::naturalist_insect_goals::state_property;
    use horizon_world::level::{LevelEvent, flags::UPDATE_CLIENTS};
    use std::ops::ControlFlow;
    let [x, y, z] = at;
    let state = game.world.block(x, y, z);
    let registry = game.pack.assets.registry;
    let Some(row) = registry.state(state) else {
        return ControlFlow::Continue(());
    };
    let sound = match row.name.as_str() {
        "naturalist:alligator_egg" => "naturalist:entity.alligator.egg_break",
        "naturalist:tortoise_egg" => "naturalist:entity.tortoise.egg_break",
        "naturalist:ostrich_egg" => "naturalist:entity.ostrich.egg_break",
        _ => return ControlFlow::Continue(()),
    };
    let eggs = if row.name == "naturalist:ostrich_egg" { 1 } else { row
        .properties
        .get("eggs")
        .and_then(|v| v.parse::<u8>().ok())
        .unwrap_or(1) };
    let pos = BlockPos::new(x, y, z);
    let pitch = 0.9 + game.world.random.next_float() * 0.2;
    game.level_events.push(LevelEvent::Sound {
        pos,
        sound: sound.into(),
        volume: 0.7,
        pitch,
    });
    game.with_level(|b, level, ticks| {
        if eggs <= 1 {
            b.destroy_block(level, ticks, pos, false);
        } else if let Some(next) = state_property(registry, state, "eggs", &(eggs - 1).to_string())
        {
            b.set_block(level, ticks, pos, next, UPDATE_CLIENTS);
        }
    });
    if eggs > 1 {
        game.post_block_game_event("minecraft:block_destroy", at, None, Some(state));
        game.level_event_at(2001, at, state.0 as i32);
    }
    ControlFlow::Break(game.process_level_events())
}
