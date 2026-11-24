//! Great White Shark hunt satiation and prey-goal gating.

use mod_api::{GoalSpec, MobFilter, ServerHost, Tag};

pub const HUNTING_COOLDOWN: i32 = 2400;

const DROWN_TICKS: i32 = 20;
const DROWN_DAMAGE: f32 = 2.0;

fn next_air(current: i32, in_water: bool, maximum: i32) -> (i32, bool) {
    if in_water {
        (maximum, false)
    } else if current <= 1 - DROWN_TICKS {
        (0, true)
    } else {
        (current - 1, false)
    }
}

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

/// Advance the prey cooldown saved by the landed-kill combat hook.
///
/// Keep this in the entity tick path so the timer survives save/reload and
/// eventually re-enables the prey selector instead of leaving a Shark sated
/// forever.
pub(crate) fn tick_hunting_cooldown(host: &mut dyn ServerHost, id: u32) {
    let Some(entity) = host.entity(id) else {
        return;
    };
    let Some(data) = entity.nbt(host) else {
        return;
    };
    let current = hunting_cooldown(&data);
    if current == 0 {
        return;
    }
    let mut patch = mod_api::Compound::new();
    patch.insert(
        "HuntingCooldown".into(),
        Tag::Int(next_hunting_cooldown(current)),
    );
    let _ = entity.merge_nbt(host, patch);
}

/// Keep Shark air behavior independent of its Cow-derived mob template.
/// Java loses one air per dry tick and takes two drown damage on tick twenty.
pub(crate) fn tick_air(host: &mut dyn ServerHost, id: u32) {
    let Some(entity) = host.entity(id) else { return };
    let Some(current) = entity.air_supply(host) else { return };
    let maximum = entity.max_air_supply(host).unwrap_or(300);
    let in_water = entity.is_in_water(host) == Some(true);
    let (next, drown) = next_air(current, in_water, maximum);
    if !entity.set_air_supply(host, next) {
        return;
    }
    if drown {
        let _ = entity.hurt(host, "minecraft:drown", DROWN_DAMAGE, None, None);
    }
}

#[cfg(test)]
mod tests {
    use super::{next_air, next_hunting_cooldown, targeting_enabled, HUNTING_COOLDOWN};

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

    #[test]
    fn shark_dry_air_matches_nineteen_safe_ticks_and_twenty_tick_damage() {
        let mut air = 0;
        for _ in 0..19 {
            let (next, damage) = next_air(air, false, 300);
            assert!(!damage);
            air = next;
        }
        assert_eq!(air, -19);
        assert_eq!(next_air(air, false, 300), (0, true));
        assert_eq!(next_air(-19, true, 300), (300, false));
        assert_eq!(next_air(300, false, 300), (299, false));
    }
}
