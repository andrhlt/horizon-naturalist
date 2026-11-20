use super::Navigation;

pub const CONVERSION_TICKS: i32 = 100;
pub const DEEP_OFFSET: i32 = 33;
pub const SNAPSHOT: u8 = 245;
pub const SNAIL_PREY: u8 = 246;
pub const CRAB_PREY: u8 = 247;

pub fn gray(nav: &Navigation) -> bool {
    *nav.__horizon_mixins_naturalist_blobfish_gray()
}

pub fn set_gray(nav: &mut Navigation, gray: bool) {
    *nav.__horizon_mixins_naturalist_blobfish_gray_mut() = gray;
}

pub fn conversion(nav: &Navigation) -> i32 {
    *nav.__horizon_mixins_naturalist_blobfish_conversion()
}

pub fn set_conversion(nav: &mut Navigation, ticks: i32) {
    *nav.__horizon_mixins_naturalist_blobfish_conversion_mut() = ticks;
}
