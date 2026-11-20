fn deer_scale_load(game: &mut Game<'_>, tag: &Tag, mount: Option<u32>) -> std::ops::ControlFlow<Option<u32>> {
    use super::naturalist_deer_scale as scale;
    use std::{cell::Cell, ops::ControlFlow};
    thread_local! { static REENTER: Cell<bool> = const { Cell::new(false) }; }

    if REENTER.replace(false) || !scale::is_naturalist(tag) {
        return ControlFlow::Continue(());
    }
    let Some(packed) = scale::read(tag) else { return ControlFlow::Continue(()) };
    let tag = tag.clone();
    REENTER.set(true);
    let loaded = game.load_mob(tag, mount);
    if let Some(id) = loaded {
        scale::restore(game, id, Some(packed));
    }
    ControlFlow::Break(loaded)
}
