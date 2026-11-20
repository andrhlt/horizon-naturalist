fn boar_lightning(game: &mut Game<'_>, id: u32, outcome: &str, cancel: &mut bool) {
    if outcome != "minecraft:zombified_piglin" {
        return;
    }
    let Some(index) = game.mobs.iter().position(|mob| mob.id == id && mob.kind.id == "naturalist:boar") else {
        return;
    };
    let Some(kind) = game.res.mob_types().get("minecraft:zoglin").cloned() else {
        return;
    };
    let source = &mut game.mobs[index];
    let mut zoglin = horizon_client::mobs::Mob::new(0, kind, source.position, source.yaw);
    horizon_client::mobs::finalize::set_baby(&mut zoglin, source.is_baby());
    zoglin.no_ai = source.no_ai;
    zoglin.name_visible = source.name_visible;
    zoglin.custom_name = source.custom_name.clone();
    zoglin.custom_name_text = source.custom_name_text.clone();
    zoglin.persistent = true;
    zoglin.head_pitch = source.head_pitch;
    source.removed = true;
    *cancel = true;
    let _ = game.handle_mob_events(vec![horizon_client::mobs::MobEvent::Spawn(Box::new(zoglin))]);
}
