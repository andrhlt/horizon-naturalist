fn ray_poison(mob: &Mob, context: &mut Cx<'_, '_>) {
    if mob.kind.id != "naturalist:ray" {
        return;
    }

    // The ordinary hit has already computed melee attributes and knockback.
    // The stock hit-with-effect path applies poison only if the damage lands.
    const EFFECT: &str = "minecraft:poison";
    const TICKS: i32 = 100;
    if let Some(event) = context.events.last_mut() {
        match event {
            MobEvent::AttackPlayer(amount, from) => {
                *event = MobEvent::AttackPlayerWithEffect { amount: *amount, from: *from,
                    effect: EFFECT, ticks: TICKS, amplifier: 0 };
            }
            MobEvent::AttackMob { target, amount, source } => {
                *event = MobEvent::AttackMobWithEffect { target: *target, amount: *amount, source: *source,
                    effect: EFFECT, ticks: TICKS, amplifier: 0 };
            }
            _ => {}
        }
    }
}
