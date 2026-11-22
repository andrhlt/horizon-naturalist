fn crab_follow_tick(game: &mut Game<'_>, entity: u32, kind: &str) {
    use mod_api::{Host, Holder};
    use mod_api::prelude::ServicesServerHost;
    use crate::game::naturalist_crab_follow::{CrabFollow, OwnerView, OWNER_VIEW};
    if kind != "naturalist:crab" { return; }
    let Some(crab_entity)=game.entity(entity) else { return; };
    let Some(handle)=game.mob(crab_entity) else { return; };
    if game.mobs.iter().find(|m|m.id==entity).is_some_and(|m|!m.ai.targets.has("NaturalistCrabRetaliate")) {
        handle.remove_goal(game, "TamePanic");
        handle.remove_goal(game, "melee_attack");
        if let Some(m)=game.mobs.iter_mut().find(|m|m.id==entity) { crate::game::naturalist_crab_targets::install(m); }
    }
    let following=game.persist.raw.get(&entity).and_then(|raw|raw.get("FollowingOwner"))
        .and_then(horizon_world::datafix::dynamic::as_number).map_or(true,|n|n.int_value()!=0);
    let owner_uuid=game.mobs.iter().find(|m|m.id==entity && m.extra.tame).and_then(|m|m.extra.owner);
    let has=|name|game.mobs.iter().find(|m|m.id==entity).is_some_and(|m|m.ai.goals.has(name));
    let (has_wolf,has_crab)=(has("FollowOwner"),has("NaturalistCrabFollowOwner"));
    if has_wolf {handle.remove_goal(game,"FollowOwner");}
    let holder=Holder::Entity(entity);
    if !following || owner_uuid.is_none() {
        if let Some(cell)=OWNER_VIEW.get_existing(&mut *game,holder).ok().flatten()
            && let Ok(mut state)=cell.write() { *state=None; }
        if has_crab {handle.remove_goal(game,"NaturalistCrabFollowOwner");}
        return;
    }
    let level=crab_entity.level(game);
    let view=owner_uuid.and_then(|uuid|game.players().server_players().into_iter()
        .find(|p|horizon_client::lan::playerdata::uuid_ints(p.uuid())==uuid).and_then(|p|{
            let other=p.entity();
            if other.level(game)!=level {return None;}
            let [x,y,z]=other.position(game)?;let eye=other.eye_position(game)?[1];
            let spectator=p.is_spectator(game)?;
            Some(OwnerView{position:horizon_core::geometry::Vec3::new(x,y,z),eye,spectator,uuid})
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
    if let Some(m)=game.mobs.iter_mut().find(|m|m.id==entity) && !m.ai.goals.has("NaturalistCrabFollowOwner") {
        m.ai.goals.add(7,CrabFollow::new(cell));
    }
}
