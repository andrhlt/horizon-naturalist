fn clam_goals(mob: &mut Mob) {
    use super::nav::naturalist_clam as clam;

    if mob.kind.id == clam::KIND {
        clam::set_open(mob, false);
        mob.extra.drop_chances.get_or_insert_with(|| Box::new([super::drops::DEFAULT_DROP_CHANCE; 8]))[super::finalize::MAINHAND] = 0.0;
    }
}
