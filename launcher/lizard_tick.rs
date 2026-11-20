fn lizard_tick(mob: &mut Mob, _world: &MobWorld<'_>, events: &mut Vec<MobEvent>) {
    if !mob.alive() {
        return;
    }
    match mob.kind.id.as_str() {
        "naturalist:lizard" => {
            if super::naturalist_lizard::tick_regrow(&mut mob.ai.nav) && !mob.silent {
                events.push(MobEvent::Sound("minecraft:entity.slime.squish", mob.center(), 1.0, 1.0));
            }
        }
        "naturalist:lizard_tail" if !mob.fluids.in_water && mob.on_ground && mob.vertical_collision_below => {
            let drift = |value: f32| f64::from((value * 2.0 - 1.0) * 0.05);
            mob.velocity.x += drift(mob.random.next_float());
            mob.velocity.y += f64::from(0.4_f32);
            mob.velocity.z += drift(mob.random.next_float());
            mob.on_ground = false;
            mob.voice("entity.salmon.flop", events);
        }
        _ => {}
    }
}
