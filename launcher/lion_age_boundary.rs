fn lion_age_boundary(mob: &mut Mob) {
    use super::goals::naturalist_lion_goals as lion;
    if mob.kind.id == lion::KIND {
        lion::roll_mane(mob);
    }
}
