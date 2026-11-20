fn variant_pool_fixture(game: &mut Game<'_>, command: &str) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    let Some(args) = command.strip_prefix("naturalist_fixture_variant_pool ") else {
        return ControlFlow::Continue(());
    };
    if !game.options.headless {
        return ControlFlow::Continue(());
    }
    let mut args = args.split_whitespace();
    let Some(kind) = args.next() else { return ControlFlow::Break(false) };
    let Some(report) = super::naturalist_deer_variants::fixture_pools(game, kind, args) else {
        return ControlFlow::Break(false);
    };
    game.feedback(Text::literal(format!("Naturalist variant pools {report}")));
    ControlFlow::Break(true)
}
