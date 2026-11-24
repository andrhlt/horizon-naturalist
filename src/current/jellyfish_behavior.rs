//! Periodic Jellyfish movement pulse and close-range sting.

use mod_api::{ServerHost, Tag};

const PULSE_INTERVAL: i32 = 20;
const PULSE_STRENGTH: f64 = 0.18;
const PULSE_RADIUS: f64 = 1.0;
const STING_DAMAGE: f32 = 2.0;

fn pulse_hits(target_kind: Option<&str>, distance_squared: f64) -> bool {
    target_kind != Some("naturalist:jellyfish") && distance_squared <= PULSE_RADIUS * PULSE_RADIUS
}

/// Whether this tick pulses, and the timer value to persist afterwards.
pub const fn next_pulse(ticks: i32) -> (bool, i32) {
    if ticks <= 0 {
        (true, PULSE_INTERVAL - 1)
    } else {
        (false, ticks - 1)
    }
}

pub(crate) fn tick(host: &mut dyn ServerHost, id: u32) {
    let Some(entity) = host.entity(id) else {
        return;
    };
    let data = entity.nbt(host).unwrap_or_default();
    let ticks = match data.get(&"NaturalistPulseTicks".into()) {
        Some(Tag::Int(ticks)) => *ticks,
        _ => 0,
    };
    let (pulse, next) = next_pulse(ticks);
    let mut updated = data;
    updated.insert("NaturalistPulseTicks".into(), Tag::Int(next));
    entity.merge_nbt(host, updated);
    if !pulse {
        return;
    }

    let Some(level) = entity.level(host) else {
        return;
    };
    let Some(position) = entity.position(host) else {
        return;
    };
    apply_movement_pulse(host, entity, position);

    let Some(nearby) = level.entities_in(
        host,
        [
            position[0] - PULSE_RADIUS,
            position[1] - PULSE_RADIUS,
            position[2] - PULSE_RADIUS,
        ],
        [
            position[0] + PULSE_RADIUS,
            position[1] + PULSE_RADIUS,
            position[2] + PULSE_RADIUS,
        ],
    ) else {
        return;
    };
    for target in nearby {
        if target.id() == id {
            continue;
        }
        let kind = target.kind(host).map(|kind| kind.id(host));
        let Some(target_position) = target.position(host) else {
            continue;
        };
        let dx = target_position[0] - position[0];
        let dy = target_position[1] - position[1];
        let dz = target_position[2] - position[2];
        if !pulse_hits(kind.as_deref(), dx * dx + dy * dy + dz * dz) {
            continue;
        }
        if host.living(target).is_some() {
            let _ = target.hurt(
                host,
                "minecraft:mob_attack",
                STING_DAMAGE,
                Some(entity),
                Some(entity),
            );
        }
    }
}

fn apply_movement_pulse(host: &mut dyn ServerHost, entity: mod_api::EntityRef, position: [f64; 3]) {
    let Some(mob) = host.mob(entity) else {
        return;
    };
    let direction = mob
        .wanted_position(host)
        .map(|wanted| {
            [
                wanted[0] - position[0],
                wanted[1] - position[1],
                wanted[2] - position[2],
            ]
        })
        .filter(|delta| delta.iter().any(|component| component.abs() > f64::EPSILON))
        .map(normalize)
        .or_else(|| {
            let [yaw, pitch] = entity.rotation(host)?;
            let yaw = f64::from(yaw).to_radians();
            let pitch = f64::from(pitch).to_radians();
            Some([
                -yaw.sin() * pitch.cos(),
                -pitch.sin(),
                yaw.cos() * pitch.cos(),
            ])
        });
    let Some(direction) = direction else {
        return;
    };
    let Some(mut velocity) = entity.velocity(host) else {
        return;
    };
    for axis in 0..3 {
        velocity[axis] += direction[axis] * PULSE_STRENGTH;
    }
    entity.set_velocity(host, velocity);
}

fn normalize(vector: [f64; 3]) -> [f64; 3] {
    let length = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
    if length > 0.0 {
        vector.map(|component| component / length)
    } else {
        [0.0; 3]
    }
}

#[cfg(test)]
mod tests {
    use super::{next_pulse, normalize, pulse_hits};

    #[test]
    fn pulses_immediately_then_every_twenty_ticks() {
        let (first, mut timer) = next_pulse(0);
        assert!(first);
        assert_eq!(timer, 19);
        for _ in 0..19 {
            let (pulse, next) = next_pulse(timer);
            assert!(!pulse);
            timer = next;
        }
        let (second, reset) = next_pulse(timer);
        assert!(second);
        assert_eq!(reset, 19);
    }

    #[test]
    fn movement_pulse_uses_normalized_target_direction() {
        let direction = normalize([4.0, 0.0, 0.0]);
        assert_eq!(direction, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn pulse_only_stings_close_non_jellyfish() {
        assert!(pulse_hits(Some("minecraft:cod"), 0.04));
        assert!(!pulse_hits(Some("minecraft:cod"), 4.0));
        assert!(!pulse_hits(Some("naturalist:jellyfish"), 0.04));
    }
}
