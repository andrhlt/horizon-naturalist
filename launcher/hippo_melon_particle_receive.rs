fn hippo_melon_particle_receive(
    game: &Game<'_>,
    particle: &Particle,
) -> std::ops::ControlFlow<Option<ParticleOptions>> {
    use horizon_client::items::ItemStack;
    use std::ops::ControlFlow;
    if let Some(ParticleData::Item(stack)) = &particle.data
        && game
            .remote
            .as_ref()
            .and_then(|remote| remote.builtin.get("minecraft:item"))
            .and_then(|registry| registry.name(stack.item))
            == Some("minecraft:melon_slice")
    {
        return ControlFlow::Break(Some(
            game.item_particle(&ItemStack::new("minecraft:melon_slice", 1)),
        ));
    }
    ControlFlow::Continue(())
}
