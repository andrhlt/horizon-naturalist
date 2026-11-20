fn deer_scale_update(
    game: &mut Game<'_>,
    command: &str,
    context: &Context,
    target: Target,
    attribute: &str,
) -> std::ops::ControlFlow<R<Option<(i32, Text)>>> {
    use std::{cell::Cell, ops::ControlFlow};

    let Target::Mob(id) = target else { return ControlFlow::Continue(()) };
    if attribute != "minecraft:scale"
        || !game.mobs.iter().any(|mob| mob.id == id && mob.kind.id.starts_with("naturalist:"))
    {
        return ControlFlow::Continue(());
    }

    struct ScaleCommand;
    thread_local! { static ACTIVE: Cell<bool> = const { Cell::new(false) }; }
    impl Drop for ScaleCommand {
        fn drop(&mut self) {
            ACTIVE.set(false);
        }
    }

    if ACTIVE.replace(true) {
        return ControlFlow::Continue(());
    }
    let _guard = ScaleCommand;
    let result = game.attribute_in_level(command, context, target, attribute);
    let scale = game.cmds.attributes.get_mut(&target).and_then(|attributes| {
        let key = attributes.catalog().id(attribute)?;
        attributes.value(key).ok()
    });
    if let Some(scale) = scale
        && let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id)
    {
        mob.extra.model_scale = scale as f32;
    }
    ControlFlow::Break(result)
}
