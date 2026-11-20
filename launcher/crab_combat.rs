use super::Game;
use horizon_client::mobs::{finalize::MAINHAND, nav::naturalist_crab as crab};

impl Game<'_> {
    pub(in crate::game) fn naturalist_crab_hit(&mut self, id: u32) {
        let Some(mob) = self.mobs.iter_mut().find(|mob| mob.id == id && mob.kind.id == crab::KIND) else {
            return;
        };
        let at = mob.position.components();
        let silent = mob.silent;
        crab::state_mut(&mut mob.ai.nav).swing_until = mob.age as u64 + 13;
        mob.needs_sync = true;
        if !silent {
            self.audio.play("naturalist:entity.crab.pincer", Some(at), 1.0, 1.0);
        }
        self.wear_mob_piece(id, MAINHAND, 1);
    }
}
