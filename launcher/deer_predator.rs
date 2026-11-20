use super::Navigation;

pub const TAG: &str = "naturalist:deer_predators";

pub fn set(navigation: &mut Navigation, predator: bool) {
    *navigation.__horizon_mixins_naturalist_deer_predator_mut() = predator;
}

pub fn is_predator(navigation: &Navigation) -> bool {
    *navigation.__horizon_mixins_naturalist_deer_predator()
}

pub const TURKEY_HOSTILES: &str = "naturalist:turkey_hostiles";

pub fn set_turkey_hostile(navigation: &mut Navigation, hostile: bool) {
    *navigation.__horizon_mixins_naturalist_turkey_hostile_mut() = hostile;
}

pub fn is_turkey_hostile(navigation: &Navigation) -> bool {
    *navigation.__horizon_mixins_naturalist_turkey_hostile()
}
