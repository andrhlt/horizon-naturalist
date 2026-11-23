fn ant_anger_save(game: &Game<'_>, mob: &Mob, tag: &mut Tag) {
    if mob.kind.id != "naturalist:ant" {
        return;
    }
    let Tag::Compound(fields) = tag else { return };
    fields.remove(&"NaturalistAngryActor".into());
    if mob.extra.anger_end > game.world.game_time
        && mob.extra.angry_at == Some(horizon_client::mobs::Who::Player)
        && let Some(Tag::IntArray(actor)) = game
            .persist
            .raw
            .get(&mob.id)
            .and_then(|raw| raw.get("NaturalistAngryActor"))
    {
        fields.insert("angry_at".into(), Tag::IntArray(actor.clone()));
    }
}
