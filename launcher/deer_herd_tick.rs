fn deer_herd_tick(
    mob: &mut Mob,
    world: &MobWorld<'_>,
    random: &mut LegacyRandom,
    events: &mut Vec<MobEvent>,
) -> std::ops::ControlFlow<bool> {
    use std::ops::ControlFlow;
    if mob.kind.id != naturalist_deer_herd::DEER || mob.extra.client_side {
        return ControlFlow::Continue(());
    }
    naturalist_deer_herd::enter_turn(mob, world);
    mob.track_section();
    mob.combat.recheck(mob.alive(), mob.age);
    let live = neutral_anger::around_tick(mob, world.game_time, |mob| {
        ai::tick(mob, world, random, events)
    });
    naturalist_deer_herd::finish_tick(mob);
    ControlFlow::Break(live)
}
