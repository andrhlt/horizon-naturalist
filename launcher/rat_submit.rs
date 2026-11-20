fn rat_submit(
    game: &Game<'_>,
    mob: &NativeMobRender<'_>,
    draw: &mut BlockEntityDraw,
    drawn: &mut bool,
) {
    if !*drawn
        && mob.kind == "naturalist:rat"
        && let Some(entity) = game.mobs.iter().find(|entity| entity.id == mob.id)
    {
        *drawn = game.draw_naturalist_rat(entity, mob, draw);
    }
}
