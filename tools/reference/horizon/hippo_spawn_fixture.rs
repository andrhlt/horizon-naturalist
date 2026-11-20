use crate::game::naturalist_deer_placement::{Pass, PlacementPass};
use super::*;

impl Game<'_> {
    pub(in crate::game) fn naturalist_fixture_hippo_spawn(&self, generation: bool) -> bool {
        use horizon_client::mobs::spawn_rules::{Reason, check_spawn_rules};
        let (pass, reason) = if generation {
            (Pass::Generation, Reason::ChunkGeneration)
        } else {
            (Pass::Natural, Reason::Natural)
        };
        let _placement = PlacementPass::enter(pass).expect("an isolated fixture placement pass");
        let level = self.natural_level();
        let (species, _, _, _) = level.mob_type("naturalist:hippo").unwrap();
        check_spawn_rules(species, &level, reason, &mut LegacyRandom::new(5), 8, 132, 8)
    }
}
