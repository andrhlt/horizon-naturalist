//! Ant foraging shares a short-lived snapshot with the goal while the mob is lent out.
use mod_api::AttachmentType;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug)]
pub struct Food {
    pub id: u32,
    pub at: [f64; 3],
}
#[derive(Default, Debug)]
pub struct Snapshot {
    pub carried: bool,
    pub cooldown: i32,
    pub food: Option<Food>,
    pub pursuing: Option<u32>,
    pub store: Option<[i32; 3]>,
    pub enter: Option<[i32; 3]>,
    pub pick_up: Option<u32>,
}
pub type Shared = Arc<Mutex<Snapshot>>;
pub static STATE: AttachmentType<Shared> = AttachmentType {
    namespace: "naturalist",
    name: "ant_worker_runtime",
    default: Shared::default,
    codec: None,
    copy_on_death: false,
    sync: false,
};
