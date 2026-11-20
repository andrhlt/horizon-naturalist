fn bass_school_goal(goal: &mut FollowFlockLeader, mob: &mut Mob, context: &mut Cx<'_, '_>) -> std::ops::ControlFlow<bool> {
    use super::nav::{naturalist_bass_state as bass, naturalist_catfish_state as catfish, naturalist_piranha_state as piranha};
    use std::ops::ControlFlow;

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum School { Cod, Bass, Piranha }
    fn class(seen: &crate::mobs::Seen) -> Option<School> {
        if seen.species != Species::Cod || matches!(seen.variant, catfish::SNAPSHOT | catfish::STARFISH_SNAPSHOT | catfish::CLAM_SNAPSHOT | catfish::ANGLERFISH_SNAPSHOT | super::nav::naturalist_blobfish_state::SNAPSHOT | super::nav::naturalist_jellyfish_state::SNAPSHOT | piranha::RAY_SNAPSHOT) {
            return None;
        }
        if bass::snapshot_tier(seen).is_some() { return Some(School::Bass); }
        if seen.variant == piranha::SNAPSHOT { return Some(School::Piranha); }
        Some(School::Cod)
    }

    if mob.species() != Species::Cod {
        return ControlFlow::Continue(());
    }
    let school = match mob.kind.id.as_str() {
        "naturalist:bass" => School::Bass,
        "naturalist:piranha" => School::Piranha,
        "naturalist:catfish" | "naturalist:blobfish" | "naturalist:jellyfish" | "naturalist:ray" | "naturalist:anglerfish" | "naturalist:starfish" | "naturalist:clam" => return ControlFlow::Break(false),
        _ => School::Cod,
    };
    let area = mob.bounds().inflate(Vec3::new(8.0, 8.0, 8.0));
    if school == School::Cod && !context.w.others_near(area).any(|seen| seen.species == Species::Cod && class(seen) != Some(School::Cod)) {
        return ControlFlow::Continue(());
    }
    if mob.extra.followers > 0 {
        return ControlFlow::Break(false);
    }
    if follows(context.w, mob.extra.leader) {
        return ControlFlow::Break(true);
    }
    if goal.next_start > 0 {
        goal.next_start -= 1;
        return ControlFlow::Break(false);
    }
    goal.next_start = FollowFlockLeader::next_start_tick(mob);

    let capacity = match school {
        School::Cod => max_school_size(Species::Cod),
        School::Bass => bass::MAX_SCHOOL_SIZE,
        School::Piranha => piranha::MAX_SCHOOL_SIZE,
    };
    let joinable = |followers: i32| followers > 0 && followers + 1 < capacity;
    let mut nearby: Vec<_> = context.w.others_near(area).filter(|seen| {
        seen.alive && seen.bounds.intersects(area) && class(seen) == Some(school)
    }).collect();
    nearby.sort_by_key(|seen| {
        let own = seen.id == mob.id;
        let pos = if own { mob.position } else { seen.position };
        let section = pos.components().map(|coordinate| (coordinate.floor() as i32) >> 4);
        let track = if own { (mob.extra.section, mob.extra.section_seq) } else { seen.section };
        // A newly joined fish can be visible before its first section update.
        // Java orders it after the established fish in its actual section.
        let valid_order = track.0 == section && track.1 != 0;
        let [x, y, z] = section;
        (x, z, y, !valid_order, if valid_order { track.1 } else { u64::from(seen.id) })
    });

    let candidates: Vec<_> = nearby.into_iter().filter_map(|seen| {
        let school = if seen.id == mob.id {
            (mob.extra.leader, mob.extra.followers)
        } else {
            seen.school
        };
        (joinable(school.1) || !follows(context.w, school.0)).then_some((seen, school))
    }).collect();
    let tier_now = (school == School::Bass).then_some(mob.extra.variant);
    let (leader, followers, leader_tier) = candidates.iter()
        .find(|(_, (_, followers))| joinable(*followers))
        .map_or((mob.id, mob.extra.followers, tier_now),
            |(seen, (_, followers))| (seen.id, *followers, bass::snapshot_tier(seen)));
    let room = (capacity - followers - 1).max(0) as usize;
    // Java applies Stream.limit before the leader and ineligible bass tiers are skipped.
    for (seen, _) in candidates.into_iter().filter(|(_, (leader, _))| !follows(context.w, *leader)).take(room) {
        let tier = if seen.id == mob.id { tier_now } else { bass::snapshot_tier(seen) };
        if seen.id == leader || leader_tier != tier || tier == Some(2) {
            continue;
        }
        if seen.id == mob.id {
            mob.extra.leader = Some(leader);
            context.events.push(MobEvent::SchoolSize { leader, by: 1 });
        } else {
            context.events.push(MobEvent::StartFollowing { fish: seen.id, leader });
        }
    }
    ControlFlow::Break(follows(context.w, mob.extra.leader))
}
