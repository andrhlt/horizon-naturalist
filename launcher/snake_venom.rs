fn snake_venom(mob: &Mob, who: Who, cx: &mut Cx<'_, '_>) {
    if !super::goals::naturalist_snake_goals::venomous(mob) {
        return;
    }
    let effect = "minecraft:poison";
    let ticks = 40;
    let amplifier = 0;
    // Snake.doHurtTarget applies poison before delegating to Mob.doHurtTarget.
    cx.events.push(match who {
        Who::Player => MobEvent::EffectPlayer {
            effect,
            ticks,
            amplifier,
            ambient: false,
        },
        Who::Mob(target) => MobEvent::EffectMob {
            target,
            effect,
            ticks,
            amplifier,
        },
    });
}
