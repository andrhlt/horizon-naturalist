fn antivenom_splash_mobs(game: &mut Game<'_>, item: &ItemStack, at: Vec3, age: u32) {
    use super::naturalist_antivenom as antivenom;
    if item.item.as_ref() == "minecraft:splash_potion" && item.potion() == Some(antivenom::ID) {
        // The hub is held by the caller while a guest's projectile hit runs.
        if let Some(conn) = game.lan_acting && game.lan.is_none() {
            let attachments = &mut game.world.attachments;
            if let Ok(mut hits) = attachments.get(&antivenom::PENDING_SPLASHES) {
                hits.push((at, age, conn));
                let _ = attachments.set(&antivenom::PENDING_SPLASHES, hits);
            }
        }
        for mob in game.mobs.iter_mut().filter(|mob| mob.alive() && splash_distance_sqr(at, age, mob.bounds()).is_some()) {
            antivenom::clear_mob(mob);
        }
    }
}
