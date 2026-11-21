//! PetFollowOwnerGoal for Crab; movement follows Horizon's pinned FollowOwner.
use horizon_client::mobs::{Mob, MobWorld, goals::{Cx, Goal, MOVE, LOOK, reduced}};
use horizon_core::{geometry::{Aabb, Vec3}, random::RandomSource};
use std::sync::{Arc, RwLock};
#[derive(Clone, Debug)]
pub(super) struct OwnerView { pub position: Vec3, pub eye: f64, pub spectator: bool, pub uuid: [i32; 4] }
pub(super) type OwnerCell = Arc<RwLock<Option<OwnerView>>>;
fn default_owner_cell() -> OwnerCell { Arc::new(RwLock::new(None)) }
pub(super) static OWNER_VIEW: mod_api::AttachmentType<OwnerCell> = mod_api::AttachmentType {
    namespace:"naturalist", name:"crab_follow_owner_view", default:default_owner_cell,
    codec:None, copy_on_death:false, sync:false,
};
#[derive(Clone, Debug)]
pub(super) struct CrabFollow { recalc: i32, pub owner_view: OwnerCell }
impl CrabFollow { pub fn new(owner_view: OwnerCell) -> Self { Self { recalc:0, owner_view } } }
fn owner(m: &Mob, cell: &OwnerCell) -> Option<OwnerView> {
    let view=cell.read().ok()?.clone()?;
    (m.extra.tame && m.extra.owner == Some(view.uuid)).then_some(view)
}
fn unable(m: &Mob, view: &OwnerView) -> bool {
    m.extra.sitting || m.extra.passenger || m.extra.leash.is_some() || m.extra.leash_pending.is_some() || view.spectator
}
fn teleport(m: &mut Mob, w: &MobWorld<'_>, at: Vec3) {
    let base = [at.x.floor() as i32, at.y.floor() as i32, at.z.floor() as i32];
    for _ in 0..10 {
        let x = m.random.next_int_bound(7).unwrap_or(0) - 3;
        let z = m.random.next_int_bound(7).unwrap_or(0) - 3;
        if x.abs() < 2 && z.abs() < 2 { continue; }
        let y = m.random.next_int_bound(3).unwrap_or(0) - 1;
        let p = [base[0]+x,base[1]+y,base[2]+z];
        let below = w.facts.get(w.world.block(p[0],p[1]-1,p[2]));
        if !below.full_collision || below.name.ends_with("_leaves") { continue; }
        let here = w.facts.get(w.world.block(p[0],p[1],p[2]));
        if here.fluid != horizon_client::blocks::Fluid::None { continue; }
        let half = f64::from(m.width())/2.0;
        let at = Vec3::new(f64::from(p[0])+0.5,f64::from(p[1]),f64::from(p[2])+0.5);
        if !w.no_collision(Aabb::new(Vec3::new(at.x-half,at.y,at.z-half),Vec3::new(at.x+half,at.y+f64::from(m.height()),at.z+half))) { continue; }
        horizon_client::mobs::living::teleports::snap_to_with(m,at,w.section_writer.clone());
        m.nav_stop();
        return;
    }
}
impl Goal for CrabFollow {
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
    fn name(&self) -> &'static str { "NaturalistCrabFollowOwner" }
    fn flags(&self) -> u8 { MOVE | LOOK }
    fn can_use(&mut self,m:&mut Mob,cx:&mut Cx) -> bool {
        owner(m,&self.owner_view).is_some_and(|p| !unable(m,&p) && m.distance_sqr(p.position)>=25.0)
    }
    fn can_continue(&mut self,m:&mut Mob,cx:&mut Cx) -> bool {
        !m.ai.nav.is_done() && owner(m,&self.owner_view).is_some_and(|p| !unable(m,&p) && m.distance_sqr(p.position)>1.0)
    }
    fn start(&mut self,m:&mut Mob,_:&mut Cx) { self.recalc=0; m.set_malus(horizon_client::mobs::path::PathType::Water,0.0); }
    fn stop(&mut self,m:&mut Mob,_:&mut Cx) { m.nav_stop(); let water=horizon_client::mobs::path::PathType::Water.malus(); m.set_malus(horizon_client::mobs::path::PathType::Water,water); }
    fn tick(&mut self,m:&mut Mob,cx:&mut Cx) {
        let Some(player)=owner(m,&self.owner_view) else { return }; let at=player.position; let eye=player.eye;
        let far=m.distance_sqr(at)>=144.0;
        if !far { let max=m.max_head_x_rot(); m.look_at_speed([at.x,eye,at.z],10.0,max); }
        self.recalc-=1;
        if self.recalc<=0 { self.recalc=reduced(10); if far { teleport(m,cx.w,at); } else { m.nav_move_to_entity(cx.w,[at.x,at.y,at.z],0,1.5); } }
    }
}
