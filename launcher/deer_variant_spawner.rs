fn deer_variant_spawner(ids: HashMap<u32, u32>, game: &mut Game<'_>) -> HashMap<u32, u32> {
    use super::naturalist_deer_placement::{Pass, PlacementPass};
    let ordinary = PlacementPass::active(Pass::Spawner);
    for id in ids.values() {
        if let Some(index) = game.mobs.iter().position(|mob| mob.id == *id) {
            if ordinary && game.mobs[index].kind.id == "naturalist:deer" {
                continue;
            }
            super::naturalist_deer_variants::assign_variant_at(game, index);
        }
    }
    ids
}
