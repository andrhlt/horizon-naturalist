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
    let mut fish = Vec::new();
    for entity in nearby {
        if is_piranha(host, entity) {
            if let Some(pos) = entity.position(host) {
                fish.push((entity, pos));
            }
        }
    }
    if fish.is_empty() {
        return;
    }

    fish.sort_by(|(left, left_pos), (right, right_pos)| {
        distance_squared(*left_pos, position)
            .total_cmp(&distance_squared(*right_pos, position))
            .then_with(|| left.id().cmp(&right.id()))
    });
    let (leader, leader_pos) = fish[0];
    for (member, _) in fish.iter().take(SCHOOL_LIMIT) {
        set_leader(host, *member, leader.id());
    }

    if current.id() != leader.id() {
        if let Some(mob) = host.mob(current)
            && distance_squared(position, leader_pos) > 4.0
        {
            let _ = mob.move_to(host, leader_pos, 1.0);
        }
        // A newly spawned fish joins an existing school only while it has room.
        set_leader(host, current, leader.id());
    }
}

fn is_piranha(host: &mut dyn ServerHost, entity: EntityRef) -> bool {
    host.entity(entity.id())
        .and_then(|entity| entity.kind(host))
        .is_some_and(|kind| kind.id(host) == "naturalist:piranha")
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
