fn bird_egg_call(game: &mut Game<'_>, id: u32) {
    use horizon_client::mobs::nav::naturalist_bird_voice::set_call;
    use horizon_client_core::nbt_value::text;

    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == id && mob.kind.id == "naturalist:bird") {
        let variant = text(game.persist.raw.get(&id).and_then(|tag| tag.get("Variant")));
        set_call(&mut mob.ai.nav, variant.as_deref().unwrap_or_default());
    }
}
