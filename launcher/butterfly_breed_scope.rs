fn butterfly_breed_scope(
    game: &mut Game<'_>,
    parent: u32,
    partner: u32,
) -> std::ops::ControlFlow<()> {
    naturalist_butterfly_breeding::butterfly_breed(game, parent, partner)
}
