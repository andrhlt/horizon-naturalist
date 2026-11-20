fn scorpion_poison(mob: &Mob, cx: &mut Cx<'_, '_>) {
    use super::naturalist_scorpion::{DESERT, JUNGLE};
    let ticks = match mob.kind.id.as_str() {
        DESERT | JUNGLE => 120,
        "naturalist:komodo_dragon" => 100,
        _ => return,
    };
    let Some(attack) = cx.events.last_mut() else { return };
    let effect = "minecraft:poison";
    let amplifier = 0;
    *attack = match *attack {
        MobEvent::AttackPlayer(amount, from) => MobEvent::AttackPlayerWithEffect {
            amount, from, effect, ticks, amplifier,
        },
        MobEvent::AttackMob { target, amount, source } => MobEvent::AttackMobWithEffect {
            target, amount, source, effect, ticks, amplifier,
        },
        _ => return,
    };
}
