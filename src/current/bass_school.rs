//! Bass schooling on the engine's per-mob goal selector and tick snapshot.
use horizon_client::mobs::{
    Mob, MobEvent, MobWorld,
    aquatic::FollowFlockLeader,
    goals::{Cx, Goal, reduced},
};
use horizon_core::geometry::Vec3;

#[derive(Clone, Debug)]
pub(crate) struct BassSchool {
    next_start: i32,
    recalc: i32,
}
impl BassSchool {
    pub(crate) fn new(mob: &mut Mob) -> Self {
        Self {
            next_start: FollowFlockLeader::next_start_tick(mob),
            recalc: 0,
        }
    }
}
fn follows(world: &MobWorld, leader: Option<u32>) -> bool {
    leader
        .and_then(|id| world.seen(id))
        .is_some_and(|seen| seen.alive)
}
impl Goal for BassSchool {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn name(&self) -> &'static str {
        "NaturalistBassSchool"
    }
    fn flags(&self) -> u8 {
        0
    }
    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if mob.extra.followers > 0 {
            return false;
        }
        if follows(cx.w, mob.extra.leader) {
            return true;
        }
        if self.next_start > 0 {
            self.next_start -= 1;
            return false;
        }
        self.next_start = FollowFlockLeader::next_start_tick(mob);
        let area = mob.bounds().inflate(Vec3::new(8.0, 8.0, 8.0));
        let mut nearby: Vec<_> =
            cx.w.others_near(area)
                .filter(|seen| {
                    seen.alive && seen.kind.id == "naturalist:bass" && seen.bounds.intersects(area)
                })
                .collect();
        nearby.sort_by_key(|seen| {
            let own = seen.id == mob.id;
            let pos = if own { mob.position } else { seen.position };
            let section = pos
                .components()
                .map(|coordinate| (coordinate.floor() as i32) >> 4);
            let track = if own {
                (mob.extra.section, mob.extra.section_seq)
            } else {
                seen.section
            };
            let valid = track.0 == section && track.1 != 0;
            let [x, y, z] = section;
            (
                x,
                z,
                y,
                !valid,
                if valid { track.1 } else { u64::from(seen.id) },
            )
        });
        let joinable = |followers: i32| followers > 0 && followers + 1 < 5;
        let candidates: Vec<_> = nearby
            .into_iter()
            .filter_map(|seen| {
                let school = if seen.id == mob.id {
                    (mob.extra.leader, mob.extra.followers)
                } else {
                    seen.school
                };
                (joinable(school.1) || !follows(cx.w, school.0)).then_some((seen, school))
            })
            .collect();
        let (leader, followers, leader_tier) = candidates
            .iter()
            .find(|(_, (_, count))| joinable(*count))
            .map_or(
                (mob.id, mob.extra.followers, mob.extra.variant),
                |(seen, (_, count))| (seen.id, *count, seen.variant),
            );
        let room = (5 - followers - 1).max(0) as usize;
        for (seen, _) in candidates
            .into_iter()
            .filter(|(_, (leader, _))| !follows(cx.w, *leader))
            .take(room)
        {
            let tier = if seen.id == mob.id {
                mob.extra.variant
            } else {
                seen.variant
            };
            if seen.id == leader || leader_tier != tier || tier == 2 {
                continue;
            }
            if seen.id == mob.id {
                mob.extra.leader = Some(leader);
                cx.events.push(MobEvent::SchoolSize { leader, by: 1 });
            } else {
                cx.events.push(MobEvent::StartFollowing {
                    fish: seen.id,
                    leader,
                });
            }
        }
        follows(cx.w, mob.extra.leader)
    }
    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        mob.extra
            .leader
            .and_then(|id| cx.w.seen(id))
            .is_some_and(|leader| leader.alive && mob.distance_sqr(leader.position) <= 121.0)
    }
    fn start(&mut self, _: &mut Mob, _: &mut Cx) {
        self.recalc = 0;
    }
    fn stop(&mut self, mob: &mut Mob, cx: &mut Cx) {
        if let Some(leader) = mob.extra.leader.take() {
            cx.events.push(MobEvent::SchoolSize { leader, by: -1 });
        }
    }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        self.recalc -= 1;
        if self.recalc > 0 {
            return;
        }
        self.recalc = reduced(10);
        if let Some(leader) = mob
            .extra
            .leader
            .and_then(|id| cx.w.seen(id))
            .filter(|leader| leader.alive)
        {
            mob.nav_move_to_entity(
                cx.w,
                [leader.position.x, leader.position.y, leader.position.z],
                1,
                1.0,
            );
        }
    }
}
