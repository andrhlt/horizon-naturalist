fn anglerfish_reach(ordinary: bool, mob: &Mob, context: &Cx<'_, '_>) -> bool {
    if super::nav::naturalist_elephants::includes(&mob.kind.id) {
        let Some(target) = mob.target_info(context.w) else { return false };
        let width = mob.width() as f32;
        return mob.distance_sqr(target.position) <= f64::from(width * width);
    }
    if mob.kind.id == "naturalist:alligator" {
        let Some(target) = mob.target_info(context.w) else { return false };
        let reach = mob.width() as f32 * 1.2;
        return mob.distance_sqr(target.position) <= f64::from(reach * reach);
    }
    if mob.kind.id != "naturalist:anglerfish" {
        return ordinary;
    }
    let Some(target) = mob.target_info(context.w) else { return false };
    let width = mob.width() as f32;
    let target_width = target.bounds.size().x as f32;
    let reach = width * 2.0 * width * 2.0 + target_width;
    mob.distance_sqr(target.position) <= f64::from(reach)
}
