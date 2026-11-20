fn starfish_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    let Some(query) = command.strip_prefix("naturalist_fixture_starfish ") else {
        return ControlFlow::Continue(());
    };
    let text = if query == "spawn" {
        game.naturalist_fixture_starfish_spawn()
    } else {
        let fish = game.mobs.iter().find(|mob| mob.kind.id == "naturalist:starfish" && mob.alive()).unwrap();
        serde_json::json!({"pushable": fish.pushable(),
            "portal": horizon_client_lib::mobs::mob_can_use_portal(fish)}).to_string()
    };
    game.feedback(Text::literal(format!("Starfish fixture {text}")));
    ControlFlow::Break(true)
}
