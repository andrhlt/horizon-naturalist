fn zebra_angry_sound(mob: &mut Mob, events: &mut Vec<MobEvent>) -> std::ops::ControlFlow<()> {
    if mob.kind.id != super::naturalist_zebra::KIND {
        return std::ops::ControlFlow::Continue(());
    }
    if mob.extra.counter <= 0 {
        stand_if_possible(mob);
        mob.voice("naturalist:entity.zebra.angry", events);
    }
    std::ops::ControlFlow::Break(())
}
