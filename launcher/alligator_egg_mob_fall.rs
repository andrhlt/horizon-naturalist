fn alligator_egg_mob_fall(
    mob: &mut Mob,
    world: &MobWorld<'_>,
    events: &mut Vec<MobEvent>,
) -> std::ops::ControlFlow<()> {
    use std::ops::ControlFlow;
    let pos = mob.block_on_legacy(world);
    let [x, y, z] = pos;
    let owner = match world.block_name(x, y, z) {
        "naturalist:alligator_egg" => "naturalist:alligator",
        "naturalist:tortoise_egg" => "naturalist:tortoise",
        "naturalist:ostrich_egg" => "naturalist:ostrich",
        _ => return ControlFlow::Continue(()),
    };
    let fall = mob.fall_distance;
    if mob.kind.id != owner
        && !super::doors::is_zombie(mob.species())
        && !world
            .data
            .entity_has("naturalist:safe_egg_walkers", &mob.kind.id)
        && world.mob_griefing
        && mob.random.next_int_bound(3) == Ok(0)
    {
        events.push(MobEvent::CrackTurtleEgg { pos });
    }
    mob.cause_fall_damage_at(
        fall,
        1.0,
        events,
        Some((world, pos)),
        super::DamageKind::Fall,
    );
    ControlFlow::Break(())
}
