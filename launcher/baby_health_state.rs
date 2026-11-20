use super::Navigation;

pub const MODIFIER_ID: &str = "naturalist:baby_health";

pub fn last_age(nav: &mut Navigation) -> &mut Option<bool> {
    nav.__horizon_mixins_naturalist_baby_health_age_mut()
}

pub fn has_baby_modifier_lifecycle(nav: &Navigation) -> bool {
    *nav.__horizon_mixins_naturalist_baby_health_age() == Some(true)
}
