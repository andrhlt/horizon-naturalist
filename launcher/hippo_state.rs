use super::Navigation;
use horizon_core::geometry::{Aabb, Vec3};
use std::sync::Arc;

pub const KIND: &str = "naturalist:hippo";

#[derive(Clone, Copy, Debug)]
pub struct BoatTarget {
    pub id: u32,
    pub position: Vec3,
    pub bounds: Aabb,
}

#[derive(Clone, Debug, Default)]
pub struct HippoState {
    pub eating: i32,
    pub boats: Arc<[BoatTarget]>,
}

pub fn state(nav: &Navigation) -> &HippoState {
    nav.__horizon_mixins_naturalist_hippo_state()
}

pub fn state_mut(nav: &mut Navigation) -> &mut HippoState {
    nav.__horizon_mixins_naturalist_hippo_state_mut()
}
