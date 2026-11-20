fn zebra_chest_mount(game: &mut Game<'_>, index: usize, item: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    use horizon_client::mobs::naturalist_zebra::KIND;
    let mob = &game.mobs[index];
    if item != "minecraft:chest" || mob.kind.id != KIND || mob.extra.chested
        || !mob.alive() || !mob.extra.tame || mob.is_baby()
        || game.riding == Some(mob.id) || game.ridden_by_guest(mob.id)
        || game.keys.held(Action::Sneak) || game.keys.raw(&winit::keyboard::KeyCode::ShiftRight)
        || game.mobs.iter().any(|passenger| passenger.extra.mount == Some(mob.id) && !passenger.removed && passenger.alive())
    {
        return ControlFlow::Continue(());
    }
    let mob = &mut game.mobs[index];
    mob.extra.chested = true;
    let at = [mob.position.x, mob.position.y, mob.position.z];
    if !mob.silent {
        let pitch = 1.0 + (mob.random.next_float() - mob.random.next_float()) * 0.2;
        game.audio.play("minecraft:entity.mule.chest", Some(at), 1.0, pitch);
    }
    game.consume_held();
    game.world.modified.insert((at[0].floor() as i32 >> 4, at[2].floor() as i32 >> 4));
    ControlFlow::Break(true)
}
