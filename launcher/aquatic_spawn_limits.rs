use std::cell::Cell;

#[derive(Clone, Copy, Default)]
enum Cluster {
    #[default]
    Vanilla,
    Bass,
    Piranha,
    Four,
}

thread_local! {
    static NATURAL_CLUSTER: Cell<Option<Cluster>> = const { Cell::new(None) };
}

struct SpawnPass(Option<Cluster>);

impl Drop for SpawnPass {
    fn drop(&mut self) {
        NATURAL_CLUSTER.set(self.0);
    }
}

pub fn natural_pass<T>(spawn: impl FnOnce() -> T) -> T {
    let _pass = SpawnPass(NATURAL_CLUSTER.replace(Some(Cluster::Vanilla)));
    spawn()
}

// The spawner resolves a group's type immediately before using its species-level cap.
pub fn select(entity: &str) {
    if NATURAL_CLUSTER.get().is_some() {
        let cluster = match entity {
            "naturalist:bass" => Cluster::Bass,
            "naturalist:piranha" => Cluster::Piranha,
            "naturalist:starfish" | "naturalist:giant_isopod" | "naturalist:clam" | "naturalist:whale" | "naturalist:great_white_shark" => Cluster::Four,
            _ => Cluster::Vanilla,
        };
        NATURAL_CLUSTER.set(Some(cluster));
    }
}

pub(super) fn limit() -> Option<u32> {
    match NATURAL_CLUSTER.get() {
        Some(Cluster::Bass) => Some(6),
        Some(Cluster::Piranha) => Some(24),
        Some(Cluster::Four) => Some(4),
        _ => None,
    }
}

pub fn is_bass() -> bool {
    matches!(NATURAL_CLUSTER.get(), Some(Cluster::Bass))
}
