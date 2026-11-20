use super::{
    Mob,
    goals::{
        Breed, Cx, Float, Goal, LookAtPlayer, Panic, RandomLookAround, Stroll, Tempt, reduced,
    },
};
use horizon_core::geometry::Vec3;
use std::sync::Arc;

pub const KIND: &str = "naturalist:giraffe";

pub fn register(mob: &mut Mob) {
    let food: Arc<[String]> = Arc::from([
        "minecraft:apple".to_owned(),
        "minecraft:golden_apple".to_owned(),
        "minecraft:hay_block".to_owned(),
    ]);
    let goals = &mut mob.ai.goals;
    goals.clear();
    goals.add(0, Float);
    goals.add(1, Panic::new(1.4));
    goals.add(2, Breed::new(1.0));
    goals.add(3, Tempt::new(1.0, food, false));
    goals.add(4, DistancedFollowParent::new(KIND, 1.0, 16.0, 8.0, 5.0));
    goals.add(5, Stroll::avoiding_water(0.7));
    goals.add(6, LookAtPlayer::new(6.0));
    goals.add(7, RandomLookAround::default());
}

#[derive(Clone, Debug)]
pub(super) struct DistancedFollowParent {
    kind: &'static str,
    speed: f64,
    scan: Vec3,
    minimum_sqr: f64,
    parent: Option<u32>,
    recalc: i32,
}

impl DistancedFollowParent {
    pub(super) fn new(kind: &'static str, speed: f64, horizontal: f64, vertical: f64, minimum: f64) -> Self {
        Self { kind, speed, scan: Vec3::new(horizontal, vertical, horizontal),
            minimum_sqr: minimum * minimum, parent: None, recalc: 0 }
    }
}

impl Goal for DistancedFollowParent {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn flags(&self) -> u8 {
        0
    }

    fn can_use(&mut self, mob: &mut Mob, context: &mut Cx) -> bool {
        use super::nav::naturalist_elephants::{ELEPHANT, MAMMOTH};

        if !mob.is_baby() || mob.extra.tame {
            return false;
        }
        let area = mob.bounds().inflate(self.scan);
        let mut best: Option<(u32, f64)> = None;
        for adult in context.w.others_near(area).filter(|adult| {
            adult.alive
                && !adult.baby
                // Mammoth extends Elephant in Java's class-based parent lookup.
                && (adult.effects.contains(&self.kind)
                    || self.kind == ELEPHANT && adult.effects.contains(&MAMMOTH))
                && area.intersects(adult.bounds)
        }) {
            let distance = mob.distance_sqr(adult.position);
            if best.is_none_or(|(_, nearest)| distance <= nearest) {
                best = Some((adult.id, distance));
            }
        }
        self.parent = best
            .filter(|(_, distance)| *distance >= self.minimum_sqr)
            .map(|(id, _)| id);
        self.parent.is_some()
    }

    fn can_continue(&mut self, mob: &mut Mob, context: &mut Cx) -> bool {
        mob.is_baby()
            && !mob.extra.tame
            && self
                .parent
                .and_then(|id| context.w.seen(id))
                .is_some_and(|adult| {
                    adult.alive && (self.minimum_sqr..=256.0).contains(&mob.distance_sqr(adult.position))
                })
    }

    fn start(&mut self, _: &mut Mob, _: &mut Cx) {
        self.recalc = 0;
    }

    fn stop(&mut self, _: &mut Mob, _: &mut Cx) {
        self.parent = None;
    }

    fn tick(&mut self, mob: &mut Mob, context: &mut Cx) {
        self.recalc -= 1;
        if self.recalc <= 0
            && let Some(adult) = self.parent.and_then(|id| context.w.seen(id))
        {
            self.recalc = reduced(10);
            mob.nav_move_to_entity(context.w, adult.position.components(), 1, self.speed);
        }
    }
}
