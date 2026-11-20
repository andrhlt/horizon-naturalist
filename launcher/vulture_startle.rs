fn vulture_startle(hurt: bool, mob: &mut Mob, source: Damage) -> bool {
    if hurt && mob.kind.id == "naturalist:vulture" {
        let from = source.from.map(|[x, y, z]| Vec3::new(x, y, z));
        goals::naturalist_vulture_goals::startle(mob, from);
    }
    hurt
}
