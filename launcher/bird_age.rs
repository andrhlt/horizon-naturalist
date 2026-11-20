fn bird_age(mob: &mut Mob, level: &MobWorld) {
    if mob.kind.id != "naturalist:bird" || !mob.alive() {
        return;
    }
    if can_age_up(mob) {
        let (width, height, _) = mob.dims();
        mob.extra.age += 1;
        if mob.extra.age == 0 {
            mob.fudge_position_after_size_change(level, width, height);
        }
    } else if mob.extra.age > 0 {
        mob.extra.age -= 1;
    }
}
