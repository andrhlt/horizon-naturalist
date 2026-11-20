fn bird_prepare_call(game: &mut Game<'_>) {
    use horizon_client::mobs::nav::naturalist_bird_voice::set_call;
    use horizon_client_core::nbt_value::text;
    for mob in game.mobs.iter_mut().filter(|mob| mob.kind.id == "naturalist:bird") {
        let variant = text(game.persist.raw.get(&mob.id).and_then(|tag| tag.get("Variant")));
        set_call(&mut mob.ai.nav, variant.as_deref().unwrap_or_default());
    }
}
