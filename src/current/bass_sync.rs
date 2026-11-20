fn bass_sync(game: &mut Game<'_>, entity: u32, kind: &str) {
    use mod_api::{Holder, Host};
    use naturalist::bass_appearance::{BASS_APPEARANCE, BassAppearance};
    if kind != "naturalist:bass" {
        return;
    }
    let holder = Holder::Entity(entity);
    let previous = BASS_APPEARANCE
        .get_existing(&mut *game, holder)
        .ok()
        .flatten()
        .unwrap_or_default();
    let variant = game
        .entity(entity)
        .and_then(|e| e.nbt(game))
        .and_then(|data| match data.get(&"Variant".into()) {
            Some(mod_api::Tag::String(s)) => Some(s.to_string_lossy()),
            Some(mod_api::Tag::Int(n)) => Some(
                match n.rem_euclid(3) {
                    1 => "naturalist:bass_medium",
                    2 => "naturalist:bass_large",
                    _ => "naturalist:bass",
                }
                .to_owned(),
            ),
            _ => None,
        })
        .unwrap_or_else(|| "naturalist:bass".to_owned());
    let texture = variant
        .split_once(':')
        .and_then(|(namespace, path)| {
            let id = mod_api::Identifier::parse(&format!(
                "{namespace}:naturalist/bass_variant/{path}.json"
            ))
            .ok()?;
            let bytes = game.resource(mod_api::PackType::ServerData, &id)?;
            let json: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
            Some(json.get("texture")?.as_str()?.to_owned())
        })
        .unwrap_or_else(|| "naturalist:entity/bass".to_owned());
    let Some(mob) = game.mobs.iter().find(|mob| mob.id == entity) else {
        return;
    };
    let in_water = mob.fluids.in_water;
    let mut tilt = 0.0;
    let mut target = 0.0;
    if in_water {
        let turn = (mob.previous_yaw - mob.yaw + 180.0).rem_euclid(360.0) - 180.0;
        tilt = if turn.abs() > 1.0 {
            if previous.tilt.abs() < 25.0 {
                previous.tilt - turn.signum()
            } else {
                previous.tilt
            }
        } else {
            previous.tilt.signum() * (previous.tilt.abs() - 0.85).max(0.0)
        };
        let dx = mob.position.x - mob.previous.x;
        let dy = mob.position.y - mob.previous.y;
        let dz = mob.position.z - mob.previous.z;
        let horizontal = dx.hypot(dz);
        let weight = ((horizontal.hypot(dy) - 0.01) / 0.04).clamp(0.0, 1.0) as f32;
        target = -(dy.atan2(horizontal).to_degrees().clamp(-55.0, 55.0) as f32) * weight;
    }
    let value = BassAppearance {
        variant,
        texture,
        in_water,
        tilt,
        pitch: previous.pitch + (target - previous.pitch) * 0.2,
    };
    if value != previous
        || BASS_APPEARANCE
            .get_existing(&mut *game, holder)
            .ok()
            .flatten()
            .is_none()
    {
        let _ = BASS_APPEARANCE.set(&mut *game, holder, value);
    }
}
