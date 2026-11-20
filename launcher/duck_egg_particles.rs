fn duck_egg_particles(game: &mut Game<'_>, at: [f64; 3], item: &str) -> std::ops::ControlFlow<()> {
    use std::ops::ControlFlow;

    if item != "naturalist:duck_egg" {
        return ControlFlow::Continue(());
    }

    let options = game.item_particle(&horizon_client::items::ItemStack::new(item, 1));
    for _ in 0..8 {
        let random = game.particles.random();
        let velocity = std::array::from_fn(|_| (f64::from(random.next_float()) - 0.5) * 0.08);
        game.particles.add(&options, at, velocity);
    }
    ControlFlow::Break(())
}
