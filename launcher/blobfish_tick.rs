fn blobfish_tick(kept: bool, mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) -> bool {
    use super::nav::naturalist_blobfish_state as blobfish;
    if !kept || !mob.alive() || mob.kind.id != "naturalist:blobfish" {
        return kept;
    }
    let wants_gray = mob.fluids.in_water && mob.position.y <= f64::from(world.sea_level - blobfish::DEEP_OFFSET);
    let nav = &mut mob.ai.nav;
    if wants_gray == blobfish::gray(nav) {
        blobfish::set_conversion(nav, 0);
    } else {
        let ticks = blobfish::conversion(nav).saturating_add(1);
        if ticks < blobfish::CONVERSION_TICKS {
            blobfish::set_conversion(nav, ticks);
        } else {
            blobfish::set_gray(nav, wants_gray);
            blobfish::set_conversion(nav, 0);
            let at = mob.position;
            let size = mob.bounds().size();
            for _ in 0..16 {
                let x = size.x * (2.0 * mob.random.next_double() - 1.0);
                let y = size.y * mob.random.next_double();
                let z = size.z * (2.0 * mob.random.next_double() - 1.0);
                events.push(MobEvent::ItemParticles { item: "naturalist:blobfish",
                    at: [at.x + x, at.y + y, at.z + z], spread: [0.0; 3], speed: 0.05, count: 1 });
            }
            mob.voice("naturalist:entity.blobfish.hurt", events);
        }
    }
    kept
}
