use super::nav::Navigation;

pub const DESERT: &str = "naturalist:desert_scorpion";
pub const JUNGLE: &str = "naturalist:jungle_scorpion";
pub const PREY: &str = "naturalist:scorpion_hostiles";

pub fn from_hand(nav: &Navigation) -> bool {
    *nav.__horizon_mixins_naturalist_scorpion_from_hand()
}

pub fn set_from_hand(nav: &mut Navigation, from_hand: bool) {
    *nav.__horizon_mixins_naturalist_scorpion_from_hand_mut() = from_hand;
}

pub fn is_prey(nav: &Navigation) -> bool {
    *nav.__horizon_mixins_naturalist_scorpion_prey()
}

pub fn set_prey(nav: &mut Navigation, prey: bool) {
    *nav.__horizon_mixins_naturalist_scorpion_prey_mut() = prey;
}
