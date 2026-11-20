fn bird_ambient(mob: &mut Mob, world: &MobWorld, events: &mut Vec<MobEvent>) -> std::ops::ControlFlow<()> {
    use super::nav::naturalist_bird_voice::{call, request_note};
    use std::ops::ControlFlow;
    match mob.kind.id.as_str() {
        "naturalist:bird" if world.daylight => {
            mob.voice(call(&mob.ai.nav), events);
            let p = mob.position;
            request_note(&mut mob.ai.nav, [p.x, p.y + 1.0, p.z]);
        }
        "naturalist:bird" | "naturalist:dragonfly" => {}
        "naturalist:ant" => mob.voice("naturalist:entity.ant.ambient", events),
        "naturalist:butterfly" => mob.voice("naturalist:entity.butterfly.ambient", events),
        "naturalist:hedgehog" => {
            let pose = super::naturalist_hedgehog::state(mob);
            if !pose.hiding && !pose.rolling {
                mob.voice("naturalist:entity.hedgehog.ambient", events);
            }
        }
        "naturalist:vulture" => mob.voice("naturalist:entity.vulture.ambient", events),
        "naturalist:firefly" => mob.voice("naturalist:entity.firefly.ambient", events),
        _ => return ControlFlow::Continue(()),
    }
    ControlFlow::Break(())
}
