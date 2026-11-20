use super::Navigation;
use crate::mobs::naturalist_deer_herd::Turn;

pub fn turn(navigation: &mut Navigation) -> &mut Option<Turn> {
    navigation.__horizon_mixins_naturalist_deer_herd_turn_mut()
}
