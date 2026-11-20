fn deer_social_follow(goal: &mut FollowParent, mob: &Mob, context: &Cx<'_, '_>) -> std::ops::ControlFlow<bool> {
    use super::naturalist_deer_social::{ZEBRA_MARKER, same_farm_template_animal};
    use std::ops::ControlFlow;

    let equine = mob.species() == Species::Donkey;
    let ostrich = mob.kind.id == super::naturalist_ostrich::KIND;
    let zebra = mob.kind.id == super::naturalist_zebra::KIND;
    if !equine && !matches!(mob.species(), Species::Cow | Species::Chicken | Species::Pig) {
        return ControlFlow::Continue(());
    }
    let bear = super::naturalist_bears::is_bear(&mob.kind.id);
    let distanced = bear || matches!(mob.kind.id.as_str(), "naturalist:duck" | "naturalist:capybara");
    if mob.extra.age >= 0 || (distanced && mob.extra.tame) {
        return ControlFlow::Break(false);
    }
    let pos = mob.position;
    let whale = mob.kind.id == crate::mobs::nav::naturalist_whale_state::KIND;
    let bounds_scan = whale || matches!(
        mob.kind.id.as_str(),
        "naturalist:deer" | "naturalist:boar" | "naturalist:turkey"
            | "naturalist:rhino" | "naturalist:komodo_dragon" | "naturalist:rat"
            | "naturalist:tiger" | "naturalist:lion" | "naturalist:giant_isopod"
    );
    let scan = if bounds_scan {
        mob.bounds().inflate(Vec3::new(8.0, 4.0, 8.0))
    } else if distanced {
        let range = if bear { 48.0 } else { 16.0 };
        let bounds = mob.bounds();
        let lo = bounds.min_position();
        let hi = bounds.max_position();
        Aabb::new(Vec3::new(lo.x - range, lo.y - 8.0, lo.z - range),
                  Vec3::new(hi.x + range, hi.y + 8.0, hi.z + range))
    } else {
        Aabb::new(
            Vec3::new(pos.x - 9.0, pos.y - 5.0, pos.z - 9.0),
            Vec3::new(pos.x + 9.0, pos.y + 5.0, pos.z + 9.0),
        )
    };
    let threshold = if bear { 144.0 } else if distanced { 25.0 } else { 9.0 };
    let mut nearest: Option<(u32, f64)> = None;
    for other in context.w.others_near(scan).filter(|other| {
        other.alive
            && if equine {
                other.species == Species::Donkey && zebra == (other.variant == ZEBRA_MARKER)
                    && ostrich == other.effects.contains(&super::naturalist_ostrich::KIND)
            } else {
                same_farm_template_animal(mob, other)
            }
            && !other.baby
            && if distanced || bounds_scan {
                scan.intersects(other.bounds)
            } else {
                (other.position.x - pos.x).abs() <= 8.0
                    && (other.position.y - pos.y).abs() <= 4.0
                    && (other.position.z - pos.z).abs() <= 8.0
            }
    }) {
        let distance = mob.distance_sqr(other.position);
        if nearest.is_none_or(|(_, best_distance)| distance <= best_distance) {
            nearest = Some((other.id, distance));
        }
    }
    ControlFlow::Break(match nearest {
        Some((parent, distance)) if distance >= threshold => {
            goal.parent = Some(parent);
            true
        }
        _ => false,
    })
}
