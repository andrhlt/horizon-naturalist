fn fox_wild_ones_generated(
    mut saved: Compound,
    mobs: &[(Mob, Option<Tag>)],
    index: usize,
) -> Compound {
    // The root generated mob has no source NBT; a structure's root always
    // comes from its existing template. Foxes never finalize as jockeys.
    let Some((mob, None)) = mobs.get(index) else { return saved };
    if index != 0 || mob.kind.id != "minecraft:fox" || mob.is_baby() {
        return saved;
    }
    let mut fox = mob.clone();
    if !crate::mobs::fox::naturalist_wild_ones::equip(&mut fox) {
        return saved;
    }
    if let Tag::Compound(mut gear) = entity_nbt::mob_to_nbt(&fox, [0; 4], None, None, |_| None) {
        for key in ["equipment", "drop_chances", "CanPickUpLoot"] {
            if let Some(value) = gear.remove(&key.into()) {
                saved.insert(key.into(), value);
            }
        }
    }
    saved
}
