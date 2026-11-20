fn duck_no_ai_fall(mob: &mut Mob) {
    if !mob.no_ai || mob.kind.id != "naturalist:duck" {
        return;
    }
    // Duck.aiStep still flaps and dampens an existing downward motion
    // when NoAI suppresses the stock Chicken travel and wing step.
    chicken_flap(mob);
    if !mob.on_ground && mob.velocity.y < 0.0 {
        mob.velocity.y *= 0.6;
    }
}
