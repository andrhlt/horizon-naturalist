use horizon_client::mobs::{breeding_experience, BreedExperience, Mob, MobEvent};
use horizon_client::mobs::goals::{self, Cx, Goal, LOOK, MOVE};
use horizon_core::random::RandomSource;

#[derive(Clone, Debug, Default)]
struct SameKindBreed {
    partner: Option<u32>,
    love_time: i32,
}

impl Goal for SameKindBreed {
    fn name(&self) -> &'static str { "NaturalistRatBreed" }
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { MOVE | LOOK }

    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if m.extra.in_love <= 0 { return false; }
        let id = m.id;
        let kind = m.kind.id.clone();
        self.partner = cx.w.nearby(m.position, 8.0, move |s| {
            s.id != id && s.kind.id == kind && s.in_love && !s.baby && !s.panicking
        })
        .find(|s| m.distance_sqr(s.position) <= 64.0)
        .map(|s| s.id);
        self.partner.is_some()
    }

    fn can_continue(&mut self, _: &mut Mob, cx: &mut Cx) -> bool {
        self.partner
            .and_then(|id| cx.w.seen(id))
            .is_some_and(|s| s.alive && s.in_love && !s.panicking)
            && self.love_time < 60
    }

    fn stop(&mut self, _: &mut Mob, _: &mut Cx) {
        self.partner = None;
        self.love_time = 0;
    }

    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        let Some(partner) = self.partner.and_then(|id| cx.w.seen(id)).cloned() else { return; };
        m.look_at_speed([partner.position.x, partner.eye_y, partner.position.z], 10.0, m.max_head_x_rot());
        m.nav_move_to_entity(cx.w, [partner.position.x, partner.position.y, partner.position.z], 1, 1.0);
        self.love_time += 1;
        if self.love_time >= goals::reduced(60) && m.distance_sqr(partner.position) < 9.0 {
            let experience = if !cx.w.asks_baby_spawn {
                BreedExperience::Drawn(breeding_experience(m, cx.w.mob_drops))
            } else {
                BreedExperience::Deferred
            };
            m.extra.in_love = 0;
            m.extra.age = 6000;
            cx.events.push(MobEvent::Breed {
                parent: m.id,
                partner: partner.id,
                equine: None,
                parent_variant: Some(m.random.next_boolean()),
                wolf_voice: None,
                experience,
            });
        }
    }
}

pub(crate) fn install(m: &mut Mob) {
    if !m.ai.goals.has("NaturalistRatBreed") {
        m.ai.goals.add(2, SameKindBreed::default());
    }
}
