fn clam_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::{ai, nav::naturalist_clam as clam};
    use std::ops::ControlFlow;

    let Some(query) = command.strip_prefix("naturalist_fixture_clam ") else {
        return ControlFlow::Continue(());
    };
    let text = if query == "spawn" {
        game.naturalist_fixture_clam_spawn()
    } else {
        let mob = game.mobs.iter_mut().find(|mob| mob.kind.id == clam::KIND && mob.alive() && !mob.no_ai).unwrap();
        let shell = clam::state_mut(&mut mob.ai.nav);
        if query == "reset-snap" {
            shell.snap_cooldown = 0;
        }
        serde_json::json!({
            "open": shell.open, "treasure": shell.has_treasure, "timer": shell.state_timer,
            "cooldown": shell.snap_cooldown, "delay": shell.launch_delay, "pushable": mob.pushable(),
            "stored_persistent": ai::requires_custom_persistence(mob),
        }).to_string()
    };
    game.feedback(Text::literal(format!("Clam fixture {text}")));
    ControlFlow::Break(true)
}
