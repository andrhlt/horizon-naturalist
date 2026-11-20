// The chunk spawn loop temporarily lends its owning LegacyRandom to its
// callbacks. The existing one-lane draw gate moves with the source and
// identifies that source without adding a field to every RNG.
use super::{LegacyRandom, OrderedDraws, ordered::Gate};
use std::{cell::Cell, ptr};

#[derive(Clone, Copy)]
struct Scope {
    owner: *const Gate,
    state: u64,
    replacement: bool,
}

thread_local! {
    static CURRENT: Cell<Option<Scope>> = const { Cell::new(None) };
}

pub fn open(random: &mut LegacyRandom) {
    if random.gate.is_some() || CURRENT.get().is_some() {
        return;
    }
    let mut scoped = OrderedDraws::new(random, 1).lane(0);
    scoped.sync();
    let owner = scoped.gate.as_deref().map(ptr::from_ref).expect("scoped RNG gate");
    CURRENT.set(Some(Scope { owner, state: scoped.state, replacement: false }));
    *random = scoped;
}

pub fn close(random: &mut LegacyRandom) {
    let Some(scope) = CURRENT.get() else { return };
    if random.gate.as_deref().is_none_or(|gate| ptr::from_ref(gate) != scope.owner) {
        return;
    }
    CURRENT.set(None);
    random.gate = None;
    if scope.replacement {
        random.state = scope.state;
    }
}

#[inline(always)]
pub fn before_draw(random: &mut LegacyRandom) {
    let Some(gate) = random.gate.as_deref() else { return };
    let Some(mut scope) = CURRENT.get() else { return };
    if scope.owner != ptr::from_ref(gate) {
        return;
    }
    if scope.replacement {
        random.state = scope.state;
    }
    scope.state = random.state
        .wrapping_mul(LegacyRandom::MULTIPLIER)
        .wrapping_add(11)
        & LegacyRandom::MASK;
    scope.replacement = false;
    CURRENT.set(Some(scope));
}

pub fn record_state(random: &LegacyRandom) {
    let Some(gate) = random.gate.as_deref() else { return };
    if let Some(mut scope) = CURRENT.get().filter(|scope| scope.owner == ptr::from_ref(gate)) {
        scope.state = random.state;
        scope.replacement = false;
        CURRENT.set(Some(scope));
    }
}

pub fn with_current<T>(draw: impl FnOnce(&mut LegacyRandom) -> T) -> Option<T> {
    let mut scope = CURRENT.get()?;
    let mut random = LegacyRandom::from_state(scope.state);
    let value = draw(&mut random);
    scope.state = random.state();
    scope.replacement = true;
    CURRENT.set(Some(scope));
    Some(value)
}
