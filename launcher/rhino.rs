use super::{Damage, Mob, MobEvent, MobWorld, Who, goals::{Cx, Goal, LOOK, MOVE}, nav::Navigation, path::Path};
use horizon_core::geometry::Vec3;

pub const KIND: &str = "naturalist:rhino";
pub const PUMPKIN: &str = "naturalist:rhino_pumpkin_target";

#[derive(Clone, Debug, Default)]
pub struct State {
    cooldown: i32,
    stun: i32,
    preparing: bool,
}

fn state(nav: &Navigation) -> &State {
    nav.__horizon_mixins_naturalist_rhino_state()
}

fn state_mut(nav: &mut Navigation) -> &mut State {
    nav.__horizon_mixins_naturalist_rhino_state_mut()
}

pub fn stun_ticks(nav: &Navigation) -> i32 {
    state(nav).stun
}

pub fn put_stun(nav: &mut Navigation, remaining: i32) {
    state_mut(nav).stun = remaining;
}

pub fn preparing(nav: &Navigation) -> bool {
    state(nav).preparing
}

pub fn stun(mob: &mut Mob) {
    let state = state_mut(&mut mob.ai.nav);
    state.stun = 60;
    state.cooldown = 42;
    mob.nav_stop();
}

pub(super) fn tick_stun(mob: &mut Mob) {
    let state = state_mut(&mut mob.ai.nav);
    let stunned = state.stun > 0;
    if stunned {
        state.stun -= 1;
        mob.ai.jump = false;
    }
    super::finalize::set_base(mob, super::finalize::Attr::MovementSpeed, if stunned { 0.0 } else { 0.2 });
}

pub(super) fn protects_baby(mob: &Mob, world: &MobWorld) -> bool {
    !mob.is_baby() && world.player.as_ref().is_some_and(|player| {
        (player.position.y - mob.position.y).abs() < 3.0
    }) && {
        let bounds = mob.bounds().inflate(Vec3::new(8.0, 4.0, 8.0));
        world.others_near(bounds).any(|seen| {
            seen.alive && seen.baby && seen.effects.contains(&KIND) && bounds.intersects(seen.bounds)
        })
    }
}

#[derive(Clone, Debug)]
pub(super) struct Prepare;

impl Goal for Prepare {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { MOVE | LOOK }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        let target = mob.target_info(cx.w);
        if target.is_none_or(|target| !target.alive || (target.position.y - mob.position.y).abs() >= 3.0)
            || stun_ticks(&mob.ai.nav) > 0
        {
            state_mut(&mut mob.ai.nav).cooldown = 42;
            return false;
        }
        mob.ai.target == Some(Who::Player) && state(&mob.ai.nav).cooldown > 0
    }

    fn start(&mut self, mob: &mut Mob, _: &mut Cx) {
        let state = state_mut(&mut mob.ai.nav);
        state.cooldown = 42;
        state.preparing = true;
    }

    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        state_mut(&mut mob.ai.nav).preparing = false;
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(target) = mob.target_info(cx.w) else { return; };
        mob.look_at([target.position.x, target.eye_y, target.position.z]);
        let state = state_mut(&mut mob.ai.nav);
        state.cooldown = (state.cooldown - 1).max(0);
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct Charge {
    path: Option<Path>,
    target: [i32; 3],
    direction: Vec3,
}

impl Charge {
    fn allowed(mob: &Mob, world: &MobWorld) -> bool {
        state(&mob.ai.nav).cooldown == 0 && stun_ticks(&mob.ai.nav) == 0
            && mob.target_info(world).is_some_and(|target| target.alive)
    }
}

impl Goal for Charge {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn flags(&self) -> u8 { MOVE | LOOK }
    fn every_tick(&self) -> bool { true }

    fn can_use(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        if mob.ai.target != Some(Who::Player) || !Self::allowed(mob, cx.w) {
            return false;
        }
        self.path = mob.target_info(cx.w).and_then(|target| {
            mob.create_path_to_entity(cx.w, target.position.components(), 0)
        });
        self.path.is_some()
    }

    fn can_continue(&mut self, mob: &mut Mob, cx: &mut Cx) -> bool {
        Self::allowed(mob, cx.w) && !mob.ai.nav.is_done()
    }

    fn start(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(path) = self.path.take() else { return; };
        self.target = path.target;
        self.direction = Vec3::new(
            mob.position.x.floor() - f64::from(self.target[0]), 0.0,
            mob.position.z.floor() - f64::from(self.target[2]),
        ).normalize();
        mob.nav_move_to_path(cx.w, Some(path), 2.5);
        mob.ai.aggressive = true;
    }

    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) {
        state_mut(&mut mob.ai.nav).cooldown = 42;
        mob.nav_stop();
    }

    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        mob.look_at(self.target.map(|value| f64::from(value) + 0.5));
        if mob.horizontal_collision && mob.on_ground {
            mob.ai.jump = true;
        }
        let bounds = mob.bounds();
        if cx.w.mob_griefing {
            let box_ = bounds.inflate(Vec3::new(0.2, 0.2, 0.2));
            let [x0, y0, z0] = box_.min_position().components().map(|v| v.floor() as i32);
            let [x1, y1, z1] = box_.max_position().components().map(|v| v.floor() as i32);
            for y in y0..=y1 {
                for z in z0..=z1 {
                    for x in x0..=x1 {
                        if cx.w.has_tag("naturalist:rhino_charge_breakable", cx.w.block_name(x, y, z)) {
                            cx.events.push(MobEvent::BreakBlock { pos: [x, y, z], drop: true });
                        }
                    }
                }
            }
        }
        let at = mob.position.components();
        cx.events.push(MobEvent::Particles { kind: "minecraft:campfire_cosy_smoke", at, count: 5 });
        if cx.w.game_time % 2 == 0 {
            let pitch = mob.voice_pitch();
            if !mob.silent {
                cx.events.push(MobEvent::Sound("entity.hoglin.step", at, 0.5, pitch));
            }
        }
        let player = cx.w.player.as_ref().is_some_and(|player| {
            player.alive && !player.creative && !player.spectator && bounds.intersects(player.bounds)
                && !mob.spares_target(cx.w, Who::Player) && mob.sees(cx.w, Who::Player)
        });
        let other = cx.w.others_near(bounds).find(|other| {
            let who = Who::Mob(other.id);
            other.id != mob.id && other.alive && bounds.intersects(other.bounds)
                && !mob.spares_target(cx.w, who) && mob.sees(cx.w, who)
        });
        let target = if player {
            Who::Player
        } else if let Some(other) = other.filter(|other| !other.effects.contains(&KIND)) {
            Who::Mob(other.id)
        } else {
            return;
        };
        let amount = mob.attrs.attack_damage;
        let source = Damage::mob(mob.id, at).with_species(mob.species());
        cx.events.push(match target {
            Who::Player => MobEvent::AttackPlayer(amount, at),
            Who::Mob(target) => MobEvent::AttackMob { target, amount, source },
        });
        let power = f64::from((mob.ai.speed * 1.65).clamp(0.2, 3.0)) * 2.0;
        cx.events.push(MobEvent::Knockback {
            target, power, x: self.direction.x, z: self.direction.z, source, apply_blocking: Some(1.0),
        });
        mob.swing();
        if Some(target) == mob.ai.target {
            self.stop(mob, cx);
        }
    }
}
