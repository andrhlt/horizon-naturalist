fn hedgehog_load_dye(mob: &mut Mob, tag: &Tag) {
    if mob.kind.id == crate::mobs::naturalist_hedgehog::KIND {
        let saved = horizon_client_core::nbt_value::number(tag.get("DyeColor"));
        mob.extra.color = saved.filter(|color| (0..=15).contains(color))
            .map_or(u8::MAX, |color| color as u8);
    }
}
