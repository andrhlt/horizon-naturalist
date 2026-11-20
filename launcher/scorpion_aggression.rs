fn scorpion_aggression(mob: &mut Mob) {
    use super::naturalist_scorpion::{DESERT, JUNGLE};
    if matches!(mob.kind.id.as_str(), DESERT | JUNGLE) {
        mob.ai.aggressive = mob.ai.target.is_some();
    }
}
