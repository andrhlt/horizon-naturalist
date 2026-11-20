use std::cell::Cell;

thread_local! {
    static THROW_POSITION: Cell<Option<[f64; 3]>> = const { Cell::new(None) };
}

pub struct DuckThrow;

impl DuckThrow {
    pub fn enter(position: [f64; 3]) -> Option<Self> {
        if THROW_POSITION.get().is_some() {
            return None;
        }
        THROW_POSITION.set(Some(position));
        Some(Self)
    }
}

impl Drop for DuckThrow {
    fn drop(&mut self) {
        THROW_POSITION.set(None);
    }
}

pub(super) fn position() -> Option<[f64; 3]> {
    THROW_POSITION.get()
}
