//! Keep nearby Piranhas in bounded cohorts and steer followers toward a
//! leader. The relationship is transient: joining a loaded/reopened fish
//! clears stale entity ids so it can form a valid school again.

use mod_api::{EntityRef, ServerHost, Tag};

const LEADER_FIELD: &str = "NaturalistSchoolLeader";
const SCHOOL_RADIUS: f64 = 8.0;
const SCHOOL_LIMIT: usize = 24;

struct Fish {
    entity: EntityRef,
    pos: [f64; 3],
    leader: i32,
}

pub(crate) fn reset(host: &mut dyn ServerHost, id: u32) {
    if let Some(entity) = host.entity(id) {
        let _ = entity.merge_nbt(host, [(LEADER_FIELD.into(), Tag::Int(0))].into());
    }
}

pub(crate) fn tick(host: &mut dyn ServerHost, id: u32) {
    let Some(current) = host.entity(id) else {
        return;
    };
    let Some(position) = current.position(host) else {
        return;
    };
    let Some(level) = current.level(host) else {
        return;
    };
    let nearby = level
        .entities_in(
            host,
            [
                position[0] - SCHOOL_RADIUS,
                position[1] - SCHOOL_RADIUS,
                position[2] - SCHOOL_RADIUS,
            ],
            [
                position[0] + SCHOOL_RADIUS,
                position[1] + SCHOOL_RADIUS,
                position[2] + SCHOOL_RADIUS,
            ],
        )
        .unwrap_or_default();
    let mut fish = Vec::new();
    for entity in nearby {
        let is_piranha = entity
            .kind(host)
            .is_some_and(|kind| kind.id(host) == "naturalist:piranha");
        if !is_piranha {
            continue;
        }
        if let (Some(pos), Some(data)) = (entity.position(host), entity.nbt(host)) {
            fish.push(Fish {
                entity,
                pos,
                leader: data
                    .get(&LEADER_FIELD.into())
                    .and_then(|value| match value {
                        Tag::Int(value) => Some(*value),
                        _ => None,
                    })
                    .unwrap_or_default(),
            });
        }
    }
    if !fish.iter().any(|fish| fish.entity.id() == current.id()) {
        if let (Some(pos), Some(data)) = (current.position(host), current.nbt(host)) {
            fish.push(Fish {
                entity: current,
                pos,
                leader: data
                    .get(&LEADER_FIELD.into())
                    .and_then(|value| match value {
                        Tag::Int(value) => Some(*value),
                        _ => None,
                    })
                    .unwrap_or_default(),
            });
        }
    }
    if fish.is_empty() {
        return;
    }

    let current_leader = fish
        .iter()
        .find(|fish| fish.entity.id() == current.id())
        .map_or(0, |fish| fish.leader);
    let mut leaders: Vec<_> = fish
        .iter()
        .filter(|fish| fish.leader == fish.entity.id() as i32)
        .map(|candidate| {
            let size = fish
                .iter()
                .filter(|member| member.leader == candidate.entity.id() as i32)
                .count();
            (candidate.entity, candidate.pos, size)
        })
        .filter(|(_, _, size)| *size < SCHOOL_LIMIT)
        .collect();
    leaders.sort_by(|(left, left_pos, _), (right, right_pos, _)| {
        distance_squared(*left_pos, position)
            .total_cmp(&distance_squared(*right_pos, position))
            .then_with(|| left.id().cmp(&right.id()))
    });

    let selected = if current_leader != 0
        && fish
            .iter()
            .any(|fish| fish.entity.id() as i32 == current_leader)
    {
        fish.iter()
            .find(|fish| fish.entity.id() as i32 == current_leader)
            .map(|fish| (fish.entity, fish.pos))
    } else {
        leaders.first().map(|(entity, pos, _)| (*entity, *pos))
    };

    let (leader, leader_pos) = if let Some(leader) = selected {
        leader
    } else {
        // Stable first leader; remaining unassigned fish can form another
        // cohort when an existing school reaches the 24-member cap.
        let mut unassigned: Vec<_> = fish
            .iter()
            .filter(|fish| fish.leader == 0)
            .map(|fish| (fish.entity, fish.pos))
            .collect();
        let Some((leader, leader_pos)) = unassigned.iter().min_by_key(|(entity, _)| entity.id())
        else {
            return;
        };
        unassigned.sort_by(|(left, left_pos), (right, right_pos)| {
            distance_squared(*left_pos, *leader_pos)
                .total_cmp(&distance_squared(*right_pos, *leader_pos))
                .then_with(|| left.id().cmp(&right.id()))
        });
        for (member, _) in unassigned.into_iter().take(SCHOOL_LIMIT) {
            set_leader(host, member, leader.id());
        }
        (*leader, *leader_pos)
    };

    if current.id() != leader.id() {
        if distance_squared(position, leader_pos) > 4.0 {
            if let Some(mob) = host.mob(current) {
                let _ = mob.move_to(host, leader_pos, 1.0);
            }
        }
        if current_leader == 0 {
            let size = fish
                .iter()
                .filter(|fish| fish.leader == leader.id() as i32)
                .count();
            if size < SCHOOL_LIMIT {
                set_leader(host, current, leader.id());
            }
        }
    } else if current_leader == 0 {
        set_leader(host, current, current.id());
    }
}

fn set_leader(host: &mut dyn ServerHost, entity: EntityRef, leader: u32) {
    let _ = entity.merge_nbt(
        host,
        [(LEADER_FIELD.into(), Tag::Int(leader as i32))].into(),
    );
}

fn distance_squared(left: [f64; 3], right: [f64; 3]) -> f64 {
    (left[0] - right[0]).powi(2) + (left[1] - right[1]).powi(2) + (left[2] - right[2]).powi(2)
}
