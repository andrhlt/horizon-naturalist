fn dragonfly_audio(game: &mut Game<'_>) {
    if !game.options.execution_role.is_client() {
        return;
    }
    let sources = game
        .mobs
        .iter()
        .filter(|mob| !mob.removed && mob.kind.id == "naturalist:dragonfly")
        .map(|mob| {
            (
                mob.id,
                [mob.position.x, mob.position.y, mob.position.z],
                mob.velocity.x.hypot(mob.velocity.z) as f32,
                mob.silent,
            )
        });
    horizon_client_render::sound_engine::naturalist_dragonfly_audio::update(
        &mut game.audio,
        sources,
    );
}
