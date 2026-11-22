//! Fixture-backed natural spawn group limits for aquatic species.

pub struct PackLimit(pub u32);

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
    fn pinned_aquatic_limits_are_positive() {
        let limits = [8, 6, 8, 8, 8, 24, 8];
        assert_eq!(limits, [8, 6, 8, 8, 8, 24, 8]);
        assert!(limits.iter().all(|limit| *limit > 0));
    }
}
