fn snail_travel(
    mob: &mut Mob,
    world: &MobWorld,
    events: &mut Vec<MobEvent>,
) -> std::ops::ControlFlow<()> {
    use super::naturalist_snail::{snail, snail_mut};
    use std::ops::ControlFlow;
    if mob.kind.id != "naturalist:snail" {
        return ControlFlow::Continue(());
    }
    let hiding = snail(mob).hiding;
    let mut surface = std::mem::take(&mut snail_mut(mob).surface);
    if hiding || mob.no_ai {
        surface.halt();
    }
    let attached_travel = !mob.no_ai && surface.travel(mob, world, events);
    snail_mut(mob).surface = surface;
    if !attached_travel {
        if !hiding {
            return ControlFlow::Continue(());
        }
        if (mob.is_in_liquid() || mob.fluids.modded.is_some()) && mob.affected_by_fluids() {
            mob.travel_in_fluid(world, Vec3::ZERO, events);
        } else {
            mob.travel_in_air(world, Vec3::ZERO, events);
        }
    }
    if hiding {
        mob.velocity.x = 0.0;
        mob.velocity.z = 0.0;
    }
    ControlFlow::Break(())
}
