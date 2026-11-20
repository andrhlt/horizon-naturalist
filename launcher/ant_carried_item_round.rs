use super::{Game, TickRound};

impl TickRound {
    pub(in crate::game) fn skip_carried_food(&mut self, game: &Game<'_>) {
        let raw = &game.persist.raw;
        for item in &game.dropped {
            if raw.get(&item.id).is_some_and(crate::game::naturalist_ant_workers::is_carried_food) {
                self.canceled.push(item.id);
                self.not_ticked(item.id);
            }
        }
    }
}
