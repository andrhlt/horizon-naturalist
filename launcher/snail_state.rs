use super::{
    Mob, MobEvent, MobWorld, Who, naturalist_snail_control::SurfaceControl,
    naturalist_snail_surface::Surface,
};
use horizon_core::{geometry::Vec3, random::RandomSource};

pub const KIND: &str = "naturalist:snail";
pub const APPEARANCE_META: u8 = 252;
pub const NORMAL_META: u8 = 253;

#[derive(Clone, Debug)]
pub struct SnailState {
    pub color: u8,
    pub hiding: bool,
    slime_ball_delay: Option<u32>,
    pub has_egg: bool,
    pub surface: Surface,
    pub control: SurfaceControl,
    pub crawl_target: Option<[i32; 3]>,
}

impl Default for SnailState {
    fn default() -> Self {
        Self {
            color: 12,
            hiding: false,
            slime_ball_delay: None,
            has_egg: false,
            surface: Surface::default(),
            control: SurfaceControl::default(),
            crawl_target: None,
        }
    }
}

pub fn snail(mob: &Mob) -> &SnailState {
    mob.ai.nav.__horizon_mixins_naturalist_snail_state()
}

pub fn snail_mut(mob: &mut Mob) -> &mut SnailState {
    mob.ai.nav.__horizon_mixins_naturalist_snail_state_mut()
}

pub fn check_hiding(mob: &mut Mob, world: &MobWorld) {
    let hiding = world.player.as_ref().is_some_and(|p| {
        if !p.alive
            || p.creative
            || p.spectator
            || !mob
                .bounds()
                .inflate(Vec3::new(5.0, 3.0, 5.0))
                .contains(p.position)
        {
            return false;
        }
        let range = (5.0 * p.visibility_percent(Some(KIND), world.items)).max(2.0);
        mob.distance_sqr(p.position) <= range * range && mob.sees(world, Who::Player)
    });
    snail_mut(mob).hiding = hiding;
}

pub fn tick(mob: &mut Mob, events: &mut Vec<MobEvent>) {
    if !mob.alive() {
        return;
    }
    let mut surface = std::mem::take(&mut snail_mut(mob).surface);
    surface.tick(mob);
    snail_mut(mob).surface = surface;
    if snail(mob).hiding {
        mob.velocity.x = 0.0;
        mob.velocity.z = 0.0;
    }
    if mob.is_baby() {
        return;
    }
    let delay = snail(mob).slime_ball_delay;
    let remaining =
        delay.unwrap_or_else(|| 12000 + mob.random.next_int_bound(1200).unwrap_or(0) as u32) - 1;
    if remaining != 0 {
        snail_mut(mob).slime_ball_delay = Some(remaining);
        return;
    }
    let at = [mob.position.x, mob.position.y, mob.position.z];
    if !mob.silent {
        let pitch = (mob.random.next_float() - mob.random.next_float()) * 0.2 + 1.0;
        events.push(MobEvent::Sound("entity.slime.squish_small", at, 1.0, pitch));
    }
    events.push(MobEvent::DropItem {
        mob: mob.id,
        item: "minecraft:slime_ball".into(),
        count: 1,
        at,
    });
    snail_mut(mob).slime_ball_delay = None;
}
