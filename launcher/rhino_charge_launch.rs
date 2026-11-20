fn rhino_charge_launch(game: &mut Game<'_>, events: &mut Vec<MobEvent>) {
    use horizon_client::mobs::naturalist_rhino;
    use horizon_core::geometry::Vec3;

    for index in (0..events.len()).rev() {
        let MobEvent::Knockback { target, source, apply_blocking: Some(1.0), .. } = events[index] else {
            continue;
        };
        let Some(Who::Mob(attacker)) = source.attacker else { continue };
        let Some(at) = game.mobs.iter().position(|mob| mob.id == attacker && mob.kind.id == naturalist_rhino::KIND) else {
            continue;
        };
        let resistance = match target {
            Who::Player => game.player.as_mut().and_then(|player| player.attribute("minecraft:knockback_resistance").ok()),
            Who::Mob(id) => game.mobs.iter().find(|mob| mob.id == id).map(|mob| mob.knockback_resistance()),
        }.unwrap_or(0.0);
        let blocked = target == Who::Player && game.player_blocking(source.from, "minecraft:mob_attack", false)
            .is_some_and(|shield| shield.blocked(1.0) > 0.0);
        if let MobEvent::Knockback { power, apply_blocking, .. } = &mut events[index] {
            *apply_blocking = None;
            if blocked {
                *power *= 0.5;
            }
        }
        if blocked {
            let mob = &mut game.mobs[at];
            naturalist_rhino::stun(mob);
            if let Some(player) = game.player.as_mut() {
                let (x, z) = (player.position.x - mob.position.x, player.position.z - mob.position.z);
                let distance = x.abs().max(z.abs());
                if distance >= 0.01 {
                    let root = distance.sqrt();
                    let scale = (1.0 / root).min(1.0) * 0.05 / root;
                    let velocity = Vec3::new(x * scale, 0.0, z * scale);
                    player.velocity = player.velocity.add_vector(velocity);
                    mob.velocity = mob.velocity.subtract(velocity);
                }
            }
            let at = mob.position.components();
            events.push(MobEvent::EntityEvent { mob: mob.id, event: 39, at });
            if !mob.silent {
                events.push(MobEvent::Sound("entity.ravager.stunned", at, 1.0, 1.0));
            }
        }
        let up = f64::from(0.4_f32) * (1.0 - resistance).max(0.0);
        events.insert(index + 1, MobEvent::Push { target, velocity: [0.0, up, 0.0] });
    }
}
