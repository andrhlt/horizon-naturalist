use super::{Navigation, super::Seen};
use horizon_core::geometry::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Seed {
    pub id: u32,
    pub position: Vec3,
}

#[derive(Clone, Debug, Default)]
pub struct Foraging {
    pub first: Option<Seed>,
    pub selected: Option<Seed>,
    pub target: Option<u32>,
    pub consume: Option<u32>,
}

pub fn foraging(nav: &mut Navigation) -> &mut Option<Foraging> {
    nav.__horizon_mixins_naturalist_bird_foraging_mut()
}

pub const BIRD_SNAPSHOT: u8 = 252;

pub fn bird_look(seen: &Seen) -> Option<[f64; 3]> {
    (seen.species == super::super::Species::Parrot && seen.variant == BIRD_SNAPSHOT).then_some(seen.stats)
}

pub fn section_order([x, y, z]: [i32; 3], order: u64) -> (i32, u32, u32, u64) {
    (x, z as u32 & 0x3F_FFFF, y as u32 & 0xF_FFFF, order)
}
