fn alligator_tick(
    keep: bool,
    mob: &mut Mob,
    world: &MobWorld<'_>,
    events: &mut Vec<MobEvent>,
) -> bool {
    use super::nav::naturalist_catfish_state as hunt;
    if !keep || mob.kind.id != "naturalist:alligator" || !mob.alive() {
        return keep;
    }
    let cooldown = hunt::cooldown(&mob.ai.nav);
    if cooldown > 0 {
        hunt::set_cooldown(&mut mob.ai.nav, cooldown - 1);
    }
    if mob.fluids.in_water
        && let Some(path) = &mob.ai.nav.path
        && !path.is_done()
    {
        let node = path.next_node();
        if world.fluid(node.x, node.y, node.z) == crate::blocks::Fluid::None {
            let [x, _, z] = mob.look_angle();
            mob.velocity.x += x * 0.025;
            mob.velocity.y += 0.05;
            mob.velocity.z += z * 0.025;
        }
    }
    if mob.extra.flag2 && mob.extra.counter > 0 && mob.extra.counter % 5 == 0 {
        let pos = mob.position.components().map(|v| v.floor() as i32);
        let [x, y, z] = pos;
        let substrate = world.block_name(x, y - 1, z);
        if world.has_tag("naturalist:alligator_egg_layable_on", substrate) {
            events.push(MobEvent::BlockBreakEffect {
                pos,
                block: [x, y - 1, z],
            });
        }
    }
    keep
}
