fn deer_variant_bred_coin(game: &mut Game<'_>, parent: u32) -> std::ops::ControlFlow<bool> {
    super::super::naturalist_deer_variants::bred_coat(game, parent).map_or(
        std::ops::ControlFlow::Continue(()),
        std::ops::ControlFlow::Break,
    )
}
