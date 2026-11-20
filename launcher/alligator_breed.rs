fn alligator_breed(game: &mut Game<'_>, parent: u32, partner: u32) -> std::ops::ControlFlow<()> {
    use horizon_client::mobs::naturalist_ostrich as ostrich;
    use std::ops::ControlFlow;
    let Some(index) = game
        .mobs
        .iter()
        .position(|m| m.id == parent && (m.kind.id == "naturalist:alligator"
            || m.kind.id == "naturalist:tortoise" && !m.extra.flag
            || m.kind.id == ostrich::KIND && !ostrich::state(m).has_egg))
    else {
        return ControlFlow::Continue(());
    };
    let Some(other) = game
        .mobs
        .iter()
        .position(|m| m.id == partner && m.kind.id == game.mobs[index].kind.id)
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
    if mob.kind.id == ostrich::KIND {
        ostrich::state_mut(mob).has_egg = true;
    } else {
        mob.extra.flag = true;
    }
    mob.extra.age = 0;
    mob.extra.in_love = 0;
    let p = mob.position.components();
    if game.ui.rules.bool("mob_drops") {
        let amount = 1 + mob.random.next_int_bound(7).unwrap_or(0) as u32;
        game.add_orb(p, amount);
    }
    // The stock Breed event pre-applies a live-birth cooldown to the partner.
    // Java's egg-laying goal only clears love on each parent.
    game.mobs[other].extra.age = 0;
    game.mobs[other].extra.in_love = 0;
    ControlFlow::Break(())
}
