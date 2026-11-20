use super::*;
use horizon_client::mobs::{nav::naturalist_bass_state as bass, spawn_rules::Reason};
use std::collections::{BTreeMap, BTreeSet};

impl Game<'_> {
    pub(in crate::game) fn naturalist_fixture_bass_natural(&mut self) -> String {
        const MAX_SAMPLE: u32 = 384;
        let mut ranges = BTreeMap::<u8, BTreeSet<u32>>::new();
        let mut seen = BTreeSet::new();
        let mut variants_shared = true;
        let mut small_and_medium_follow = true;
        let mut large_follow = false;
        let mut rejects_after_target = true;
        for sample in 0..MAX_SAMPLE {
            let mut pending = Vec::new();
            let pos = [100.5 + f64::from(sample % 8), 58.0, 104.5 + f64::from(sample / 8)];
            let mut request = SpawnRequest { entity: "naturalist:bass", species: Species::Cod,
                position: pos, yaw: 0.0, group_index: 0, reason: Reason::Natural };
            let leader = self.finalize_spawned(&request, &mut pending).unwrap();
            let group = bass::group(&pending[0].ai.nav).unwrap().clone();
            let tier = bass::tier(&group.variant);
            ranges.entry(tier).or_default().insert(group.target);
            for member_index in 1..group.target {
                request.group_index = member_index;
                request.position[0] = pos[0] + f64::from(member_index);
                let accepted = self.finalize_spawned(&request, &mut pending).unwrap();
                let member = bass::group(&pending[member_index as usize].ai.nav).unwrap();
                variants_shared &= member.variant == group.variant;
                assert_eq!(accepted.school, member_index + 1 < group.target);
            }
            assert_eq!(leader.school, 1 < group.target);
            request.group_index = group.target;
            rejects_after_target &= self.finalize_spawned(&request, &mut pending).is_none();
            if seen.insert(tier) {
                let before = self.mobs.len();
                self.add_spawned(pending, false);
                let band = &self.mobs[before..];
                let actual = band.first().unwrap();
                if tier < 2 {
                    small_and_medium_follow &= actual.extra.followers == (group.target - 1) as i32
                        && band.iter().skip(1).all(|member| member.extra.leader == Some(actual.id));
                } else {
                    large_follow |= actual.extra.leader.is_some() || actual.extra.followers != 0;
                }
            }
        }
        format!(r#"{{"small_targets":{:?},"medium_targets":{:?},"large_targets":{:?},"variants_shared":{variants_shared},"small_and_medium_follow":{small_and_medium_follow},"large_follow":{large_follow},"rejects_after_target":{rejects_after_target}}}"#,
            ranges.get(&0).unwrap().iter().collect::<Vec<_>>(), ranges.get(&1).unwrap().iter().collect::<Vec<_>>(), ranges.get(&2).unwrap().iter().collect::<Vec<_>>())
    }
}
