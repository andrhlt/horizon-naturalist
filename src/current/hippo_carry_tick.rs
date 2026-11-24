fn hippo_carry_tick(game: &mut Game<'_>, entity: u32, kind: &str) {
    use horizon_core::nbt::Tag;
    if kind != "naturalist:hippo" {
        return;
    }
    let timer = game
        .persist
        .raw
        .get(&entity)
        .and_then(|raw| raw.get("NaturalistHippoCarryTimer"))
        .and_then(horizon_world::datafix::dynamic::as_number)
        .map(|value| value.int_value());
    let Some(timer) = timer else { return };
    let Some(index) = game.mobs.iter().position(|mob| mob.id == entity) else {
        return;
    };
    if timer <= 1 {
        game.mobs[index].extra.set_main_hand_stack(None);
        if let Some(Tag::Compound(fields)) = game.persist.raw.get_mut(&entity) {
            fields.remove("NaturalistHippoCarryTimer");
        }
    } else if let Some(Tag::Compound(fields)) = game.persist.raw.get_mut(&entity) {
        fields.insert("NaturalistHippoCarryTimer".into(), Tag::Int(timer - 1));
    }
}
