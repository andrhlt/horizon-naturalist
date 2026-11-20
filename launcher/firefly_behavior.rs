use super::{Cx, Float, Goal, JUMP, MOVE, Mob, MobWorld, MoveToBlock, goal_clone, reduced};
use crate::mobs::MobEvent;
use horizon_core::random::{LegacyRandom, RandomSource};

#[derive(Clone, Debug, Default)]
pub struct Glow {
    ticks: u8,
    frame: u8,
    sun_ticks: u16,
    light: Light,
}

#[derive(Clone, Copy, Debug, Default)]
struct Light {
    sky_darken: i32,
    fixed_time: bool,
    ambient: f32,
}

impl Light {
    fn raw(self, packed: u8) -> i32 {
        (i32::from(packed >> 4) - self.sky_darken).max(i32::from(packed & 15))
    }
}

pub fn set_level_light(m: &mut Mob, sky_darken: i32, fixed_time: bool, ambient: f32) {
    m.ai.nav.__horizon_mixins_naturalist_firefly_glow_mut().light =
        Light { sky_darken, fixed_time, ambient };
}

pub fn glow_frame(m: &Mob) -> (u8, bool) {
    let glow = m.ai.nav.__horizon_mixins_naturalist_firefly_glow();
    if glow.ticks == 0 {
        (0, false)
    } else {
        (glow.frame.min(29), true)
    }
}

// The host sends the sprite index and emissive bit. This data expires with
// the tracked entity and is never written to the save.
pub const GLOW_META: u8 = 250;

pub fn glow_meta(m: &Mob) -> i8 {
    let (frame, emissive) = glow_frame(m);
    (frame | if emissive { 32 } else { 0 }) as i8
}

pub fn apply_glow_meta(m: &mut Mob, value: i8) {
    let glow = m.ai.nav.__horizon_mixins_naturalist_firefly_glow_mut();
    let value = value as u8;
    glow.ticks = u8::from(value & 32 != 0);
    glow.frame = (value & 31).min(29);
}

pub fn register(m: &mut Mob) {
    super::naturalist_insect_goals::configure_small_flyer(m);
    m.ai.goals.add(1, HideInGrass(MoveToBlock::new(f64::from(1.2_f32), 10, 4)));
    m.ai.goals
        .add(2, super::naturalist_insect_goals::FlyingWander);
    m.ai.goals.add(3, Float);
}

pub fn step(m: &mut Mob, w: &MobWorld, random: &mut LegacyRandom, events: &mut Vec<MobEvent>) {
    if m.removed {
        return;
    }
    let sunny = in_bright_sunlight(m, w);
    let [x, y, z] = [m.position.x, m.position.y, m.position.z].map(|value| value.floor() as i32);
    let light = w.world.light(x, y, z);
    let glow = m.ai.nav.__horizon_mixins_naturalist_firefly_glow_mut();
    let dark = (!glow.light.fixed_time && glow.light.sky_darken >= 4) || glow.light.raw(light) < 8;
    if glow.ticks > 0 {
        glow.ticks -= 1;
        glow.frame += 1;
    } else {
        glow.frame = 0;
    }
    if dark && f64::from(m.random.next_float()) <= 0.01 && glow.ticks == 0 {
        glow.ticks = 40 + m.random.next_int_bound(20).unwrap_or(0) as u8;
        glow.frame = 0;
    }
    if sunny {
        glow.sun_ticks += 1;
        if glow.sun_ticks > 600 {
            let at = [f64::from(x) + 0.5, f64::from(y), f64::from(z) + 0.5];
            for _ in 0..20 {
                let spread = std::array::from_fn(|_| m.random.next_gaussian() * 0.02);
                events.push(MobEvent::LevelParticles {
                    kind: "poof",
                    at,
                    spread,
                    speed: f64::from(0.15_f32),
                    count: 1,
                });
            }
            hide(m, random, [x, y, z], events);
        }
    }
}

fn in_bright_sunlight(m: &Mob, w: &MobWorld) -> bool {
    let light = m.ai.nav.__horizon_mixins_naturalist_firefly_glow().light;
    if light.fixed_time || light.sky_darken >= 4 || m.custom_name.is_some() {
        return false;
    }
    let packed = w.world.light(
        m.position.x.floor() as i32,
        m.eye_y().floor() as i32,
        m.position.z.floor() as i32,
    );
    let raw = light.raw(packed) as f32 / 15.0;
    let brightness = raw / (4.0 - 3.0 * raw);
    brightness + light.ambient * (1.0 - brightness) > 0.5
}

fn hide(m: &mut Mob, random: &mut LegacyRandom, pos: [i32; 3], events: &mut Vec<MobEvent>) {
    events.push(MobEvent::Sound(
        "naturalist:entity.firefly.hide",
        pos.map(|value| f64::from(value) + 0.5),
        0.7,
        0.9 + random.next_float() * 0.2,
    ));
    m.removed = true;
}

#[derive(Clone, Debug)]
struct HideInGrass(MoveToBlock);

fn valid(w: &MobWorld, [x, y, z]: [i32; 3]) -> bool {
    matches!(
        w.block_name(x, y, z),
        "minecraft:short_grass" | "minecraft:fern" | "minecraft:tall_grass"
    )
}

impl Goal for HideInGrass {
    goal_clone!();

    fn flags(&self) -> u8 {
        MOVE | JUMP
    }

    fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
        if m.persistent || !in_bright_sunlight(m, cx.w) {
            return false;
        }
        if self.0.next_start > 0 {
            self.0.next_start -= 1;
            return false;
        }
        self.0.next_start = reduced(200 + m.random.next_int_bound(200).unwrap_or(0));
        self.0.find_nearest(m, cx.w, valid)
    }

    fn can_continue(&mut self, _: &mut Mob, cx: &mut Cx) -> bool {
        self.0.can_continue(cx.w, valid)
    }

    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.0.start(m, cx.w);
    }

    fn every_tick(&self) -> bool {
        true
    }

    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        // Java initially navigates above the grass, then tests and repaths to the grass itself.
        self.0.block[1] -= 1;
        self.0.tick(m, cx.w);
        self.0.block[1] += 1;
        if self.0.reached {
            let at = [m.position.x, m.position.y, m.position.z];
            cx.events.push(MobEvent::LevelParticles {
                kind: "naturalist:firefly_hiding",
                at,
                spread: [m.width(), m.height(), m.width()].map(|length| f64::from(length / 4.0)),
                speed: 0.05,
                count: 50,
            });
            hide(m, cx.lr, at.map(|value| value.floor() as i32), cx.events);
        }
    }
}
