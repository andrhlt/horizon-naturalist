fn crab_whistle_interact(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::{actions::Action, text::Text};
    use horizon_core::nbt::Tag;
    use mod_api::{Host, SoundSource};
    use std::ops::ControlFlow;
    let Some(index) = game.mob_interaction_target(target) else { return ControlFlow::Continue(()) };
    if game.mobs[index].kind.id != "naturalist:crab" || !game.mobs[index].alive() { return ControlFlow::Continue(()) }
    let id = game.mobs[index].id;
    let actor = game.acting_player_uuid();
    let item = game.held().map(|stack| stack.item.to_string()).unwrap_or_default();
    if item != "naturalist:whistle" || !game.mobs[index].extra.tame || game.mobs[index].extra.owner != Some(actor)
        || game.keys.held(Action::Sneak) || game.cooldowns.is_on_cooldown(&item) { return ControlFlow::Continue(()) }
    let following = game.persist.raw.get(&id).and_then(|tag|tag.get("FollowingOwner"))
        .and_then(horizon_world::datafix::dynamic::as_number).is_none_or(|n|n.int_value()!=0);
    let at = game.mobs[index].center();
    horizon_client::mobs::tame::restore_sitting(&mut game.mobs[index],false);
    if following { game.mobs[index].ai.target = None; }
    let raw = game.persist.raw.entry(id).or_insert_with(||Tag::Compound(Default::default()));
    if let Tag::Compound(fields)=raw { fields.insert("FollowingOwner".into(), Tag::Byte(i8::from(!following))); }
    game.add_cooldown(std::sync::Arc::from(item),20);
    if let Some(level)=game.level(game.dim.id()) { level.play_sound(game,"naturalist:item.whistle",at,SoundSource::Neutral,0.8,1.0); }
    let name=game.mobs[index].custom_name.as_deref().unwrap_or("Crab").to_owned();
    let mode=Text::translate(if !following {"naturalist.whistle.following"} else {"naturalist.whistle.wandering"}, &[]);
    game.send_overlay_message(Text::translate("naturalist.whistle.message", &[Text::literal(name),mode]));
    ControlFlow::Break(Ok(true))
}
