fn shark_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_client::mobs::nav::{
        naturalist_catfish_state as hunt, naturalist_shark_state as shark,
    };
    use std::ops::ControlFlow;

    let Some(query) = command.strip_prefix("naturalist_fixture_shark ") else {
        return ControlFlow::Continue(());
    };
    let values = if query == "spawn" {
        use horizon_client::mobs::{
            Species,
            spawn_selection::{self, naturalist_aquatic_spawn_limits as limits},
        };
        let cluster = limits::natural_pass(|| {
            limits::select(shark::KIND);
            spawn_selection::max_cluster_size(Species::Tadpole)
        });
        serde_json::json!({"wet":game.shark_fixture_spawn([8.5, 92.0, 10.5]),
            "dry":game.shark_fixture_spawn([8.5, 92.0, 6.5]), "cluster":cluster})
    } else {
        let selected = game.mobs.iter().filter(|mob| mob.kind.id == shark::KIND && mob.alive())
            .map(|mob| {
                let [w, h, eye] = [mob.dims().0, mob.dims().1, mob.dims().2].map(|size| (size * 10000.0).round() as i32);
                let parts = game.shark_part_boxes(mob);
                serde_json::json!({
                    "id": mob.id, "dims": [w, h, eye], "health": mob.health,
                    "air": mob.air, "wet": mob.fluids.in_water, "pushable": mob.pushable(),
                    "cooldown": hunt::cooldown(&mob.ai.nav), "speed": mob.movement_speed(),
                    "part_sizes": parts.map(|part| [(part.size().x * 100.0).round() as i32, (part.size().y * 100.0).round() as i32]),
                    "part_offsets_z": parts.map(|part| ((part.center().z - mob.position.z) * 100.0).round() as i32),
                    "position": mob.position.components(), "target": format!("{:?}", mob.ai.target),
                    "food": mob.kind.food.iter().map(String::as_str).collect::<Vec<_>>(),
                })
            }).collect::<Vec<_>>();
        serde_json::json!({"mobs":selected})
    };
    game.feedback(Text::literal(format!("Shark fixture {values}")));
    ControlFlow::Break(true)
}
