use super::{Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_deer_variants::appearance_texture, naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target}};
use horizon_client::mobs::{Mob, model_layers::ModelPart, nav::naturalist_giant_isopod as isopod};
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};

struct Pose {
    clocks: [AnimationClock; 5],
    tick: i32,
    last_seen: Instant,
    hiding: bool,
    end_tick: i32,
}

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(giant_isopod_models::adult());
    static POSES: RefCell<HashMap<u32, Pose>> = RefCell::new(HashMap::new());
}

impl Game<'_> {
    pub(super) fn draw_naturalist_giant_isopod(&self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw) -> bool {
        use giant_isopod_animations::{ADULT_IDLE, ADULT_WALK, ADULT_SWIM, ADULT_HIDE_START, ADULT_HIDE_END};

        let texture = appearance_texture(self, mob.id, render.baby);
        let texture = texture.as_deref().unwrap_or("naturalist:entity/giant_isopod/brown");
        let hiding = if self.remote.is_some() {
            !mob.is_baby() && self.player_view().is_some_and(|player| isopod::player_near(mob, &player))
        } else {
            isopod::hiding(mob)
        };
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let tick = render.age.floor() as i32;
                if poses.len() > 2048 {
                    poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
                }
                let fresh = || Pose { clocks: Default::default(), tick: -1, last_seen: now, hiding: false, end_tick: -1 };
                let pose = poses.entry(render.id).or_insert_with(fresh);
                if tick < pose.tick { *pose = fresh(); }
                pose.last_seen = now;
                if pose.tick != tick {
                    if hiding {
                        pose.end_tick = -1;
                    } else if pose.hiding {
                        pose.end_tick = tick + 14;
                    }
                    let ending = !hiding && tick < pose.end_tick;
                    let swimming = !hiding && !ending && !render.on_ground && mob.fluids.in_water;
                    let walking = !hiding && !ending && !swimming && render.walk_speed > 0.001;
                    let idle = !hiding && !ending && !swimming && !walking;
                    pose.clocks[0].advance_with_rate(idle, tick, 0.5);
                    pose.clocks[1].advance_with_rate(walking, tick, 0.5);
                    pose.clocks[2].advance_with_rate(swimming, tick, 0.5);
                    pose.clocks[3].advance_with_rate(hiding, tick, 1.0);
                    pose.clocks[4].advance_with_rate(ending, tick, 1.0);
                    pose.tick = tick;
                    pose.hiding = hiding;
                }
                let speed = render.walk_speed;
                for (clock, (clip, rate, weight)) in pose.clocks.iter_mut().zip([
                    (&ADULT_IDLE, 1.0, 1.0 - speed.min(1.0)),
                    (&ADULT_WALK, (speed * 6.0).min(1.0), 1.0),
                    (&ADULT_SWIM, (speed * 2.0).min(1.0), 1.0),
                    (&ADULT_HIDE_START, 1.0, 1.0),
                    (&ADULT_HIDE_END, 1.0, 1.0),
                ]) {
                    clock.draw(clip, model, render.age, render.partial, rate, weight);
                }
            });
            if render.baby { draw.scale([0.5, 0.5, 0.5]); }
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            self.native_draw_model_part(draw, model, texture, if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF })
        })
    }
}
