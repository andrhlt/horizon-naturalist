//! Piranha schooling based on Naturalist's 24-fish school cap.
use horizon_client::mobs::{Mob, MobEvent, MobWorld, goals::{Cx, Goal, reduced}};
use horizon_core::geometry::Vec3;

#[derive(Clone, Debug)]
pub(crate) struct PiranhaSchool {
    next_start: i32,
    recalc: i32,
}
impl PiranhaSchool {
    pub(crate) fn new(mob: &mut Mob) -> Self {
        Self { next_start: horizon_client::mobs::aquatic::FollowFlockLeader::next_start_tick(mob), recalc: 0 }
    }
}
fn follows(world: &MobWorld, leader: Option<u32>) -> bool {
    leader.and_then(|id| world.seen(id)).is_some_and(|seen| seen.alive)
}
impl Goal for PiranhaSchool {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn name(&self) -> &'static str { "NaturalistPiranhaSchool" }
    fn flags(&self) -> u8 { 0 }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if mob.extra.followers > 0 { return false; }
        if follows(cx.w, mob.extra.leader) { return true; }
        if self.next_start > 0 { self.next_start -= 1; return false; }
        self.next_start = horizon_client::mobs::aquatic::FollowFlockLeader::next_start_tick(mob);
        let area = mob.bounds().inflate(Vec3::new(8.0, 8.0, 8.0));
        let mut nearby: Vec<_> = cx.w.others_near(area)
            .filter(|seen| seen.alive && seen.kind.id == "naturalist:piranha" && seen.bounds.intersects(area))
            .collect();
        nearby.sort_by_key(|seen| (seen.section.0[0], seen.section.0[2], seen.section.0[1], seen.section.1));
        let candidates: Vec<_> = nearby.into_iter().filter_map(|seen| {
            let school = if seen.id == mob.id { (mob.extra.leader, mob.extra.followers) } else { seen.school };
            (school.1 > 0 && school.1 + 1 < 24 || !follows(cx.w, school.0)).then_some((seen, school))
        }).collect();
        let (leader, followers) = candidates.iter().find(|(_, (_, count))| *count > 0 && *count + 1 < 24)
            .map_or((mob.id, mob.extra.followers), |(seen, (_, count))| (seen.id, *count));
        let room = (24 - followers - 1).max(0) as usize;
        for (seen, _) in candidates.into_iter().filter(|(_, (leader, _))| !follows(cx.w, *leader)).take(room) {
            if seen.id == mob.id {
                mob.extra.leader = Some(leader);
                cx.events.push(MobEvent::SchoolSize { leader, by: 1 });
            } else if seen.id != leader {
                cx.events.push(MobEvent::StartFollowing { fish: seen.id, leader });
            }
        }
        follows(cx.w, mob.extra.leader)
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.extra.leader.and_then(|id| cx.w.seen(id)).is_some_and(|leader| leader.alive && mob.distance_sqr(leader.position) <= 121.0)
    }
    fn start(&mut self, _: &mut Mob, _: &mut Cx) { self.recalc = 0; }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) {
        if let Some(leader) = mob.extra.leader.take() { cx.events.push(MobEvent::SchoolSize { leader, by: -1 }); }
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.recalc -= 1;
        if self.recalc > 0 { return; }
        self.recalc = reduced(10);
        if let Some(leader) = mob.extra.leader.and_then(|id| cx.w.seen(id)).filter(|leader| leader.alive) {
            mob.nav_move_to_entity(cx.w, [leader.position.x, leader.position.y, leader.position.z], 1, 1.0);
        }
    }
}
