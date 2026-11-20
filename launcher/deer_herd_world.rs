use super::Game;
use horizon_client::mobs::naturalist_deer_herd::{DEER, Round, prepare_turn, take_roll, take_turn_hits};

pub(super) fn begin_turns(game: &mut Game<'_>) {
    let mut deer = game.mobs.iter_mut().filter(|mob| !mob.removed && mob.kind.id == DEER);
    let (Some(first), Some(second)) = (deer.next(), deer.next()) else {
        return;
    };
    let round = Round::default();
    for mob in [first, second].into_iter().chain(deer) {
        prepare_turn(mob, round.clone());
    }
}

pub(super) fn flush_hit(game: &mut Game<'_>, id: u32) {
    let Some(hit) = game
        .mobs
        .iter_mut()
        .find(|mob| mob.id == id)
        .and_then(take_roll)
    else {
        return;
    };
    for mob in &mut game.mobs {
        hit.share(mob);
    }
}

pub(super) fn flush_pending(game: &mut Game<'_>) {
    let pending: Vec<_> = game.mobs.iter_mut().filter_map(take_roll).collect();
    for hit in pending {
        for mob in &mut game.mobs {
            hit.share(mob);
        }
    }
}

pub(super) fn flush_mob_turns(game: &mut Game<'_>) {
    let pending: Vec<_> = game
        .mobs
        .iter_mut()
        .enumerate()
        .flat_map(|(index, mob)| take_turn_hits(mob).into_iter().map(move |hit| (index, hit)))
        .collect();
    for (source_index, hit) in pending {
        for (recipient_index, mob) in game.mobs.iter_mut().enumerate() {
            // Later deer saw the hit before starting their turn. Apply it now
            // only to deer that had already ticked or stayed outside the batch.
            if recipient_index < source_index || !game.creatures.ticked.contains(&mob.id) {
                hit.share(mob);
            }
        }
    }
}
