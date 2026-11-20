fn jellyfish_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use horizon_core::geometry::Vec3;
    use std::ops::ControlFlow;
    let mut args = command.split_ascii_whitespace();
    if args.next() != Some("naturalist_fixture_jellyfish") {
        return ControlFlow::Continue(());
    }
    let id = game.mobs.iter().find(|mob| {
        game.tags_of(Target::Mob(mob.id)).iter().any(|tag| tag == "jellyfish_fixture")
    }).unwrap().id;
    let mob = game.mobs.iter_mut().find(|mob| mob.id == id).unwrap();
    let text = match args.next() {
        Some("reset") => {
            mob.velocity = Vec3::ZERO;
            mob.ai.mov.set_wanted(mob.position.x + 4.0, mob.position.y, mob.position.z, 1.0);
            "ready".to_owned()
        }
        Some("pulse") => {
            let cooldown = horizon_client_lib::mobs::nav::naturalist_jellyfish_state::pulse_cooldown(&mob.ai.nav);
            format!("{} wet={} wanted={} motion={:?} cooldown={cooldown}", (mob.velocity.x - 0.18).abs() < 1e-8,
                mob.fluids.in_water, mob.ai.mov.has_wanted(), mob.velocity)
        }
        _ => "invalid operation".to_owned(),
    };
    game.feedback(Text::literal(format!("Jellyfish fixture {text}")));
    ControlFlow::Break(true)
}
