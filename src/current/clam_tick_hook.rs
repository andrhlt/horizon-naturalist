fn clam_tick(game: &mut Game<'_>, entity: u32, kind: &str) {
    if kind != "naturalist:clam" {
        return;
    }
    use horizon_core::nbt::Tag;
    use mod_api::{Host, prelude::ServicesServerHost};

    let Some(clam_ref) = game.entity(entity) else { return };
    let Some(clam_pos) = clam_ref.position(game) else { return };
    let Some(level) = clam_ref.level(game) else { return };
    let players = game.players().server_players();
    let mut nearest = None;
    for player in players {
        let player_entity = player.entity();
        if player_entity.level(game) != Some(level) || player_entity.is_shift_key_down(game) == Some(true) {
            continue;
        }
        let Some(pos) = player_entity.position(game) else { continue };
        let dx = pos[0] - clam_pos[0];
        let dy = pos[1] - clam_pos[1];
        let dz = pos[2] - clam_pos[2];
        let distance = dx * dx + dy * dy + dz * dz;
        if distance <= 16.0 && nearest.is_none_or(|(_, best): (mod_api::PlayerRef, f64)| distance < best) {
            nearest = Some((player, distance));
        }
    }

    let number = |name: &str| -> i32 {
        game.persist.raw.get(&entity).and_then(|tag| tag.get(name)).and_then(|tag| match tag {
            Tag::Byte(value) => Some(i32::from(*value)),
            Tag::Short(value) => Some(i32::from(*value)),
            Tag::Int(value) => Some(*value),
            _ => None,
        }).unwrap_or(0)
    };
    let mut open = number("NaturalistClamOpen") != 0;
    let mut timer = number("NaturalistClamTimer");
    let mut cooldown = number("NaturalistClamCooldown");
    let mut delay = number("NaturalistClamLaunchDelay");

    let previous_delay = delay;
    if cooldown > 0 { cooldown -= 1; }
    if timer > 0 {
        timer -= 1;
        if timer == 0 { open = false; }
    }
    if delay > 0 {
        delay -= 1;
        if delay == 0 && previous_delay == 1 {
            if let Some((player, _)) = nearest {
                let current = player.entity().velocity(game).unwrap_or([0.0; 3]);
                game.player_set_velocity(player, [current[0], current[1] + 2.2, current[2]]);
            }
        }
    }

    let treasure = game.mobs.iter().find(|mob| mob.id == entity)
        .and_then(|mob| mob.extra.main_hand_stack()).is_some();
    if let Some((_, distance)) = nearest {
        if treasure && distance <= 9.0 {
            open = false;
            delay = 0;
            timer = 300;
        } else if !open && cooldown <= 0 && distance <= 4.0 {
            open = true;
            delay = 4;
            cooldown = 60;
            timer = 150;
        }
    }

    let raw = game.persist.raw.entry(entity).or_insert_with(|| Tag::Compound(Default::default()));
    if let Tag::Compound(fields) = raw {
        fields.insert("NaturalistClamOpen".into(), Tag::Byte(i8::from(open)));
        fields.insert("NaturalistClamTimer".into(), Tag::Int(timer));
        fields.insert("NaturalistClamCooldown".into(), Tag::Int(cooldown));
        fields.insert("NaturalistClamLaunchDelay".into(), Tag::Int(delay));
    }
}
