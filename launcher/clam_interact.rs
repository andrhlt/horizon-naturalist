fn clam_interact(game: &mut Game<'_>, target: Option<InteractTarget>) -> std::ops::ControlFlow<Result<bool>> {
    use horizon_client::mobs::nav::naturalist_clam as clam;
    use std::ops::ControlFlow;

    let Some(index) = game.mob_interaction_target(target) else {
        return ControlFlow::Continue(());
    };
    let held = game.held().cloned();
    let item = held.as_ref().map(|stack| stack.item.as_ref());
    let mob = &mut game.mobs[index];
    if mob.kind.id != clam::KIND || !mob.alive() {
        return ControlFlow::Continue(());
    }
    if item == Some("minecraft:lead")
        || item == Some("minecraft:name_tag") && held.as_ref().is_some_and(|stack| stack.custom_name().is_some())
        || mob.extra.leash == Some(horizon_client::mobs::leash::Holder::Player)
    {
        return ControlFlow::Continue(());
    }
    if game.creative && item == Some("minecraft:debug_stick") {
        let shell = clam::state_mut(&mut mob.ai.nav);
        shell.open = true;
        shell.state_timer = 200;
        shell.snap_cooldown = 200;
    } else {
        let shell = clam::state(&mob.ai.nav);
        if !shell.open {
            return if item == Some("minecraft:water_bucket") {
                ControlFlow::Break(Ok(false))
            } else {
                ControlFlow::Continue(())
            };
        }
        if shell.has_treasure {
            let Some(stored) = mob.extra.main_hand_stack().cloned() else {
                return ControlFlow::Continue(());
            };
            mob.extra.set_main_hand_stack(None);
            clam::state_mut(&mut mob.ai.nav).has_treasure = false;
            clam::set_open(mob, false);
            mob.needs_sync = true;
            game.give_or_drop(stored);
            game.swing_arm();
            return ControlFlow::Break(Ok(true));
        }
        let Some(mut stored) = held else {
            return ControlFlow::Continue(());
        };
        stored.count = 1;
        mob.extra.set_main_hand_stack(Some(stored));
        clam::state_mut(&mut mob.ai.nav).has_treasure = true;
        mob.needs_sync = true;
        if !game.creative {
            let held = game.held_mut();
            if let Some(stack) = held {
                stack.count -= 1;
                if stack.count == 0 { *held = None; }
            }
        }
    }
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
