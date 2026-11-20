use std::{
    collections::BTreeSet,
    sync::{RwLock, atomic::{AtomicBool, Ordering}},
};

static GLOW_GOOP_TOOLTIP: AtomicBool = AtomicBool::new(true);

pub fn set_glow_goop_tooltip(enabled: bool) {
    GLOW_GOOP_TOOLTIP.store(enabled, Ordering::Relaxed);
}

pub fn glow_goop_tooltip() -> bool {
    GLOW_GOOP_TOOLTIP.load(Ordering::Relaxed)
}

static SNAIL_CRUSHING: AtomicBool = AtomicBool::new(false);
static REMOVED: RwLock<BTreeSet<String>> = RwLock::new(BTreeSet::new());

pub fn set_removed(removed: BTreeSet<String>) {
    *REMOVED.write().unwrap_or_else(|error| error.into_inner()) = removed;
}

pub fn removed(entity: &str) -> bool {
    let Some(name) = entity.strip_prefix("naturalist:") else { return false };
    let key = if name == "caterpillar" { "butterfly" } else { name };
    REMOVED.read().unwrap_or_else(|error| error.into_inner()).contains(key)
}

pub fn set_snail_crushing(enabled: bool) {
    SNAIL_CRUSHING.store(enabled, Ordering::Relaxed);
}

pub fn snail_crushing() -> bool {
    SNAIL_CRUSHING.load(Ordering::Relaxed)
}
