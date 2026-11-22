use horizon_client::mobs::goals::{self, Cx, Goal, LOOK, MOVE};
use horizon_client::mobs::{BreedExperience, Mob, MobEvent};

#[derive(Clone, Debug, Default)]
struct ButterflyBreed { partner: Option<u32>, love_time: i32 }
impl Goal for ButterflyBreed {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn name(&self) -> &'static str { "NaturalistButterflyBreed" }
    fn flags(&self) -> u8 { MOVE | LOOK }
    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if m.extra.in_love <= 0 { return false }
        let id = m.id;
        self.partner = cx.w.nearby(m.position, 8.0, move |s| {
            s.id != id && s.kind.id == "naturalist:butterfly" && s.in_love && !s.baby && !s.panicking
        }).find(|s| m.distance_sqr(s.position) <= 64.0).map(|s| s.id);
        self.partner.is_some()
    }
    fn can_continue(&mut self, _: &mut Mob, cx: &mut Cx) -> bool {
        self.partner.and_then(|id| cx.w.seen(id)).is_some_and(|s| s.alive && s.in_love && !s.panicking) && self.love_time < 60
    }
    fn stop(&mut self, _: &mut Mob, _: &mut Cx) { self.partner = None; self.love_time = 0; }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        let Some(partner) = self.partner.and_then(|id| cx.w.seen(id)).cloned() else { return };
        m.look_at_speed([partner.position.x, partner.eye_y, partner.position.z], 10.0, m.max_head_x_rot());
        m.nav_move_to_entity(cx.w, [partner.position.x, partner.position.y, partner.position.z], 1, 1.0);
        self.love_time += 1;
        if self.love_time >= goals::reduced(60) && m.distance_sqr(partner.position) < 9.0 {
            m.extra.in_love = 0;
            m.extra.age = 6000;
            cx.events.push(MobEvent::Breed { parent: m.id, partner: partner.id, equine: None, parent_variant: None, wolf_voice: None, experience: BreedExperience::Deferred });
        }
    }
}
pub(super) fn install(game: &mut super::Game<'_>, id: u32) {
    use horizon_core::nbt::Tag;
    let Some(mob) = game.mobs.iter_mut().find(|m| m.id == id && m.kind.id == "naturalist:butterfly") else { return };
    if !mob.ai.goals.has("NaturalistButterflyBreed") { mob.ai.goals.add(1, ButterflyBreed::default()); }
    if let Some(raw) = game.persist.raw.get(&id) {
        mob.extra.in_love = raw.get("InLove").and_then(horizon_world::datafix::dynamic::as_number).map_or(0, |n|n.int_value());
        mob.extra.love_cause = match raw.get("LoveCause") { Some(Tag::IntArray(words)) if words.len() == 4 => Some([words[0],words[1],words[2],words[3]]), _ => None };
    }
}
