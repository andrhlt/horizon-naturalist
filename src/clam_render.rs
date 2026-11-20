use super::{Game, native_block_entity_draw::BlockEntityDraw, native_mob_draw::NativeMobRender,
    naturalist_deer_variants::clam_appearance, naturalist_preview::{AnimationClock, Channel, Clip, Interpolation, Target}};
use horizon_assets::item_model::DisplayContext;
use horizon_client::mobs::{Mob, model_layers::ModelPart};
use std::{cell::RefCell, collections::HashMap, time::{Duration, Instant}};

struct Pose {
    clocks: [AnimationClock; 4],
    tick: i32,
    last_seen: Instant,
    open: bool,
    transition_end: i32,
}

thread_local! {
    static MODEL: RefCell<ModelPart> = RefCell::new(clam_models::adult());
    static POSES: RefCell<HashMap<u32, Pose>> = RefCell::new(HashMap::new());
}

impl Game<'_> {
    pub(super) fn draw_naturalist_clam(&self, mob: &Mob, render: &NativeMobRender<'_>, draw: &mut BlockEntityDraw) -> bool {
        use clam_animations::{ADULT_OPEN, ADULT_CLOSE, ADULT_IDLE_OPEN, ADULT_IDLE_CLOSED};

        let (texture, open, _) = clam_appearance(self, mob);
        MODEL.with_borrow_mut(|model| {
            model.reset_all_poses();
            POSES.with_borrow_mut(|poses| {
                let now = Instant::now();
                let tick = render.age.floor() as i32;
                if poses.len() > 2048 {
                    poses.retain(|_, pose| now.duration_since(pose.last_seen) < Duration::from_secs(30));
                }
                let new_pose = || Pose { clocks: Default::default(), tick: -1, last_seen: now, open, transition_end: -1 };
                let pose = poses.entry(render.id).or_insert_with(new_pose);
                if tick < pose.tick { *pose = new_pose(); }
                pose.last_seen = now;
                if pose.tick != tick {
                    if pose.open != open {
                        pose.open = open;
                        pose.transition_end = tick + if open { 25 } else { 10 };
                    }
                    let changing = tick < pose.transition_end;
                    for (clock, active) in pose.clocks.iter_mut().zip([
                        open && changing, !open && changing, open && !changing, !open && !changing,
                    ]) {
                        clock.advance_with_rate(active, tick, 0.5);
                    }
                    pose.tick = tick;
                }
                for (clock, clip) in pose.clocks.iter_mut().zip([&ADULT_OPEN, &ADULT_CLOSE, &ADULT_IDLE_OPEN, &ADULT_IDLE_CLOSED]) {
                    clock.draw(clip, model, render.age, render.partial, 1.0, 1.0);
                }
            });
            draw.push();
            draw.scale([1.0, -1.0, -1.0]);
            draw.translate([0.0, -1.501, 0.0]);
            let drawn = self.native_draw_model_part(draw, model, &texture, if render.hurt_time > 0 { 0xFF7070 } else { 0xFFFFFF });
            draw.pop();
            if let Some(item) = mob.extra.main_hand_stack().filter(|_| open) {
                draw.push();
                draw.translate([0.0, 0.7 + (render.age * 0.1).sin() * 0.2, 0.0]);
                draw.rotate_about([0.0, 1.0, 0.0], self.yaw - render.body_yaw, [0.0; 3]);
                draw.rotate_about([1.0, 0.0, 0.0], self.pitch, [0.0; 3]);
                draw.scale([0.8; 3]);
                self.native_draw_item(draw, item, DisplayContext::Fixed, None, mob.id as i32);
                draw.pop();
            }
            drawn
        })
    }
}
