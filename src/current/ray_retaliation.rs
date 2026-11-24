//! Preserve Ray retaliation when its native hurt-by goal misses a recorded attacker.

use mod_api::{ActiveEffect, Compound, ServerHost, Tag};

const ATTACK_INTERVAL: i32 = 20;
const ATTACK_RANGE_SQUARED: f64 = 2.25;
const ATTACK_COOLDOWN: &str = "NaturalistRayAttackCooldown";

fn integer(data: &Compound, key: &str) -> Option<i32> {
    match data.get(&key.into()) {
        Some(Tag::Int(value)) => Some(*value),
        Some(Tag::Short(value)) => Some(i32::from(*value)),
        Some(Tag::Byte(value)) => Some(i32::from(*value)),
        _ => None,
    }
}

pub(crate) fn tick(host: &mut dyn ServerHost, id: u32) {
    let Some(entity) = host.entity(id) else {
        return;
    };
    let Some(mob) = host.mob(entity) else {
        return;
    };
    if mob.target(host).is_none() {
        let Some(attacker) = host
            .living(entity)
            .and_then(|living| living.last_attacker(host))
        else {
            return;
        };
        if attacker.id() == id {
            return;
        }
        let Some(attacker_living) = host.living(attacker) else {
            return;
        };
        let _ = mob.set_target(host, Some(attacker_living));
    }

    let Some(target) = mob.target(host) else {
        return;
    };
    if target.health(host).is_none_or(|health| health <= 0.0) {
        mob.set_target(host, None);
        return;
    }
    let Some(origin) = entity.position(host) else {
        return;
    };
    let Some(destination) = target.entity().position(host) else {
        return;
    };
    let dx = destination[0] - origin[0];
    let dy = destination[1] - origin[1];
    let dz = destination[2] - origin[2];
    let distance_squared = dx * dx + dy * dy + dz * dz;
    if distance_squared > ATTACK_RANGE_SQUARED {
        let _ = mob.move_to(host, destination, 1.0);
        return;
    }

    let data = entity.nbt(host).unwrap_or_default();
    let cooldown = integer(&data, ATTACK_COOLDOWN).unwrap_or(0).max(0);
    if cooldown > 0 {
        let _ = entity.merge_nbt(
            host,
            Compound::from([(ATTACK_COOLDOWN.into(), Tag::Int(cooldown - 1))]),
        );
        return;
    }
    let target_data = target.entity().nbt(host).unwrap_or_default();
    if integer(&target_data, "HurtTime").is_some_and(|ticks| ticks > 0) {
        return;
    }
    if target
        .entity()
        .hurt(
            host,
            "minecraft:mob_attack",
            2.0,
            Some(entity),
            Some(entity),
        )
        == Some(true)
    {
        if let Some(poison) = host.effect_key("minecraft:poison") {
            let _ = target.add_effect(
                host,
                &ActiveEffect::new(poison, 100, 0, false, true, true),
            );
        }
        let _ = entity.merge_nbt(
            host,
            Compound::from([(ATTACK_COOLDOWN.into(), Tag::Int(ATTACK_INTERVAL))]),
        );
    }
}
