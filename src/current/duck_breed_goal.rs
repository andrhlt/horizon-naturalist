use horizon_client::mobs::goals::{self, Cx, Goal, LOOK, MOVE};
use horizon_client::mobs::{BreedExperience, Mob, MobEvent, breeding_experience};
use horizon_core::random::RandomSource;

#[derive(Clone, Debug, Default)]
struct DuckChickenBreed {
    partner: Option<u32>,
    love_time: i32,
}
impl Goal for DuckChickenBreed {
    fn name(&self) -> &'static str { "NaturalistDuckChickenBreed" }
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn flags(&self) -> u8 {
        MOVE | LOOK
    }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if m.extra.in_love <= 0 {
            return false;
        }
        let id = m.id;
        let kind = m.kind.id.clone();
        self.partner =
            cx.w.nearby(m.position, 8.0, move |s| {
                s.id != id && s.kind.id == kind && s.in_love && !s.baby && !s.panicking
            })
            .find(|s| m.distance_sqr(s.position) <= 64.0)
            .map(|s| s.id);
        self.partner.is_some()
    }
    fn can_continue(&mut self, _: &mut Mob, cx: &mut Cx) -> bool {
        self.partner
            .and_then(|p| cx.w.seen(p))
            .is_some_and(|s| s.alive && s.in_love && !s.panicking)
            && self.love_time < 60
    }
    fn stop(&mut self, _: &mut Mob, _: &mut Cx) {
        self.partner = None;
        self.love_time = 0;
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        let Some(p) = self.partner.and_then(|p| cx.w.seen(p)).cloned() else {
            return;
        };
        let x = m.max_head_x_rot();
        m.look_at_speed([p.position.x, p.eye_y, p.position.z], 10.0, x);
        m.nav_move_to_entity(cx.w, [p.position.x, p.position.y, p.position.z], 1, 1.0);
        self.love_time += 1;
        if self.love_time >= goals::reduced(60) && m.distance_sqr(p.position) < 9.0 {
            let parent_variant = Some(m.random.next_boolean());
            let experience = if !cx.w.asks_baby_spawn {
                BreedExperience::Drawn(breeding_experience(m, cx.w.mob_drops))
            } else {
                BreedExperience::Deferred
            };
            m.extra.in_love = 0;
            m.extra.age = 6000;
            cx.events.push(MobEvent::Breed {
                parent: m.id,
                partner: p.id,
                equine: None,
                parent_variant,
                wolf_voice: None,
                experience,
            });
        }
    }
}


pub(super) fn add(m: &mut Mob) { m.ai.goals.add(2, DuckChickenBreed::default()); }
