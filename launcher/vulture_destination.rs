fn vulture_destination(stable: bool, mob: &Mob, world: &MobWorld, at: [i32; 3]) -> bool {
    let [x, y, z] = at;
    stable
        && (mob.kind.id != "naturalist:vulture"
            || world.has_tag("naturalist:vulture_perch_blocks", world.block_name(x, y, z)))
}
