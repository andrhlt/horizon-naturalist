use super::{Direction, PlacementContext};

impl PlacementContext {
    pub fn naturalist_placement_directions(&self) -> [Direction; 6] {
        self.nearest_looking_directions()
    }
}
