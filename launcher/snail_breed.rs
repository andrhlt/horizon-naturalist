fn snail_breed(game: &mut Game<'_>, parent: u32, partner: u32) -> std::ops::ControlFlow<()> {
    use horizon_client::mobs::naturalist_snail::{KIND, snail_mut};
    use std::ops::ControlFlow;
    let Some(index) = game
        .mobs
        .iter()
        .position(|mob| mob.id == parent && mob.kind.id == KIND)
    else {
        return ControlFlow::Continue(());
    };
    let Some(other) = game
        .mobs
        .iter()
        .position(|mob| mob.id == partner && mob.kind.id == KIND)
    else {
        return ControlFlow::Break(());
    };
    if let Some(cause) = breeding_credit(&mut game.love_cause, parent, partner) {
        let first = game.mob_entity(&game.mobs[index]);
        let second = game.mob_entity(&game.mobs[other]);
        let who = cause.map_or(
            super::commands::Target::Player,
            super::commands::Target::Guest,
        );
        game.for_player(who, |game| {
            game.award_custom_stat("animals_bred", 1);
            game.trigger_bred(&first, &second, None);
        });
    }
    let mob = &mut game.mobs[index];
    snail_mut(mob).has_egg = true;
    mob.extra.age = 0;
    mob.extra.in_love = 0;
    let p = mob.position;
    if game.ui.rules.bool("mob_drops") {
        let amount = 1 + mob.random.next_int_bound(7).unwrap_or(0) as u32;
        game.add_orb([p.x, p.y, p.z], amount);
    }
    // The shared Breed event already applied the live-birth cooldown to
    // the partner. Java's egg-laying goal only clears love on each adult.
    let partner = &mut game.mobs[other];
    partner.extra.age = 0;
    partner.extra.in_love = 0;
    ControlFlow::Break(())
}
