use std::cell::RefCell;

thread_local! {
    pub(super) static BEFORE: RefCell<Vec<(u32, f64)>> = const { RefCell::new(Vec::new()) };
}
