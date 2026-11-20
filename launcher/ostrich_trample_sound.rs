fn ostrich_trample_sound(
    world: &mut MotionWorld<'_, '_, '_>,
    event: &FallEvent,
) -> std::ops::ControlFlow<()> {
    use crate::mobs::naturalist_ostrich as ostrich;
    use std::ops::ControlFlow;
    let FallEvent::Sound {
        sound: SceneSound::Block(LandingSound::TurtleEggBreak),
        position: Some(position),
        pitch,
        ..
    } = *event else {
        return ControlFlow::Continue(());
    };
    let state = world.scene.block(position.x, position.y, position.z);
    if !world.assets.registry.state(state).is_some_and(|row| row.name == ostrich::EGG) {
        return ControlFlow::Continue(());
    }
    world.player.events.push(MotionEvent::Fall(FallEvent::LevelEvent {
        event: ostrich::EGG_TRAMPLE_SOUND,
        position,
        data: pitch.to_bits() as i32,
    }));
    ControlFlow::Break(())
}
