fn blobfish_goals(mob: &mut Mob) {
    use super::{control::{LookKind, MoveKind}, goals::{Avoid, AvoidEntity, MeleeAttack, NearestTarget, Prey, Stroll},
        nav::naturalist_blobfish_state as blobfish};

    if mob.kind.id != "naturalist:blobfish" {
        return;
    }
    mob.ai.mov.kind = MoveKind::SmoothSwimming {
        max_turn_x: 1000.0, max_turn_y: 5.0, in_water: 0.02, outside: 0.1, gravity: false,
    };
    mob.ai.look.kind = LookKind::SmoothSwimming(5.0);
    mob.ai.goals.clear();
    mob.ai.goals.add(0, AvoidEntity::new(Avoid::Player, 6.0, 1.5, 2.0));
    mob.ai.goals.add(0, AvoidEntity::new(Avoid::Species(vec![Species::Axolotl]), 6.0, 1.5, 2.0));
    mob.ai.goals.add(1, MeleeAttack::new(1.6, true));
    let deep_swim = Stroll::swimming(1.0, 20).named("BlobfishStayDeepGoal").position(|mob, context| {
        let deep = f64::from(context.w.sea_level - blobfish::DEEP_OFFSET);
        if mob.position.y > deep {
            let target = [mob.position.x, deep - 4.0, mob.position.z];
            return mob.default_random_pos_towards(context.w, 10, 7, target, std::f64::consts::FRAC_PI_2);
        }
        let mut position = mob.default_random_pos(context.w, 10, 7);
        let mut tries = 0;
        while let Some(pos) = position {
            let [x, y, z] = pos.map(|coordinate| coordinate.floor() as i32);
            let state = context.w.world.block(x, y, z);
            if context.w.data.state(state).flags & super::data::WATER != 0 || tries >= 10 {
                break;
            }
            tries += 1;
            position = mob.default_random_pos(context.w, 10, 7);
        }
        position.filter(|position| position[1] <= deep)
    });
    mob.ai.goals.add(2, deep_swim);
    let prey = Prey::Filter(|seen| matches!(seen.variant, blobfish::SNAIL_PREY | blobfish::CRAB_PREY));
    mob.ai.targets.add(1, NearestTarget::new(prey, true));
}
