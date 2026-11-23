fn caterpillar_cocoon_tick(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    if kind == "naturalist:caterpillar" {
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
            crate::game::naturalist_caterpillar_cocoon::install(mob);
        }
    } else if kind == "naturalist:butterfly" {
        if let Some(mob) = game.mobs.iter_mut().find(|mob| mob.id == entity) {
            crate::game::naturalist_butterfly_pollinate::install(mob);
        }
    } else if kind == "minecraft:player" {
        use mod_api::{BlockUpdateFlags, SpawnReason};
        let Some(player) = game.entity(entity).and_then(|e| e.position(game)) else { return; };
        let center = player.map(|v| v.floor() as i32);
        let Some(level) = game.level(game.dim.id()) else { return; };
        let Some(air) = game.pack.assets.registry.default_state("minecraft:air") else { return; };
        let mut hatch = None;
        'scan: for x in center[0] - 32..=center[0] + 32 {
            for y in center[1] - 32..=center[1] + 32 {
                for z in center[2] - 32..=center[2] + 32 {
                    let state = game.world.block(x, y, z);
                    let Some(row) = game.pack.assets.registry.state(state) else { continue; };
                    if row.name == "naturalist:chrysalis"
                        && row.properties.get("age").map(String::as_str) == Some("3")
                    {
                        hatch = Some([x, y, z]);
                        break 'scan;
                    }
                }
            }
        }
        let Some(pos) = hatch else { return; };
        if !game.level_set_block(level, pos, air, BlockUpdateFlags::NEIGHBORS | BlockUpdateFlags::CLIENTS).unwrap_or(false) {
            return;
        }
        let at = [f64::from(pos[0]) + 0.5, f64::from(pos[1]) + 0.5, f64::from(pos[2]) + 0.5];
        let _ = level.spawn_mob(game, "naturalist:butterfly", at, SpawnReason::Command);
    }
}
