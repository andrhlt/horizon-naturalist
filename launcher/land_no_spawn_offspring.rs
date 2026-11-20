fn land_no_spawn_offspring(game: &Game<'_>, index: usize) -> std::ops::ControlFlow<bool> {
    if matches!(game.mobs[index].kind.id.as_str(),
        "naturalist:lizard" | "naturalist:desert_scorpion" | "naturalist:jungle_scorpion")
    {
        return std::ops::ControlFlow::Break(false);
    }
    std::ops::ControlFlow::Continue(())
}
