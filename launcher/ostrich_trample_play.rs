fn ostrich_trample_play(
    game: &mut Game<'_>,
    event: i32,
    pos: [i32; 3],
    data: i32,
) -> std::ops::ControlFlow<()> {
    use std::ops::ControlFlow;
    if event != horizon_client::mobs::naturalist_ostrich::EGG_TRAMPLE_SOUND {
        return ControlFlow::Continue(());
    }
    game.native_play_sound_in_place(
        "naturalist:entity.ostrich.egg_break",
        pos.map(|axis| f64::from(axis) + 0.5),
        4,
        0.7,
        f32::from_bits(data as u32),
    );
    ControlFlow::Break(())
}
