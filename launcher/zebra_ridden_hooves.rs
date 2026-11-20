fn zebra_ridden_hooves(
    mob: &mut Mob, world: &MobWorld<'_>, pos: [i32; 3],
    state: horizon_world::registry::BlockStateId, events: &mut Vec<MobEvent>,
) -> std::ops::ControlFlow<()> {
    use std::ops::ControlFlow;
    if mob.kind.id != super::naturalist_zebra::KIND || !is_vehicle(mob) {
        return ControlFlow::Continue(());
    }
    let block = world.facts.get(state);
    if matches!(&*block.name, "minecraft:water" | "minecraft:lava" | "minecraft:bubble_column") {
        return ControlFlow::Break(());
    }
    let above = world.facts.get(world.world.block(pos[0], pos[1] + 1, pos[2]));
    let sounds = if &*above.name == "minecraft:snow" { &above.sounds } else { &block.sounds };
    mob.ai.steps.gallop += 1;
    let count = mob.ai.steps.gallop;
    let at = [mob.position.x, mob.position.y, mob.position.z];
    if count <= 5 {
        events.push(MobEvent::Sound("entity.horse.step_wood", at, sounds.volume * 0.15, sounds.pitch));
    } else if count % 3 == 0 {
        events.push(MobEvent::Sound("entity.horse.gallop", at, sounds.volume * 0.15, sounds.pitch));
        if mob.random.next_int_bound(10).unwrap_or(1) == 0 {
            events.push(MobEvent::Sound("naturalist:entity.zebra.breathe", at, sounds.volume * 0.6, sounds.pitch));
        }
    }
    ControlFlow::Break(())
}
