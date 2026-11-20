// Derived from Horizon Game::save_item at 0a875da; LGPL-3.0-or-later.
fn ant_carried_save(
    game: &mut Game<'_>,
    i: usize,
    keys: &HashSet<(i32, i32)>,
) -> Option<((i32, i32), Tag)> {
    let at = game.dropped[i].position;
    keys.contains(&chunk_of(at)).then(|| {
        let id = game.dropped[i].id;
        let uuid = game.uuid_for(id);
        let mut tag = entity_nbt::item_entity_to_nbt_with_items(
            &game.dropped[i],
            uuid,
            game.persist.raw.get(&id),
            game.items(),
        );
        game.save_portal_cooldown(id, &mut tag);
        if let Tag::Compound(fields) = &mut tag
            && fields.contains_key(&"Ant".into())
        {
            fields.insert("id".into(), Tag::String("naturalist:carried_food".into()));
            fields.insert("Age".into(), Tag::Short(0));
            fields.remove(&"NaturalistUnresolvedSince".into());
        }
        (chunk_of(at), tag)
    })
}
