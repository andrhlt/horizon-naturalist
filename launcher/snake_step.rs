fn snake_step(mob: &mut Mob, world: &MobWorld<'_>, events: &mut Vec<MobEvent>) {
    use super::{goals::naturalist_snake_goals as snake, nav::naturalist_elephants as elephant};
    if elephant::includes(&mob.kind.id) {
        elephant::step_anger(mob, world);
    }
    if mob.kind.id == snake::KIND && mob.alive() {
        snake::step(mob, world, events);
        if mob.climbing {
            mob.velocity.y *= 0.5;
        }
    }
}
