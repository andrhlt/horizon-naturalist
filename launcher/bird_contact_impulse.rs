fn bird_contact_impulse(ordinary_skip: bool, mob: &Mob) -> bool {
    // Bird.doPush is empty. The enclosing loop still checks entity cramming.
    ordinary_skip || mob.kind.id == "naturalist:bird"
}
