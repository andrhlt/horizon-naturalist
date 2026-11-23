use horizon_client::mobs::goals::{self, Cx, Goal, LOOK, MOVE};
use horizon_client::mobs::{BreedExperience, Mob, MobEvent, breeding_experience};

#[derive(Clone, Debug, Default)]
struct SameZebraBreed {
    partner: Option<u32>,
    love_time: i32,
}

impl Goal for SameZebraBreed {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn name(&self) -> &'static str { "NaturalistZebraBreed" }
    fn flags(&self) -> u8 { MOVE | LOOK }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if mob.extra.in_love <= 0 { return false; }
        let id = mob.id;
        self.partner = cx.w.nearby(mob.position, 8.0, move |seen| {
            seen.id != id && seen.kind.id == "naturalist:zebra" && seen.in_love
                && !seen.baby && !seen.panicking && seen.alive
        }).find(|seen| mob.distance_sqr(seen.position) <= 64.0).map(|seen| seen.id);
        self.partner.is_some()
    }

    fn can_continue(&mut self, _: &mut Mob, cx: &mut Cx) -> bool {
        self.partner.and_then(|id| cx.w.seen(id))
            .is_some_and(|seen| seen.alive && seen.in_love && !seen.panicking)
            && self.love_time < 60
    }

    fn stop(&mut self, _: &mut Mob, _: &mut Cx) {
        self.partner = None;
        self.love_time = 0;
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(partner) = self.partner.and_then(|id| cx.w.seen(id)).cloned() else { return; };
        mob.look_at_speed([partner.position.x, partner.eye_y, partner.position.z], 10.0, mob.max_head_x_rot());
        mob.nav_move_to_entity(cx.w, [partner.position.x, partner.position.y, partner.position.z], 1, 1.0);
        self.love_time += 1;
        if self.love_time >= goals::reduced(60) && mob.distance_sqr(partner.position) < 9.0 {
            let equine = horizon_client::mobs::equine::breed_offspring(mob, &partner).map(Box::new);
            let experience = if cx.w.asks_baby_spawn {
                BreedExperience::Deferred
            } else {
                BreedExperience::Drawn(breeding_experience(mob, cx.w.mob_drops))
            };
            mob.extra.in_love = 0;
            mob.extra.age = 6000;
            cx.events.push(MobEvent::Breed {
                parent: mob.id,
                partner: partner.id,
                equine,
                parent_variant: None,
                wolf_voice: None,
                experience,
            });
        }
    }
}

pub fn install(game: &mut Game<'_>, entity: u32, kind: &str) {
    if kind != "naturalist:zebra" { return; }
    let Some(index) = game.mobs.iter().position(|mob| mob.id == entity) else { return; };
    if game.mobs[index].ai.goals.has("NaturalistZebraBreed") { return; }
    let Some(entity_ref) = game.entity(entity) else { return; };
    let Some(handle) = game.mob(entity_ref) else { return; };
    handle.remove_goal(game, "breed");
    if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
        mob.ai.goals.add(2, SameZebraBreed::default());
    }
}
