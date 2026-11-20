fn aquatic_wet_player_bind(mob: &Mob) -> std::ops::ControlFlow<Option<u32>> {
    // The outer option distinguishes normal player routing from selecting the local host.
    match *mob.ai.nav.__horizon_mixins_naturalist_wet_player_target() {
        Some(choice) => std::ops::ControlFlow::Break(choice),
        None => std::ops::ControlFlow::Continue(()),
    }
}
