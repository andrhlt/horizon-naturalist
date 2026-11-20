fn butterfly_live_spawn(game: &Game<'_>, pending: &Vec<Mob>) {
    if !game.options.headless || std::env::var_os("NATURALIST_FIXTURE_TRACE_SPAWNS").is_none() {
        return;
    }
    let rows: Vec<_> = pending
        .iter()
        .filter(|mob| mob.kind.id == "naturalist:butterfly")
        .filter_map(|mob| {
            let (palette, pick) = mob
                .ai
                .nav
                .__horizon_mixins_naturalist_butterfly_spawn_pool()
                .as_ref()?;
            Some(serde_json::json!({
                "position": [mob.position.x, mob.position.y, mob.position.z],
                "pair": palette,
                "selected": palette[*pick],
                "adult": !mob.is_baby(),
            }))
        })
        .collect();
    if !rows.is_empty() {
        println!(
            "naturalist_butterfly_live {}",
            serde_json::json!({
                "tick": game.world.game_time,
            "day_time": game.world.day_time,
                "rows": rows,
            })
        );
    }
}
