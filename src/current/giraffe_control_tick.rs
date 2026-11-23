fn giraffe_control_tick(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    if kind != "naturalist:giraffe" {
        return;
    }

    // Player passengers live in Game.riding / the level passenger list, not
    // Mob.rider (which tracks mob passengers). Treating rider == None as a
    // dismount clears the virtual saddle on the first tick after a player
    // boards, disabling Giraffe steering before it can move.
    let has_player_passenger = game.riding == Some(entity)
        || game
            .seated_passengers(entity)
            .iter()
            .any(|(_, _, is_player)| *is_player);
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity)
        && mob.rider.is_none()
        && !has_player_passenger
        && mob.extra.saddled()
    {
        // The control bit is virtual: Java Giraffes do not need or consume a
        // saddle. Clear it as soon as a rider dismounts, including after load.
        mob.extra.set_saddled(false);
    }
}
