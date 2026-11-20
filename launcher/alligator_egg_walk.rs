fn alligator_egg_walk(
    mob: &Mob,
    world: &MobWorld<'_>,
    random: &mut LegacyRandom,
    events: &mut Vec<MobEvent>,
) {
    if !mob.on_ground || !mob.alive() || !world.mob_griefing {
        return;
    }
    let pos = mob.block_on_legacy(world);
    let [x, y, z] = pos;
    let owner = match world.block_name(x, y, z) {
        "naturalist:alligator_egg" => "naturalist:alligator",
        "naturalist:tortoise_egg" => "naturalist:tortoise",
        "naturalist:ostrich_egg" => "naturalist:ostrich",
        _ => return,
    };
    if mob.kind.id != owner
        && !world
            .data
            .entity_has("naturalist:safe_egg_walkers", &mob.kind.id)
        && random.next_int_bound(100) == Ok(0)
    {
        events.push(MobEvent::CrackTurtleEgg { pos });
    }
}
