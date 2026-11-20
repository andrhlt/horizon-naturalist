fn deer_sync(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    use crate::game::naturalist_appearance::{DeerAppearance, DEER_APPEARANCE};
    use mod_api::Holder;
    if kind != "naturalist:deer" { return; }
    let Some(mob) = game.mobs.iter_mut().find(|m| m.id == entity) else { return; };
    if mob.extra.anim_ticks > 0 { mob.extra.anim_ticks -= 1; }
    let eating = mob.extra.anim_ticks > 0;
    let sprinting = mob.extra.sprinting;
    let movement_speed = mob.movement_speed() as f32;
    let variant = game.persist.raw.get(&entity).and_then(|tag| tag.get("Variant"));
    let variant = match variant {
        Some(horizon_core::nbt::Tag::String(value)) => value.to_string_lossy(),
        None => "naturalist:deer".to_owned(),
        _ => return,
    };
    let Some((adult_texture, baby_texture)) = crate::game::naturalist_deer_goals::variant_textures(game, &variant) else { return; };
    let value = DeerAppearance { variant, adult_texture, baby_texture, eating, sprinting, movement_speed };
    let holder = Holder::Entity(entity);
    if DEER_APPEARANCE.get_existing(&mut *game, holder).ok().flatten().as_ref() != Some(&value) {
        let _ = DEER_APPEARANCE.set(&mut *game, holder, value);
    }
}
