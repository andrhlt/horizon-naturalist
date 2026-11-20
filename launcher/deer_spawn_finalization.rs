use super::{
    Mob,
    finalize::{Attr, Op, add_modifier, has_modifier},
};
use horizon_core::random::RandomSource;

const MODIFIER: &str = "minecraft:random_spawn_bonus";

pub fn finished(mob: &Mob) -> bool {
    has_modifier(mob, Attr::FollowRange, MODIFIER)
}

pub fn finish(mob: &mut Mob, random: &mut impl RandomSource) {
    if !finished(mob) {
        apply_bonus(mob, random);
    }
    mob.extra.left_handed = random.next_float() < 0.05;
}

pub fn finish_spawner(mob: &mut Mob, random: &mut impl RandomSource) {
    apply_bonus(mob, random);
    mob.extra.left_handed = random.next_float() < 0.05;
}

fn apply_bonus(mob: &mut Mob, random: &mut impl RandomSource) {
    // Java 26.3 rounds each nextDouble result to f32 before widening it.
    // Keep the two original draws and the double-precision subtraction.
    let first = f64::from(random.next_double() as f32);
    let second = f64::from(random.next_double() as f32);
    let bonus = 0.114_850_000_000_000_01 * (first - second);
    add_modifier(mob, Attr::FollowRange, MODIFIER, bonus, Op::AddMultipliedBase);
}

use horizon_core::{
    position::BlockPos,
    random::{InvalidBound, LevelPositional, LevelRandom, PositionalRandom},
};
use std::{cell::RefCell, rc::Rc};

/// The spawner's placement and its entity callback share the level stream.
#[derive(Clone)]
pub struct SpawnerRandom(Rc<RefCell<LevelRandom>>);

impl SpawnerRandom {
    pub fn new(random: LevelRandom) -> Self {
        Self(Rc::new(RefCell::new(random)))
    }

    pub fn level_random(&self) -> LevelRandom {
        self.0.borrow().clone()
    }
}

pub struct SpawnerPositional(LevelPositional);

impl PositionalRandom for SpawnerPositional {
    type Source = SpawnerRandom;

    fn at(&self, position: BlockPos) -> SpawnerRandom {
        SpawnerRandom::new(self.0.at(position))
    }

    fn from_seed(&self, seed: i64) -> SpawnerRandom {
        SpawnerRandom::new(self.0.from_seed(seed))
    }

    fn from_hash_of(&self, name: &str) -> SpawnerRandom {
        SpawnerRandom::new(self.0.from_hash_of(name))
    }

    fn from_hash_of_utf16(&self, name: &[u16]) -> SpawnerRandom {
        SpawnerRandom::new(self.0.from_hash_of_utf16(name))
    }
}

impl RandomSource for SpawnerRandom {
    type Positional = SpawnerPositional;

    fn set_seed(&mut self, seed: i64) {
        self.0.borrow_mut().set_seed(seed);
    }

    fn next_int(&mut self) -> i32 {
        self.0.borrow_mut().next_int()
    }

    fn next_int_bound(&mut self, bound: i32) -> Result<i32, InvalidBound> {
        self.0.borrow_mut().next_int_bound(bound)
    }

    fn next_long(&mut self) -> i64 {
        self.0.borrow_mut().next_long()
    }

    fn next_boolean(&mut self) -> bool {
        self.0.borrow_mut().next_boolean()
    }

    fn next_float(&mut self) -> f32 {
        self.0.borrow_mut().next_float()
    }

    fn next_double(&mut self) -> f64 {
        f64::from(self.0.borrow_mut().next_double() as f32)
    }

    fn next_gaussian(&mut self) -> f64 {
        self.0.borrow_mut().next_gaussian()
    }

    fn fork_positional(&mut self) -> SpawnerPositional {
        SpawnerPositional(self.0.borrow_mut().fork_positional())
    }

    fn fork(&mut self) -> Self {
        Self::new(self.0.borrow_mut().fork())
    }
}
