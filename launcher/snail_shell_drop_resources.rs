fn snail_shell_drop_resources(
    level: &mut GameLevel<'_>,
    pos: BlockPos,
    state: BlockStateId,
) -> std::ops::ControlFlow<()> {
    use std::ops::ControlFlow;
    let key = (pos.x, pos.y, pos.z);
    use super::naturalist_snail_shell::{flower, loot};
    let Some(drops) = loot(
        level.registry,
        state,
        flower(level.world.block_entities.get(&key)),
    ) else {
        return ControlFlow::Continue(());
    };

    if level
        .drop_tool
        .as_ref()
        .is_some_and(|(target, _)| *target == key)
        && let Some(kept) = level.drop_into.as_mut()
    {
        kept.extend(drops);
    } else {
        for stack in drops {
            level.pop_resource([pos.x, pos.y, pos.z], stack);
        }
    }
    ControlFlow::Break(())
}
