fn komodo_tick(mob: &mut Mob, cx: &Cx<'_, '_>) {
    use super::naturalist_komodo::{KIND, update_proximity};
    if mob.kind.id == KIND {
        update_proximity(mob, cx.w);
    }
}
