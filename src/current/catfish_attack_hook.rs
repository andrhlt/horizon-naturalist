fn catfish_attack(
    game: &mut Game<'_>,
    target: u32,
    source: Damage,
) -> std::ops::ControlFlow<bool> {
    crate::game::naturalist_catfish::attack(game, target, source)
}
