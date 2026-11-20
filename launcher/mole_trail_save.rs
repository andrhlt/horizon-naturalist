fn mole_trail_save(game: &Game<'_>, index: usize) -> std::ops::ControlFlow<Option<(ChunkKey, Tag)>> {
    if game.mobs[index].kind.id == "naturalist:dirt_trail" {
        return std::ops::ControlFlow::Break(None);
    }
    std::ops::ControlFlow::Continue(())
}
