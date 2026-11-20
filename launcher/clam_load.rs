fn clam_load(mob: &mut Mob, tag: &Tag) {
    use crate::mobs::nav::naturalist_clam as clam;
    if mob.kind.id == clam::KIND {
        let shell = clam::state_mut(&mut mob.ai.nav);
        shell.open = flag(tag, "Open");
        shell.has_treasure = flag(tag, "HasTreasure");
    }
}
