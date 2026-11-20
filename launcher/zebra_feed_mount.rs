fn zebra_feed_mount(game: &mut Game<'_>, index: usize, item: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    use horizon_client::mobs::naturalist_zebra::KIND;
    let mob = &mut game.mobs[index];
    if mob.kind.id != KIND || mob.silent {
        return ControlFlow::Continue(());
    }
    let id = mob.id;
    // Keep the stock feeding, inventory and love behavior. Suppress the
    // template's eating sound while it handles the input.
    mob.silent = true;
    let ate = game.feed_mount(index, item);
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id) {
        mob.silent = false;
        if ate {
            let at = [mob.position.x, mob.position.y, mob.position.z];
            let pitch = 1.0 + (mob.random.next_float() - mob.random.next_float()) * 0.2;
            game.audio.play("naturalist:entity.zebra.eat", Some(at), 1.0, pitch);
        }
    }
    ControlFlow::Break(ate)
}
