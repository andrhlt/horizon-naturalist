fn snail_shell_submit(
    game: &Game<'_>,
    entity: &ModBlockEntity,
    state: BlockStateId,
    draw: &mut BlockEntityDraw,
) {
    if entity.id == super::naturalist_snail_shell::SHELL {
        super::naturalist_snail_shell::draw(game, state, entity.nbt.get(&"Flower".into()), draw);
    }
}
