//! Keep Piranhas in nearby, bounded cohorts and steer followers toward their
//! leader. The relationship is transient: a loaded/reopened fish is free to
//! join a newly formed school instead of retaining a stale entity id.

use mod_api::{EntityRef, Host, ServerHost, Tag};

const LEADER_FIELD: &str = "NaturalistSchoolLeader";
const SCHOOL_RADIUS: f64 = 8.0;
const SCHOOL_LIMIT: usize = 24;

pub(crate) fn reset(host: &mut dyn ServerHost, id: u32) {
    if let Some(entity) = host.entity(id) {
        let _ = entity.merge_nbt(host, [(LEADER_FIELD.into(), Tag::Int(0))].into());
    }
}

pub(crate) fn tick(host: &mut dyn ServerHost, id: u32) {
    let Some(current) = host.entity(id) else {
        return;
    };
    if !is_piranha(host, current) {
        return;
    }
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
    let fish: Vec<_> = nearby
        .into_iter()
        .filter(|entity| is_piranha(host, *entity))
        .filter_map(|entity| entity.position(host).map(|pos| (entity, pos)))
        .collect();
    if fish.is_empty() {
        return;
    }

    let current_leader = school_leader(host, current);
    let mut leaders: Vec<_> = fish
        .iter()
        .filter(|(entity, _)| school_leader(host, *entity) == entity.id() as i32)
        .map(|(entity, pos)| {
            let size = fish
                .iter()
                .filter(|(member, _)| school_leader(host, *member) == entity.id() as i32)
                .count();
            (*entity, *pos, size)
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
            .any(|(entity, _)| entity.id() as i32 == current_leader)
    {
        fish.iter()
            .find(|(entity, _)| entity.id() as i32 == current_leader)
            .map(|(entity, pos)| (*entity, *pos))
    } else {
        leaders.first().map(|(entity, pos, _)| (*entity, *pos))
    };

    let (leader, leader_pos) = if let Some(leader) = selected {
        leader
    } else {
        // The smallest id makes the first school stable regardless of which
        // member receives its tick first. Unassigned fish form additional
        // schools after an earlier cohort reaches the 24-member limit.
        let mut unassigned: Vec<_> = fish
            .iter()
            .filter(|(entity, _)| school_leader(host, *entity) == 0)
            .copied()
            .collect();
        let Some((leader, leader_pos)) = unassigned.iter().min_by_key(|(entity, _)| entity.id())
        else {
            return;
        };
        let mut members = std::mem::take(&mut unassigned);
        members.sort_by(|(left, left_pos), (right, right_pos)| {
            distance_squared(*left_pos, *leader_pos)
                .total_cmp(&distance_squared(*right_pos, *leader_pos))
                .then_with(|| left.id().cmp(&right.id()))
        });
        for (member, _) in members.into_iter().take(SCHOOL_LIMIT) {
            set_leader(host, member, leader.id());
        }
        (*leader, *leader_pos)
    };

    if current.id() != leader.id() {
        if let Some(mob) = host.mob(current)
            && distance_squared(position, leader_pos) > 4.0
        {
            let _ = mob.move_to(host, leader_pos, 1.0);
        }
        // A newly spawned fish joins an existing school only while it has room.
        if current_leader == 0 {
            let size = fish
                .iter()
                .filter(|(member, _)| school_leader(host, *member) == leader.id() as i32)
                .count();
            if size < SCHOOL_LIMIT {
                set_leader(host, current, leader.id());
            }
        }
    } else if current_leader == 0 {
        set_leader(host, current, current.id());
    }
}

fn is_piranha(host: &mut dyn ServerHost, entity: EntityRef) -> bool {
    host.entity(entity)
        .and_then(|entity| entity.kind(host))
        .is_some_and(|kind| kind.id(host) == "naturalist:piranha")
}

fn school_leader(host: &mut dyn ServerHost, entity: EntityRef) -> i32 {
    entity
        .nbt(host)
        .and_then(|data| data.get(&LEADER_FIELD.into()).cloned())
        .and_then(|value| match value {
            Tag::Int(value) => Some(value),
            _ => None,
        })
        .unwrap_or_default()
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
