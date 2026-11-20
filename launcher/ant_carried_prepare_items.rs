fn ant_carried_prepare_items(game: &Game<'_>, round: &mut native_ticks::TickRound) {
    round.skip_carried_food(game);
}
