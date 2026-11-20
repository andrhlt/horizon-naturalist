fn hippo_melon_particle_send(
    hub: &Hub,
    options: &ParticleOptions,
) -> std::ops::ControlFlow<Option<Particle>> {
    use horizon_protocol::item::Stack;
    use std::ops::ControlFlow;
    if options.ty != ParticleType::Item
        || options.source.as_deref() != Some("minecraft:melon_slice")
    {
        return ControlFlow::Continue(());
    }
    let particle = hub.registry_id("minecraft:particle_type", "minecraft:item");
    let slice = hub.registry_id("minecraft:item", "minecraft:melon_slice");
    ControlFlow::Break(particle.zip(slice).map(|(id, item)| Particle {
        id,
        data: Some(ParticleData::Item(Stack {
            item,
            count: 1,
            added: Vec::new(),
            removed: Vec::new(),
            wire: Default::default(),
        })),
    }))
}
