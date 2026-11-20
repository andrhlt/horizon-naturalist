use super::Navigation;

pub const FOOD: &str = "naturalist:frog_food";
pub const FIREFLY: &str = "naturalist:firefly";

pub fn set(nav: &mut Navigation, food: bool) {
    *nav.__horizon_mixins_naturalist_frog_food_mut() = food;
}

pub fn includes(nav: &Navigation) -> bool {
    *nav.__horizon_mixins_naturalist_frog_food()
}
