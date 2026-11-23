fn tortoise_lay_love(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    use horizon_core::nbt::Tag;
    if kind != "naturalist:tortoise" { return; }
    let pending = game.persist.raw.get(&entity).and_then(|tag| tag.get("NaturalistEggLaid"))
        .is_some_and(|tag| matches!(tag, Tag::Byte(value) if *value != 0));
    if !pending { return; }
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
        mob.extra.in_love = 599;
        mob.extra.love_player = false;
        mob.extra.love_cause = None;
    }
    game.love_cause.remove(&entity);
    game.native_entity_enter_love_mode(entity, kind, None);
    if let Some(Tag::Compound(fields)) = game.persist.raw.get_mut(&entity) {
        fields.remove(&"NaturalistEggLaid".into());
    }
}
