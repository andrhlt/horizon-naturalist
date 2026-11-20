fn fish_school_cleanup(mob: &mut Mob, world: &MobWorld<'_>, level_random: &mut impl RandomSource) -> std::ops::ControlFlow<()> {
    use super::nav::{naturalist_bass_state as bass, naturalist_catfish_state as catfish, naturalist_piranha_state as piranha};
    use std::ops::ControlFlow;
    if mob.species() != Species::Cod {
        return ControlFlow::Continue(());
    }
    if mob.extra.followers > 0 && level_random.next_int_bound(200).unwrap_or(0) == 1 {
        let identity = |seen: &crate::mobs::Seen| {
            if bass::snapshot_tier(seen).is_some() { bass::SMALLEST_SNAPSHOT }
            else if matches!(seen.variant, piranha::SNAPSHOT | catfish::SNAPSHOT | catfish::STARFISH_SNAPSHOT | catfish::CLAM_SNAPSHOT | catfish::ANGLERFISH_SNAPSHOT | super::nav::naturalist_blobfish_state::SNAPSHOT | super::nav::naturalist_jellyfish_state::SNAPSHOT | piranha::RAY_SNAPSHOT) { seen.variant }
            else { 0 }
        };
        let own = match mob.kind.id.as_str() {
            "naturalist:bass" => bass::SMALLEST_SNAPSHOT,
            "naturalist:piranha" => piranha::SNAPSHOT,
            "naturalist:catfish" => catfish::SNAPSHOT,
            _ => 0,
        };
        let area = mob.bounds().inflate(Vec3::new(8.0, 8.0, 8.0));
        if !world.others_near(area).any(|seen| seen.id != mob.id && seen.alive
            && seen.species == Species::Cod && seen.bounds.intersects(area) && identity(seen) == own)
        {
            mob.extra.followers = 0;
        }
    }
    ControlFlow::Break(())
}
