use super::super::naturalist_ant::{Action, FoodTarget, HILL, state, state_mut};
use super::{Cx, Goal, JUMP, LOOK, MOVE, Mob, MobWorld, MoveToBlock, goal_clone, reduced};
use horizon_core::random::RandomSource;
use horizon_world::nbt::Tag;

#[derive(Clone, Debug, Default)]
pub struct Gather;

impl Goal for Gather {
    goal_clone!();

    fn flags(&self) -> u8 {
        MOVE | LOOK
    }

    fn can_use(&mut self, ant: &mut Mob, _: &mut Cx) -> bool {
        ant.age % 10 == 0
            && ant.ai.target.is_none()
            && state(ant).meal.is_none()
            && state(ant).food.is_some()
    }

    fn can_continue(&mut self, ant: &mut Mob, _: &mut Cx) -> bool {
        ant.ai.target.is_none()
            && state(ant).meal.is_none()
            && state(ant).pursuing_food.is_some()
            && state(ant).food.is_some()
    }

    fn start(&mut self, ant: &mut Mob, cx: &mut Cx) {
        if let Some(food) = state(ant).food {
            state_mut(ant).pursuing_food = Some(food.id);
            ant.nav_move_to(cx.w, food.at, 1.15);
        }
    }

    fn stop(&mut self, ant: &mut Mob, _: &mut Cx) {
        state_mut(ant).pursuing_food = None;
        ant.nav_stop();
    }

    fn tick(&mut self, ant: &mut Mob, cx: &mut Cx) {
        let Some(FoodTarget { id, at }) = state(ant).food else {
            return;
        };
        ant.look_at_speed(at, 30.0, 30.0);
        if ant.ai.nav.is_done() {
            ant.nav_move_to(cx.w, at, 1.15);
        }
        if ant.distance_sqr(horizon_core::geometry::Vec3::new(at[0], at[1], at[2])) < 1.0 {
            state_mut(ant).action = Some(Action::PickUp(id));
        }
    }
}

#[derive(Clone, Debug)]
pub struct ForageHill {
    walk: MoveToBlock,
    store: bool,
}

impl ForageHill {
    pub fn new(store: bool) -> Self {
        Self {
            walk: MoveToBlock::new(1.0, 16, 5),
            store,
        }
    }

    fn eligible(&self, ant: &Mob) -> bool {
        ant.ai.target.is_none() && self.store == state(ant).meal.is_some()
    }

    fn valid(world: &MobWorld, [x, y, z]: [i32; 3], owner: Option<[i32; 4]>, store: bool) -> bool {
        use super::super::super::containers::BlockEntity;
        let Some(row) = world.registry.state(world.world.block(x, y, z)) else {
            return false;
        };
        if row.name != HILL
            || row.properties.get("open").map(String::as_str) != Some("true")
            || (!store && row.properties.get("workers").map(String::as_str) == Some("3"))
        {
            return false;
        }
        let Some(BlockEntity::Mod(hill)) = world.world.block_entities.get(&(x, y, z)) else {
            return false;
        };
        let hill_owner = match hill.nbt.get(&"Owner".into()) {
            Some(Tag::IntArray(owner)) => owner.as_slice().try_into().ok(),
            _ => None,
        };
        owner == hill_owner
    }
}

impl Goal for ForageHill {
    goal_clone!();

    fn flags(&self) -> u8 {
        MOVE | JUMP
    }

    fn can_use(&mut self, ant: &mut Mob, cx: &mut Cx) -> bool {
        if !self.eligible(ant) || (!self.store && state(ant).cooldown > 0) {
            return false;
        }
        if self.walk.next_start > 0 {
            self.walk.next_start -= 1;
            return false;
        }
        self.walk.next_start = reduced(200 + ant.random.next_int_bound(200).unwrap_or(0));
        let owner = ant.extra.owner;
        let store = self.store;
        self.walk
            .find_nearest(ant, cx.w, |world, at| Self::valid(world, at, owner, store))
    }

    fn can_continue(&mut self, ant: &mut Mob, cx: &mut Cx) -> bool {
        self.eligible(ant)
            && self.walk.can_continue(cx.w, |world, at| {
                Self::valid(world, at, ant.extra.owner, self.store)
            })
    }

    fn start(&mut self, ant: &mut Mob, cx: &mut Cx) {
        self.walk.start(ant, cx.w);
    }

    fn tick(&mut self, ant: &mut Mob, cx: &mut Cx) {
        self.walk.tick(ant, cx.w);
        if self.walk.reached {
            state_mut(ant).action = Some(if self.store {
                Action::Store(self.walk.block)
            } else {
                Action::Enter(self.walk.block)
            });
        }
    }
}
