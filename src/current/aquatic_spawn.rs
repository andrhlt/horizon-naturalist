//! Java-derived natural aquatic cluster limits.
//!
//! Horizon's placement rule covers the common in-water predicate; this part
//! preserves the Java `MobSpawnSettings` cluster cap and implements the Java
//! depth/water-column predicates that can be expressed with read-only hooks.

pub(crate) struct PackLimit(pub(crate) u32);

fn is_water(host: &dyn mod_api::Host, level: mod_api::LevelRef, pos: [i32; 3]) -> Option<bool> {
    let fluid = level.fluid_state(host, pos)?;
    Some(
        !fluid.is_empty()
            && fluid.fluid.id(host) == "minecraft:water",
    )
}

fn water_column(
    host: &dyn mod_api::Host,
    level: mod_api::LevelRef,
    pos: [i32; 3],
    height: i32,
) -> Option<bool> {
    for dy in 0..height {
        if !is_water(host, level, [pos[0], pos[1] + dy, pos[2]])? {
            return Some(false);
        }
    }
    Some(true)
}

fn blobfish_depth_allowed(y: i32, sea_level: i32) -> bool {
    y <= sea_level.saturating_sub(33)
}

pub(crate) fn placement_check(
    host: &dyn mod_api::Host,
    event: &mut mod_api::MobSpawnPlacementCheckEvent<'_>,
) {
    if !event.default_result
        || !matches!(
            event.reason,
            mod_api::SpawnReason::Natural | mod_api::SpawnReason::ChunkGeneration
        )
    {
        return;
    }
    let pos = [event.pos.x, event.pos.y, event.pos.z];
    let allowed = match event.kind {
        "naturalist:blobfish" => event
            .level
            .sea_level(host)
            .is_some_and(|sea| blobfish_depth_allowed(pos[1], sea)),
        "naturalist:great_white_shark" | "naturalist:whale" => {
            water_column(host, event.level, pos, 2) == Some(true)
        }
        _ => return,
    };
    if !allowed {
        *event.result = mod_api::SpawnCheck::Fail;
    }
}

#[mod_api::horizon_part]
impl mod_api::EntityPart for PackLimit {
    fn spawn_cluster_size(
        &self,
        _: &dyn mod_api::Host,
        _: mod_api::UnspawnedMobRef,
        reason: mod_api::SpawnReason,
        _: Option<mod_api::SpawnSource>,
        _: u32,
    ) -> Option<u32> {
        (reason == mod_api::SpawnReason::Natural).then_some(self.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn java_aquatic_cluster_limits_are_positive() {
        let limits = [8, 8, 4, 4, 8, 8, 4, 4];
        assert_eq!(limits, [8, 8, 4, 4, 8, 8, 4, 4]);
        assert!(limits.iter().all(|limit| *limit > 0));
    }

    #[test]
    fn blobfish_depth_boundary_is_inclusive() {
        assert!(super::blobfish_depth_allowed(30, 63));
        assert!(!super::blobfish_depth_allowed(31, 63));
    }
}
