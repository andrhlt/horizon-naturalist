//! Bass models, keyframes, and interpolation adapted from Naturalist 2.0.7 (MIT).

use crate::duck_appearance::DuckAppearance;
use mod_api::{BlockEntityDrawContext, EntityRenderState, EntityRenderer, ModelPart};
use std::{collections::HashMap, sync::Mutex};
#[derive(Clone, Copy)]
enum Target {
    Position,
    Rotation,
    Scale,
}
enum Interpolation {
    Linear,
    CatmullRom,
    Mixed(&'static [usize]),
}
struct Channel {
    bone: &'static str,
    target: Target,
    interpolation: Interpolation,
    keys: &'static [(f32, [f32; 3])],
}
struct Clip {
    duration: f32,
    loops: bool,
    channels: &'static [Channel],
}

fn bone<'a>(part: &'a mut ModelPart, name: &str) -> Option<&'a mut ModelPart> {
    for (child_name, child) in part.children_mut() {
        if child_name == name {
            return Some(child);
        }
        if let Some(found) = bone(child, name) {
            return Some(found);
        }
    }
    None
}
impl Clip {
    fn pose(&self, model: &mut ModelPart, time: f32, weight: f32) {
        let time = if self.loops && self.duration > 0.0 {
            time.rem_euclid(self.duration)
        } else {
            time
        };
        for channel in self.channels {
            let Some(part) = bone(model, channel.bone) else {
                continue;
            };
            let end = channel
                .keys
                .partition_point(|(key, _)| *key < time)
                .min(channel.keys.len() - 1);
            let first = end.saturating_sub(1);
            let (start_time, start) = channel.keys[first];
            let (end_time, end_value) = channel.keys[end];
            let blend = if end_time == start_time {
                0.0
            } else {
                ((time - start_time) / (end_time - start_time)).clamp(0.0, 1.0)
            };
            let catmull = match channel.interpolation {
                Interpolation::Linear => false,
                Interpolation::CatmullRom => true,
                Interpolation::Mixed(keys) => keys.contains(&end),
            };
            let xyz = if catmull {
                let before = channel.keys[first.saturating_sub(1)].1;
                let after = channel.keys[(end + 1).min(channel.keys.len() - 1)].1;
                std::array::from_fn(|i| {
                    let [a, b, c, d] = [before[i], start[i], end_value[i], after[i]];
                    0.5 * (2.0 * b
                        + (-a + c) * blend
                        + (2.0 * a - 5.0 * b + 4.0 * c - d) * blend * blend
                        + (-a + 3.0 * b - 3.0 * c + d) * blend * blend * blend)
                })
            } else {
                std::array::from_fn(|i| start[i] + (end_value[i] - start[i]) * blend)
            };
            match channel.target {
                Target::Position => {
                    part.offset_pos([xyz[0] * weight, -xyz[1] * weight, xyz[2] * weight])
                }
                Target::Rotation => {
                    part.offset_rotation(xyz.map(|angle| angle.to_radians() * weight))
                }
                Target::Scale => part.offset_scale(xyz.map(|scale| (scale - 1.0) * weight)),
            }
        }
    }
}

#[derive(Default)]
struct AnimationClock {
    previous_factor: f32,
    factor: f32,
    phase_ms: i64,
    last_ms: Option<i64>,
}
impl AnimationClock {
    fn advance(&mut self, active: bool, tick: i32, rate: f32) {
        self.previous_factor = self.factor;
        let target = if active { 1.0 } else { 0.0 };
        self.factor = (self.factor + (target - self.factor) * rate).clamp(0.0, 1.0);
        if active && self.last_ms.is_none() {
            self.last_ms = Some(i64::from(tick) * 50);
            self.phase_ms = 0;
        } else if !active && self.factor < 0.001 {
            self.last_ms = None;
        }
    }
    fn draw(
        &mut self,
        clip: &Clip,
        model: &mut ModelPart,
        age: f32,
        partial: f32,
        speed: f32,
        scale: f32,
    ) {
        let weight =
            (self.previous_factor + partial * (self.factor - self.previous_factor)) * scale;
        if weight <= 0.05 {
            return;
        }
        if let Some(last) = &mut self.last_ms {
            let now = (age * 50.0).floor() as i64;
            self.phase_ms += ((now - *last) as f32 * speed) as i64;
            *last = now;
        }
        clip.pose(model, self.phase_ms as f32 / 1000.0, weight);
    }
}

