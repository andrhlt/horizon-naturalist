//! Great White Shark hunt satiation and prey-goal gating.

use mod_api::{GoalSpec, MobFilter, ServerHost, Tag};

pub const HUNTING_COOLDOWN: i32 = 2400;

pub const fn next_hunting_cooldown(ticks: i32) -> i32 {
    if ticks > 0 {
        ticks - 1
    } else {
        0
    }
}

pub const fn targeting_enabled(cooldown: i32) -> bool {
    cooldown <= 0
}

fn hunting_cooldown(data: &mod_api::Compound) -> i32 {
    match data.get(&"HuntingCooldown".into()) {
        Some(Tag::Int(ticks)) => (*ticks).max(0),
        _ => 0,
    }
}

/// Remove the prey selector while sated and restore it when the persisted
/// cooldown reaches zero. The melee goal is unaffected.
pub(crate) fn update_target_gate(host: &mut dyn ServerHost, id: u32) {
    let Some(entity) = host.entity(id) else {
        return;
    };
    let Some(mob) = host.mob(entity) else {
        return;
    };
    let sated = entity
        .nbt(host)
        .is_some_and(|data| !targeting_enabled(hunting_cooldown(&data)));
    let has_selector = mob
        .target_goals(host)
        .is_some_and(|goals| goals.iter().any(|goal| goal.id == "nearest_target"));
    if sated {
        if has_selector {
            mob.remove_target_goal(host, "nearest_target");
        }
        if mob.target(host).is_some() {
            mob.set_target(host, None);
        }
    } else if !has_selector {
        let goal = GoalSpec::NearestTarget {
            prey: MobFilter::Species(vec![
                "minecraft:cod".into(),
                "minecraft:salmon".into(),
                "minecraft:turtle".into(),
            ]),
            must_see: true,
            must_reach: false,
            interval: 10,
        };
        if !mob.add_target_goal(host, 2, &goal) {
            eprintln!("Naturalist: failed to restore Shark prey target goal");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{next_hunting_cooldown, targeting_enabled, HUNTING_COOLDOWN};

    #[test]
    fn hunting_timer_starts_at_java_value_and_stops_at_zero() {
        assert_eq!(HUNTING_COOLDOWN, 2400);
        assert_eq!(next_hunting_cooldown(HUNTING_COOLDOWN), 2399);
        assert_eq!(next_hunting_cooldown(1), 0);
        assert_eq!(next_hunting_cooldown(0), 0);
        assert_eq!(next_hunting_cooldown(-3), 0);
        assert!(!targeting_enabled(HUNTING_COOLDOWN));
        assert!(!targeting_enabled(1));
        assert!(targeting_enabled(0));
    }
}
