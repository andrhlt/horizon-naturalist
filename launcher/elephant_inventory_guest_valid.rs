fn elephant_inventory_guest_valid(
    game: &Game<'_>,
    guest: &horizon_client::lan::guest::Guest,
    id: u32,
) -> std::ops::ControlFlow<bool> {
    use horizon_client::{lan::playerdata::uuid_ints, mobs::nav::naturalist_elephants};
    let Some(mob) = game.mobs.iter().find(|mob| mob.id == id
        && naturalist_elephants::includes(&mob.kind.id)) else {
        return std::ops::ControlFlow::Continue(());
    };
    let [x, y, z] = guest.position;
    let distance = (mob.position.x - x).powi(2)
        + (mob.position.y - y).powi(2)
        + (mob.position.z - z).powi(2);
    std::ops::ControlFlow::Break(
        mob.alive() && mob.extra.tame && mob.extra.owner == Some(uuid_ints(guest.uuid))
            && distance < 64.0,
    )
}
