use super::Game;
use horizon_client::mobs::{
    Mob, MobWorld,
    goals::{self, Cx, Goal, JUMP, LOOK, MOVE, MoveToBlock, reduced},
};
use horizon_core::{nbt::Tag, random::RandomSource};
use mod_api::Holder;
use naturalist::ant_worker::{STATE, Shared};

pub(super) fn install(game: &mut Game<'_>, id: u32) {
    let Ok(shared) = STATE.get(game, Holder::Entity(id)) else {
        return;
    };
    if let Some(ant) = game.mobs.iter_mut().find(|m| m.id == id) {
        if !ant.ai.goals.has("NaturalistAntStore") {
            let kind = std::sync::Arc::make_mut(&mut ant.kind);
            kind.flags.climber = true;
            kind.food = Default::default();
            ant.ai.nav.kind = horizon_client::mobs::nav::NavKind::WallClimber;
            let goals = &mut ant.ai.goals;
            goals.clear();
            goals.add(0, goals::Float);
            goals.add(1, goals::MeleeAttack::new(1.15, true));
            goals.add(
                2,
                Store {
                    shared: shared.clone(),
                    walk: MoveToBlock::new(1.0, 16, 5),
                    store: true,
                },
            );
            goals.add(
                3,
                Gather {
                    shared: shared.clone(),
                    target: None,
                },
            );
            goals.add(
                4,
                Store {
                    shared,
                    walk: MoveToBlock::new(1.0, 16, 5),
                    store: false,
                },
            );
            goals.add(5, goals::Stroll::new(1.0));
            goals.add(6, goals::LookAtPlayer::new(6.0));
            goals.add(7, goals::RandomLookAround::default());
        }
    }
}
#[derive(Clone, Debug)]
struct Store {
    shared: Shared,
    walk: MoveToBlock,
    store: bool,
}
impl Store {
    fn eligible(&self, ant: &Mob) -> bool {
        ant.ai.target.is_none()
            && self
                .shared
                .lock()
                .ok()
                .is_some_and(|state| state.carried == self.store)
    }
    fn valid(world: &MobWorld, [x, y, z]: [i32; 3], owner: Option<[i32; 4]>, store: bool) -> bool {
        let Some(row) = world.registry.state(world.world.block(x, y, z)) else {
            return false;
        };
        if row.name != "naturalist:ant_hill"
            || row.properties.get("open").map(String::as_str) != Some("true")
            || (!store && row.properties.get("workers").map(String::as_str) == Some("3"))
        {
            return false;
        }
        let Some(horizon_client::chunk::block_entity::BlockEntity::Mod(hill)) =
            world.world.block_entities.get(&(x, y, z))
        else {
            return false;
        };
        let hill_owner = match hill.nbt.get(&"Owner".into()) {
            Some(Tag::IntArray(id)) => <[i32; 4]>::try_from(id.as_slice()).ok(),
            _ => None,
        };
        owner == hill_owner
    }
}
impl Goal for Store {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn name(&self) -> &'static str {
        if self.store {
            "NaturalistAntStore"
        } else {
            "NaturalistAntEnter"
        }
    }
    fn flags(&self) -> u8 {
        MOVE | JUMP
    }
    fn can_use(&mut self, ant: &mut Mob, cx: &mut Cx) -> bool {
        if !self.eligible(ant)
            || (!self.store
                && self
                    .shared
                    .lock()
                    .ok()
                    .is_some_and(|state| state.cooldown > 0))
        {
            return false;
        }
        if self.walk.next_start > 0 {
            self.walk.next_start -= 1;
            return false;
        }
        self.walk.next_start = reduced(200 + ant.random.next_int_bound(200).unwrap_or(0));
        let owner = ant.extra.owner;
        self.walk.find_nearest(ant, cx.w, |world, at| {
            Self::valid(world, at, owner, self.store)
        })
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
        // Horizon's reach=1 path may end at the adjacent node, outside
        // MoveToBlockGoal's own accepted distance. Ask for the actual target
        // node when that path ends so the ant can finish the same approach.
        if !self.walk.reached && ant.ai.nav.is_done() {
            let b = self.walk.block;
            let path = ant.create_path_to(cx.w, [b[0], b[1] + 1, b[2]], 0);
            ant.nav_move_to_path(cx.w, path, 1.0);
        }
        if self.walk.reached
            && let Ok(mut state) = self.shared.lock()
        {
            if self.store {
                state.store = Some(self.walk.block);
            } else {
                state.enter = Some(self.walk.block);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Gather {
    shared: Shared,
    target: Option<u32>,
}
impl Goal for Gather {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn name(&self) -> &'static str {
        "NaturalistAntGather"
    }
    fn flags(&self) -> u8 {
        MOVE | LOOK
    }
    fn can_use(&mut self, ant: &mut Mob, _: &mut Cx) -> bool {
        let Ok(state) = self.shared.lock() else {
            return false;
        };
        if ant.age % 10 != 0 || ant.ai.target.is_some() || state.carried {
            return false;
        }
        self.target = state.food.map(|food| food.id);
        self.target.is_some()
    }
    fn can_continue(&mut self, ant: &mut Mob, _: &mut Cx) -> bool {
        ant.ai.target.is_none()
            && self.shared.lock().ok().is_some_and(|state| {
                !state.carried && state.food.is_some_and(|food| Some(food.id) == self.target)
            })
    }
    fn start(&mut self, ant: &mut Mob, cx: &mut Cx) {
        if let Ok(mut state) = self.shared.lock() {
            state.pursuing = self.target;
            if let Some(food) = state.food {
                ant.nav_move_to(cx.w, food.at, 1.15);
            }
        }
    }
    fn stop(&mut self, ant: &mut Mob, _: &mut Cx) {
        if let Ok(mut state) = self.shared.lock() {
            state.pursuing = None;
        }
        self.target = None;
        ant.nav_stop();
    }
    fn tick(&mut self, ant: &mut Mob, cx: &mut Cx) {
        let Ok(mut state) = self.shared.lock() else {
            return;
        };
        let Some(food) = state.food else { return };
        if Some(food.id) != self.target {
            return;
        }
        ant.look_at_speed(food.at, 30.0, 30.0);
        if ant.ai.nav.is_done() {
            ant.nav_move_to(cx.w, food.at, 1.15);
        }
        if ant.distance_sqr(horizon_core::geometry::Vec3::new(
            food.at[0], food.at[1], food.at[2],
        )) < 1.0
        {
            state.pick_up = Some(food.id);
        }
    }
}