#[path = "duck_animations.rs"] mod animations;
#[path = "duck_models.rs"] #[allow(unused_variables, non_snake_case)] mod models;
struct Pose { handle:mod_api::EntityRef, tick:i32, clocks:[AnimationClock;6], appearance:Option<DuckAppearance>, height:Option<f32>, invisible:bool }
impl Pose {fn new(handle:mod_api::EntityRef)->Self {Self{handle,tick:-1,clocks:Default::default(),appearance:None,height:None,invisible:false}}}
pub(crate) struct DuckRenderer { adult:ModelPart, baby:ModelPart, poses:Mutex<HashMap<u32,Pose>> }
impl DuckRenderer {
 pub fn new()->Self { Self{adult:models::adult(),baby:models::baby(),poses:Mutex::new(HashMap::new())} }
 pub fn observe(&self,handle:mod_api::EntityRef,tick:i32,moving:bool,height:Option<f32>,invisible:bool,appearance:Option<DuckAppearance>){
  let mut poses=self.poses.lock().unwrap_or_else(std::sync::PoisonError::into_inner);let p=poses.entry(handle.id()).or_insert_with(||Pose::new(handle));
  if p.handle!=handle || tick<p.tick {*p=Pose::new(handle);}
  if tick>p.tick { if let Some(a)=appearance.as_ref(){ let swimming=!a.sitting&&a.in_water;let walking=!a.sitting&&!swimming&&moving;let active=[a.sitting,!a.sitting&&!swimming&&!moving,swimming,walking&&!a.sprinting,walking&&a.sprinting,!a.on_ground&&!a.in_water];for (index,(clock,state)) in p.clocks.iter_mut().zip(active).enumerate(){clock.advance(state,tick,if index==0 {0.25} else {0.5});} } p.tick=tick; }
  p.appearance=appearance;p.height=height;p.invisible=invisible;
 }
 pub fn retain_live(&self,host:&dyn mod_api::ClientHost){self.poses.lock().unwrap_or_else(std::sync::PoisonError::into_inner).retain(|_,p|p.handle.position(host).is_some());}
 pub fn clear(&self){self.poses.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clear();}
}
impl EntityRenderer for DuckRenderer {
 fn submit(&self,entity:&EntityRenderState,draw:&mut dyn BlockEntityDrawContext)->Result<(),String>{
  let Some(living)=entity.living else{return Ok(());};
  let mut model=if living.is_baby {self.baby.clone()} else {self.adult.clone()};
  let mut poses=self.poses.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
  let a=poses.get(&entity.id).and_then(|p|p.appearance.clone()).unwrap_or_default();
  let height=poses.get(&entity.id).and_then(|p|p.height);let invisible=poses.get(&entity.id).is_some_and(|p|p.invisible);
  let gait=((8.64*a.speed).min(1.0)*0.65).max(0.05);let walk=(living.walk_animation_speed/gait).clamp(0.4,2.0);let swim=(living.walk_animation_speed/0.5).clamp(0.4,2.0);
  let clips=if living.is_baby {[&animations::BABY_SIT_IDLE,&animations::BABY_IDLE,&animations::BABY_SWIM,&animations::BABY_WALK,&animations::BABY_WALK,&animations::ADULT_FLAP]} else {[&animations::ADULT_SIT,&animations::ADULT_IDLE,&animations::ADULT_SWIM,&animations::ADULT_WALK,&animations::ADULT_WALK,&animations::ADULT_FLAP]};
  if let Some(p)=poses.get_mut(&entity.id) {let rates=[1.0,1.0,swim,1.5*walk,2.0*walk,1.0];let weights=[1.0,1.0-(living.walk_animation_speed*2.5).min(1.0),1.0,1.0,1.0,if living.is_baby{0.0}else{1.0}];for i in 0..6 {p.clocks[i].draw(clips[i],&mut model,entity.age,entity.partial,rates[i],weights[i]);}}
  drop(poses);
  if let Some(neck)=bone(&mut model,"neck"){neck.x_rot+=living.x_rot.to_radians();neck.y_rot+=living.y_rot.to_radians();}
  draw.scale([living.scale;3]);draw.rotate_about([0.0,1.0,0.0],180.0-living.body_rot,[0.0;3]);
  if living.death_time>0.0 {let progress=(((living.death_time-1.0)/20.0*1.6).max(0.0)).sqrt().min(1.0);draw.rotate_about([0.0,0.0,1.0],progress*90.0,[0.0;3]);} else if living.is_upside_down && let Some(h)=height {draw.translate([0.0,h+0.1,0.0]);draw.rotate_about([0.0,0.0,1.0],180.0,[0.0;3]);}
  draw.scale([1.0,-1.0,-1.0]);draw.translate([0.0,-1.501,0.0]);
  let texture=if living.is_baby{a.baby_texture.as_str()}else{a.adult_texture.as_str()};
  if !draw.model_part(&model,texture,if living.has_red_overlay{0xFF7070}else{0xFFFFFF}){return Err(format!("missing duck texture {texture}"));}
  const COLORS:[&str;16]=["white","orange","magenta","light_blue","yellow","lime","pink","gray","light_gray","cyan","purple","blue","brown","green","red","black"];
  if !invisible && let Ok(i)=usize::try_from(a.color) && let Some(color)=COLORS.get(i){let overlay=format!("naturalist:entity/duck/dye/{color}");if !draw.model_part(&model,&overlay,0xFFFFFF){return Err(format!("missing duck dye texture {overlay}"));}}
  Ok(())
 }
}
#[cfg(test)]
mod tests {
 use super::*;
 #[derive(Default)] struct Draw(Vec<String>);
 impl BlockEntityDrawContext for Draw {
  fn rotate_about(&mut self,_:[f32;3],_:f32,_:[f32;3]){}
  fn scale(&mut self,_:[f32;3]){} fn translate(&mut self,_:[f32;3]){}
  fn model_part(&mut self,_:&ModelPart,s:&str,_:u32)->bool {self.0.push(s.to_owned());true}
 }
 #[test] fn dye_overlay_draws_on_the_posed_duck_model_and_clears(){
  let r=DuckRenderer::new(); let id=mod_api::EntityRef::new(mod_api::WorldEpoch::new(1),42);
  let living:mod_api::LivingRenderState=mod_api::__private::built::LivingRenderState {body_rot:0.0,y_rot:0.0,x_rot:0.0,death_time:0.0,walk_animation_pos:0.0,walk_animation_speed:0.0,scale:1.0,age_scale:1.0,is_upside_down:false,is_baby:false,is_in_water:false,has_red_overlay:false,pose:mod_api::Pose::Standing,bed_orientation:None}.into();
  let frame:EntityRenderState=mod_api::__private::built::EntityRenderState{id:42,partial:0.5,age:10.5,yaw:0.0,pitch:0.0,living:Some(living)}.into();
  r.observe(id,10,false,Some(1.0),false,Some(DuckAppearance{color:14,..Default::default()}));let mut d=Draw::default();r.submit(&frame,&mut d).unwrap();assert_eq!(d.0,["naturalist:entity/duck/duck","naturalist:entity/duck/dye/red"]);
  r.observe(id,11,false,Some(1.0),false,Some(DuckAppearance::default()));let mut d=Draw::default();r.submit(&frame,&mut d).unwrap();assert_eq!(d.0,["naturalist:entity/duck/duck"]);
 }
 #[test] fn all_overlays_and_texture_selection(){
  let r=DuckRenderer::new();let id=mod_api::EntityRef::new(mod_api::WorldEpoch::new(2),42);
  let names=["white","orange","magenta","light_blue","yellow","lime","pink","gray","light_gray","cyan","purple","blue","brown","green","red","black"];
  for baby in [false,true] {for (i,name) in names.iter().enumerate(){
   let living:mod_api::LivingRenderState=mod_api::__private::built::LivingRenderState {body_rot:0.0,y_rot:0.0,x_rot:0.0,death_time:0.0,walk_animation_pos:0.0,walk_animation_speed:0.0,scale:1.0,age_scale:if baby{0.5}else{1.0},is_upside_down:false,is_baby:baby,is_in_water:false,has_red_overlay:false,pose:mod_api::Pose::Standing,bed_orientation:None}.into();
   let frame:EntityRenderState=mod_api::__private::built::EntityRenderState{id:42,partial:0.5,age:10.5,yaw:0.0,pitch:0.0,living:Some(living)}.into();
   r.observe(id,i as i32+1,false,Some(1.0),false,Some(DuckAppearance{color:i as i32,..Default::default()}));let mut draw=Draw::default();r.submit(&frame,&mut draw).unwrap();
   assert_eq!(draw.0[0],if baby{"naturalist:entity/duck/duck_baby"}else{"naturalist:entity/duck/duck"});assert_eq!(draw.0[1],format!("naturalist:entity/duck/dye/{name}"));
  }}
 }

 #[test] fn supplied_variant_and_queso_texture_reach_draw(){
  let r=DuckRenderer::new();let id=mod_api::EntityRef::new(mod_api::WorldEpoch::new(3),42);
  for (baby,adult,baby_tex,want) in [(false,"naturalist:entity/duck/queso","naturalist:entity/duck/duck_baby","naturalist:entity/duck/queso"),(false,"example:entity/custom","example:entity/custom_baby","example:entity/custom"),(true,"example:entity/custom","example:entity/custom_baby","example:entity/custom_baby")] {
   let living:mod_api::LivingRenderState=mod_api::__private::built::LivingRenderState {body_rot:0.0,y_rot:0.0,x_rot:0.0,death_time:0.0,walk_animation_pos:0.0,walk_animation_speed:0.0,scale:1.0,age_scale:if baby{0.5}else{1.0},is_upside_down:false,is_baby:baby,is_in_water:false,has_red_overlay:false,pose:mod_api::Pose::Standing,bed_orientation:None}.into();
   let frame:EntityRenderState=mod_api::__private::built::EntityRenderState{id:42,partial:0.5,age:10.5,yaw:0.0,pitch:0.0,living:Some(living)}.into();
   r.observe(id,10,false,Some(1.0),false,Some(DuckAppearance{adult_texture:adult.into(),baby_texture:baby_tex.into(),..Default::default()}));let mut draw=Draw::default();r.submit(&frame,&mut draw).unwrap();assert_eq!(draw.0,[want]);
  }
 }

}
