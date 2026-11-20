use super::Game;
use std::cell::Cell;

thread_local! {
    static BUTTERFLY_BREED: Cell<bool> = const { Cell::new(false) };
}

pub(super) fn butterfly_breed(
    game: &mut Game<'_>,
    parent: u32,
    partner: u32,
) -> std::ops::ControlFlow<()> {
    use std::ops::ControlFlow;
    if BUTTERFLY_BREED.get()
        || !game.mobs.iter().any(|mob| mob.id == parent && mob.kind.id == "naturalist:butterfly")
    {
        return ControlFlow::Continue(());
    }
    BUTTERFLY_BREED.set(true);
    game.breed(parent, partner, None);
    BUTTERFLY_BREED.set(false);
    ControlFlow::Break(())
}

pub(super) fn butterfly_offspring(kind: &str) -> &str {
    if kind == "naturalist:butterfly" && BUTTERFLY_BREED.get() {
        "naturalist:caterpillar"
    } else {
        kind
    }
}
