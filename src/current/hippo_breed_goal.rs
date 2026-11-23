use horizon_client::mobs::{
    BreedExperience, Mob, MobEvent,
    goals::{self, Cx, Goal, LOOK, MOVE},
};

#[derive(Clone, Debug, Default)]
pub(crate) struct HippoBreed {
    partner: Option<u32>,
    love_time: i32,
}

impl Goal for HippoBreed {
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
    fn name(&self) -> &'static str {
        "NaturalistHippoBreed"
    }
    fn flags(&self) -> u8 {
        MOVE | LOOK
    }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if mob.is_baby() || mob.extra.in_love <= 0 || !mob.fluids.in_water {
            return false;
        }
        let id = mob.id;
        self.partner =
            cx.w.nearby(mob.position, 8.0, move |seen| {
                seen.id != id
                    && seen.kind.id == "naturalist:hippo"
                    && seen.in_love
                    && !seen.baby
                    && !seen.panicking
                    && seen.in_water
            })
            .find(|seen| mob.distance_sqr(seen.position) <= 64.0)
            .map(|seen| seen.id);
        self.partner.is_some()
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        !mob.is_baby()
            && mob.fluids.in_water
            && self
                .partner
                .and_then(|id| cx.w.seen(id))
                .is_some_and(|seen| seen.alive && seen.in_love && !seen.panicking && seen.in_water)
            && self.love_time < 60
    }

    fn stop(&mut self, _: &mut Mob, _: &mut Cx) {
        self.partner = None;
        self.love_time = 0;
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(partner) = self.partner.and_then(|id| cx.w.seen(id)).cloned() else {
            return;
        };
        mob.look_at_speed(
            [partner.position.x, partner.eye_y, partner.position.z],
            10.0,
            mob.max_head_x_rot(),
        );
        mob.nav_move_to_entity(
            cx.w,
            [partner.position.x, partner.position.y, partner.position.z],
            1,
            1.0,
        );
        self.love_time += 1;
        if self.love_time >= goals::reduced(60) && mob.distance_sqr(partner.position) < 9.0 {
            let experience = if cx.w.asks_baby_spawn {
                BreedExperience::Deferred
            } else {
                BreedExperience::Drawn(horizon_client::mobs::breeding_experience(
                    mob,
                    cx.w.mob_drops,
                ))
            };
            mob.extra.in_love = 0;
            mob.extra.age = 6000;
            cx.events.push(MobEvent::Breed {
                parent: mob.id,
                partner: partner.id,
                equine: None,
                parent_variant: None,
                wolf_voice: None,
                experience,
            });
            self.love_time = 60;
        }
    }
}
