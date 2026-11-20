use std::cell::Cell;

thread_local! {
    static LIVE_MELEE: Cell<bool> = const { Cell::new(false) };
}

pub(super) struct Scope(bool);

impl Scope {
    pub(super) fn enter() -> Self {
        Self(LIVE_MELEE.replace(true))
    }
}

impl Drop for Scope {
    fn drop(&mut self) {
        LIVE_MELEE.set(self.0);
    }
}

pub(super) fn take() -> bool {
    LIVE_MELEE.replace(false)
}
