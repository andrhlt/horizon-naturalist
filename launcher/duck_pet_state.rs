use super::Navigation;

pub fn is_following(navigation: &Navigation) -> bool {
    !*navigation.__horizon_mixins_naturalist_duck_not_following()
}

pub fn set_following(navigation: &mut Navigation, following: bool) {
    *navigation.__horizon_mixins_naturalist_duck_not_following_mut() = !following;
}
