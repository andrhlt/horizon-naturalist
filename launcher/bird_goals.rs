fn bird_goals(mob: &mut Mob) {
    use super::goals::{Float, LookAtPlayer};

    if mob.kind.id != "naturalist:bird" {
        return;
    }
    mob.extra.color = u8::MAX;
    mob.ai.goals.clear();
    super::tame::naturalist_duck_pet_goals::add_bird_goals(mob);
    super::goals::naturalist_bird_goals::register(mob);
    let goals = &mut mob.ai.goals;
    goals.add(0, Float);
    goals.add(7, LookAtPlayer::new(8.0));
}
