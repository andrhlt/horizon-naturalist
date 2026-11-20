use super::{
    Game,
    pickups::{Collector, Taken},
};
use horizon_client::{
    item_sync::food,
    items::ItemStack,
    mobs::{
        equip::Slot,
        finalize::{item_stack, set_item_stack},
        goals::naturalist_vulture_goals::{FOOD, KIND, state, state_mut},
    },
};
use horizon_core::{geometry::Vec3, random::RandomSource};
use std::sync::Arc;

impl Game<'_> {
    pub(in crate::game) fn prepare_naturalist_vultures(&mut self) {
        if !self.mobs.iter().any(|mob| mob.kind.id == KIND) {
            return;
        }
        let mut eligible = Vec::new();
        self.for_each_player(|game| {
            if game.vulture_can_steal() {
                eligible.push(game.acting_player_uuid());
            }
        });
        let eligible = Arc::new(eligible);
        let reserved: Arc<Vec<(u32, [i32; 3])>> = Arc::new(
            self.mobs
                .iter()
                .filter(|mob| mob.kind.id == KIND && mob.alive())
                .filter_map(|mob| Some((mob.id, state(mob).perch_target?)))
                .collect(),
        );
        let mut items = super::item_areas::ItemAreas::default();
        for mob in self
            .mobs
            .iter_mut()
            .filter(|mob| mob.kind.id == KIND && mob.alive())
        {
            let area = mob.bounds().inflate(Vec3::new(12.0, 24.0, 12.0));
            let food = if mob.extra.main_hand.as_deref() == Some(FOOD) {
                None
            } else {
                let mut scan = items.scan(&self.dropped, self.next_entity, area);
                let mut index = 0;
                loop {
                    index = scan.next_from(index, self.dropped.len());
                    let Some(item) = self.dropped.get(index) else {
                        break None;
                    };
                    if item.stack.item.as_ref() == FOOD && area.intersects(item.bounds()) {
                        break Some([item.position.x, item.position.y, item.position.z]);
                    }
                    index += 1;
                }
            };
            let values = state_mut(mob);
            values.vulnerable_players = eligible.clone();
            values.perch_reservations = reserved.clone();
            values.food_pos = food;
        }
    }

    fn vulture_can_steal(&self) -> bool {
        !self.creative
            && !self.spectator()
            && self.player.as_ref().is_some_and(|p| {
                let health = p.health.health();
                health > 0.0 && health <= 6.0
            })
            && self.held().is_some()
    }

    pub(in crate::game) fn tick_naturalist_vultures(&mut self) {
        let mut items = super::item_areas::ItemAreas::default();
        for index in 0..self.mobs.len() {
            let mob = &self.mobs[index];
            if mob.kind.id != KIND
                || !mob.alive()
                || mob.removed
                || !self.creatures.ticked.contains(&mob.id)
            {
                continue;
            }
            let id = mob.id;
            let pos = mob.position;
            let at = [pos.x, pos.y, pos.z];
            if let Some(who) = state_mut(&mut self.mobs[index]).steal.take() {
                self.for_each_player(|game| {
                    if game.acting_player_uuid() != who
                        || !game.vulture_can_steal()
                        || game.mobs[index].extra.main_hand.is_some()
                    {
                        return;
                    }
                    let Some(mut stolen) = game.held().cloned() else {
                        return;
                    };
                    stolen.count = 1;
                    game.consume_held();
                    game.vulture_equip(index, stolen);
                    let pitch = ((game.world.random.next_float() - game.world.random.next_float())
                        * 0.7
                        + 1.0)
                        * 2.0;
                    game.audio
                        .play("minecraft:entity.item.pickup", Some(at), 0.2, pitch);
                });
            }
            let mob = &self.mobs[index];
            if self.ui.rules.bool("mob_griefing")
                && mob.extra.pick_up_loot
                && mob.extra.main_hand.as_deref() != Some(FOOD)
            {
                let area = mob.bounds().inflate(Vec3::new(1.0, 1.0, 1.0));
                let mut scan = items.scan(&self.dropped, self.next_entity, area);
                let mut i = 0;
                loop {
                    i = scan.next_from(i, self.dropped.len());
                    let Some(item) = self.dropped.get(i) else {
                        break;
                    };
                    if item.stack.item.as_ref() == FOOD && area.intersects(item.bounds()) {
                        let item = item.clone();
                        if let Some(old) = item_stack(&self.mobs[index], Slot::MainHand.index()) {
                            self.spawn_item_own(old, at, 10);
                        }
                        self.mob_item_pickup_trigger(index, &item);
                        self.vulture_equip(index, item.stack.clone());
                        state_mut(&mut self.mobs[index]).since_eating = 1;
                        self.add_pickup(Taken::Item(Box::new(item)), Collector::Mob(id));
                        self.remove_dropped(i);
                        break;
                    }
                    i += 1;
                }
            }
            self.tick_vulture_eating(index, at);
        }
    }

    fn vulture_equip(&mut self, index: usize, stack: ItemStack) {
        let mob = &mut self.mobs[index];
        let slot = Slot::MainHand.index();
        set_item_stack(mob, slot, stack);
    }

    fn tick_vulture_eating(&mut self, index: usize, at: [f64; 3]) {
        let mob = &self.mobs[index];
        let Some(stack) = item_stack(mob, Slot::MainHand.index()) else {
            return;
        };
        let default_food = self.items().get(&stack.item).and_then(|item| item.food);
        if mob.no_ai || food(&stack, default_food).is_none() {
            return;
        }
        let since = state(mob).since_eating;
        if since > 600 {
            self.finish_vulture_meal(index, stack);
        } else if since > 560 && self.mobs[index].random.next_float() < 0.1 {
            let look = self.mobs[index].look_angle();
            let center = [at[0] + look[0] / 2.0, at[1], at[2] + look[2] / 2.0];
            self.audio
                .play("minecraft:entity.generic.eat", Some(at), 1.0, 1.0);
            let options = self.item_particle(&stack);
            self.particles
                .send_particles(&options, center, [0.05, 0.1, 0.0], 0.05, 8);
            self.lan_send_particles(super::lan_particles::QueuedParticle {
                dimension: self.dim.id(),
                options,
                state: None,
                at: center,
                spread: [0.05, 0.1, 0.0],
                speed: [0.05; 3],
                count: 8,
                randomization: 0,
            });
        }
    }

    fn finish_vulture_meal(&mut self, index: usize, stack: ItemStack) {
        use horizon_client::mobs::fox::naturalist_vulture_meal::finish;

        let mut mob = self.mobs[index].clone();
        let mut events = Vec::new();
        let remaining = finish(&self.mob_world_base(&[]), &mut mob, &mut events, stack);
        mob.extra.set_main_hand_stack(remaining);
        state_mut(&mut mob).since_eating = 0;
        self.mobs[index] = mob;
        if let Err(error) = self.handle_mob_events(events) {
            log::warn!("Vulture meal effects: {error}");
        }
    }
}
