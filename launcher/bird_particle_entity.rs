fn bird_particle_entity(entity: &str, event: u8) -> std::ops::ControlFlow<Option<(&'static str, Burst)>> {
    if entity == "naturalist:bird" {
        return std::ops::ControlFlow::Break(entity_event_burst("minecraft:parrot", event));
    }
    std::ops::ControlFlow::Continue(())
}
