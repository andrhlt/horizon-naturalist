use super::{Game, ItemStack};

impl Game<'_> {
    pub(in crate::game) fn rat_has_container(&mut self, pos: [i32; 3], peek: bool) -> bool {
        let [x, y, z] = pos;
        if self.containers().view((x, y, z), false).is_some() {
            return true;
        }
        let center = pos.map(|axis| f64::from(axis) + 0.5);
        let previous = peek.then(|| (*self.world.random).clone());
        let has = self.entity_container_at(center).is_some();
        if let Some(random) = previous {
            *self.world.random = random;
        }
        has
    }

    pub(in crate::game) fn rat_deposit(&mut self, pos: [i32; 3], stack: ItemStack) -> Option<bool> {
        let [x, y, z] = pos;
        if let Some(container) = self.containers().view((x, y, z), false) {
            return Some(self.add_item(None, &container, stack, None).is_none());
        }
        let id = self.entity_container_at(pos.map(|axis| f64::from(axis) + 0.5))?;
        self.with_entity_container(id, |game, container| {
            game.add_item(None, container, stack, None).is_none()
        })
    }
}
