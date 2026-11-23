fn giraffe_control_tick(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    if kind != "naturalist:giraffe" {
        return;
    }

    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity)
        && mob.rider.is_none()
        && mob.extra.saddled()
    {
        // The control bit is virtual: Java Giraffes do not need or consume a
        // saddle. Clear it as soon as a rider dismounts, including after load.
        mob.extra.set_saddled(false);
    }
}
