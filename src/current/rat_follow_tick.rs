fn rat_follow_tick(game: &mut Game<'_>, entity: u32, kind: &str) {
    use mod_api::{Host, Holder};
    use mod_api::prelude::ServicesServerHost;
    use crate::game::naturalist_rat_follow::{RatFollow, RatSit, OwnerView, OWNER_VIEW};
    if kind != "naturalist:rat" { return; }
    let Some(rat_entity)=game.entity(entity) else { return; };
    let Some(handle)=game.mob(rat_entity) else { return; };
    let following=game.persist.raw.get(&entity).and_then(|raw|raw.get("FollowingOwner"))
        .and_then(horizon_world::datafix::dynamic::as_number).map_or(true,|n|n.int_value()!=0);
    let owner_uuid=game.mobs.iter().find(|m|m.id==entity && m.extra.tame).and_then(|m|m.extra.owner);
    let (has_wolf,has_rat)=game.mobs.iter().find(|m|m.id==entity)
        .map(|m|(m.ai.goals.has("FollowOwner"),m.ai.goals.has("NaturalistRatFollowOwner")))
        .unwrap_or((false,false));
    if has_wolf {handle.remove_goal(game,"FollowOwner");}
    let holder=Holder::Entity(entity);
    let level=rat_entity.level(game);
    let view=owner_uuid.and_then(|uuid|game.players().server_players().into_iter()
        .find(|p|horizon_client::lan::playerdata::uuid_ints(p.uuid())==uuid).and_then(|p|{
            let other=p.entity();
            if other.level(game)!=level || game.living(other).and_then(|living| living.health(game)).is_none_or(|health| health <= 0.0) {return None;}
            let [x,y,z]=other.position(game)?;let eye=other.eye_position(game)?[1];
            let spectator=p.is_spectator(game)?;
            let attacked=game.living(other).and_then(|living| living.last_attacker(game)).is_some();
            Some(OwnerView{position:horizon_core::geometry::Vec3::new(x,y,z),eye,spectator,attacked,uuid})
        }));
    let cell=match OWNER_VIEW.get_existing(&mut *game,holder).ok().flatten() {
        Some(cell)=>cell,
        None=>{
            let Ok(cell)=OWNER_VIEW.get(&mut *game,holder) else { return; };
            if OWNER_VIEW.set(&mut *game,holder,cell.clone()).is_err() { return; }
            cell
        }
    };
    if let Ok(mut state)=cell.write() { *state=view; }
    if let Some(m)=game.mobs.iter_mut().find(|m|m.id==entity) && !m.ai.goals.has("NaturalistRatSit") {
        m.ai.goals.add(1,RatSit{owner_view:cell.clone()});
    }
    if !following || owner_uuid.is_none() {
        if has_rat { handle.remove_goal(game,"NaturalistRatFollowOwner"); }
        return;
    }
    if let Some(m)=game.mobs.iter_mut().find(|m|m.id==entity) && !m.ai.goals.has("NaturalistRatFollowOwner") {
        m.ai.goals.add(7,RatFollow::new(cell));
    }
}
