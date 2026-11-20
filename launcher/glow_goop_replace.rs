fn glow_goop_replace(canceled: bool, game: &mut Game<'_>, context: &PlaceContext) -> bool {
    if !canceled {
        super::naturalist_glow_goop::replace(game, context);
    }
    canceled
}
