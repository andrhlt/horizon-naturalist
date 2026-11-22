fn duck_appearance_tick(game:&mut Game<'_>, entity:u32, kind:&str) {
 use crate::game::naturalist_duck_appearance::{DuckAppearance, DUCK_APPEARANCE};
 use mod_api::Holder;
 if kind!="naturalist:duck" {return;}
 let Some(m)=game.mobs.iter().find(|m|m.id==entity) else{return;};
 let variant=game.persist.raw.get(&entity).and_then(|r|r.get("Variant")).and_then(|v|match v {horizon_core::nbt::Tag::String(s)=>Some(s.to_string_lossy()),_=>None}).and_then(|s|mod_api::Identifier::parse(&s).ok().map(|id|id.to_string())).unwrap_or_else(||"naturalist:duck".into());
 let queso=m.custom_name.as_deref().is_some_and(|n|n.eq_ignore_ascii_case("queso"));
 let (adult_texture,baby_texture)=crate::game::naturalist_duck_variants::textures(game,&variant,queso);
 let a=DuckAppearance{adult_texture,baby_texture,color:if m.extra.color<16{i32::from(m.extra.color)}else{-1},sitting:m.extra.sitting_pose,in_water:m.fluids.in_water,sprinting:m.extra.sprinting,on_ground:m.on_ground,speed:m.movement_speed() as f32};
 let holder=Holder::Entity(entity);
 if DUCK_APPEARANCE.get_existing(&mut *game,holder).ok().flatten().as_ref()!=Some(&a) {let _=DUCK_APPEARANCE.set(&mut *game,holder,a);}
}
