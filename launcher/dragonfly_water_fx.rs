fn dragonfly_water_fx(was_touching: bool, mob: &Mob) -> bool {
    // This local only gates water-entry effects. The real fluid state is updated below.
    was_touching || mob.kind.id == "naturalist:dragonfly"
}
