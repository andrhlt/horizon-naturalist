fn ostrich_mount_menu(game: &Game<'_>) -> std::ops::ControlFlow<bool> {
    if game.riding.and_then(|id| game.mount_mob_index(id))
        .is_some_and(|index| game.mobs[index].kind.id == "naturalist:ostrich")
    {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
