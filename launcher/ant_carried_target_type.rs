fn ant_carried_target_type(game: &Game<'_>, target: Target) -> std::ops::ControlFlow<String> {
    if let Target::Item(id) = target
        && game.persist.raw.get(&id).is_some_and(super::naturalist_ant_workers::is_carried_food)
    {
        return std::ops::ControlFlow::Break("naturalist:carried_food".into());
    }
    std::ops::ControlFlow::Continue(())
}
