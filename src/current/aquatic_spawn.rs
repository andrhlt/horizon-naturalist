//! Java-derived natural aquatic cluster limits.
//!
//! Horizon's placement rule covers the common in-water predicate; this part
//! preserves the Java `MobSpawnSettings` cluster cap independently of the
//! species-specific depth/light predicates that still need an engine seam.

pub(crate) struct PackLimit(pub(crate) u32);

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
}
