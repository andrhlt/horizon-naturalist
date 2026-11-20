fn crab_loaded(mob: &mut Mob, tag: &Tag) {
    use crate::mobs::nav::naturalist_crab as crab;

    if mob.kind.id == crab::KIND {
        let state = crab::state_mut(&mut mob.ai.nav);
        state.from_hand = flag(tag, "FromHand");
        state.no_pickup = tag.get("CanPickUpLoot").is_some() && !flag(tag, "CanPickUpLoot");
    }
}
