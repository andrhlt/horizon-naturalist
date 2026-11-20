//! Bass-specific avoidance delegates navigation and retreat selection to Horizon's vanilla goal.
use horizon_client::mobs::{
    Mob, Seen, Species,
    goals::{Avoid, AvoidEntity, Cx, Goal},
};

#[derive(Clone, Debug)]
pub(crate) struct BassAvoid {
    inner: AvoidEntity,
    catfish: bool,
}
impl BassAvoid {
    pub(crate) fn larger_bass() -> Self {
        Self {
            inner: AvoidEntity::new(Avoid::Species(vec![Species::Cod]), 10.0, 1.3, 1.7),
            catfish: false,
        }
    }
    pub(crate) fn catfish() -> Self {
        Self {
            inner: AvoidEntity::new(Avoid::Species(vec![Species::Cod]), 8.0, 1.0, 1.5),
            catfish: true,
        }
    }
}
fn catfish(seen: &Seen) -> bool {
    seen.kind.id == "naturalist:catfish"
}
fn medium_or_large(seen: &Seen) -> bool {
    seen.kind.id == "naturalist:bass" && seen.variant > 0
}
fn large(seen: &Seen) -> bool {
    seen.kind.id == "naturalist:bass" && seen.variant > 1
}
fn none(_: &Seen) -> bool {
    false
}

impl Goal for BassAvoid {
    fn name(&self) -> &'static str {
        if self.catfish {
            "NaturalistBassAvoidCatfish"
        } else {
            "NaturalistBassAvoidLarger"
        }
    }
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn flags(&self) -> u8 {
        self.inner.flags()
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        self.inner.seen_when = Some(if self.catfish {
            catfish
        } else {
            match mob.extra.variant {
                0 => medium_or_large,
                1 => large,
                _ => none,
            }
        });
        self.inner.can_use(mob, cx)
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        self.inner.can_continue(mob, cx)
    }
    fn interruptable(&self) -> bool {
        self.inner.interruptable()
    }
    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.inner.start(mob, cx);
    }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.inner.stop(mob, cx);
    }
    fn every_tick(&self) -> bool {
        self.inner.every_tick()
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.inner.tick(mob, cx);
    }
}
