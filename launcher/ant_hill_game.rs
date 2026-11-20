use super::Game;
use horizon_client::particles::{ParticleOptions, ParticleType};
use horizon_world::{
    chunk::BlockState,
    level::flags::UPDATE_ALL,
    nbt::Tag,
    registry::BlockStateId,
    worldgen::ant_hill_layout::{self, HillCell},
};

pub(super) const HILL: &str = "naturalist:ant_hill";
pub(super) const QUEEN: &str = "naturalist:queen_ant";

fn replaceable(game: &Game<'_>, position: [i32; 3]) -> bool {
    let [x, y, z] = position;
    let state = game.world.block(x, y, z);
    let block = game.res.facts().get(state);
    block.replaceable
        || block.name.as_ref() == HILL
        || game
            .items()
            .block_tags
            .get("minecraft:leaves")
            .is_some_and(|tag| tag.contains(block.name.as_ref()))
}

fn state(game: &Game<'_>, cell: HillCell) -> Option<BlockStateId> {
    game.pack
        .assets
        .registry
        .resolve(&BlockState {
            name: HILL.to_owned(),
            properties: [
                ("open".to_owned(), cell.open.to_string()),
                ("workers".to_owned(), cell.workers.to_string()),
                ("has_queen".to_owned(), cell.queen.to_string()),
            ]
            .into(),
        })
        .ok()
}

pub(super) fn place_queen(game: &mut Game<'_>, clicked: [i32; 3]) {
    let mut origin = clicked;
    if !replaceable(game, clicked) {
        origin[1] = origin[1].wrapping_add(1);
    }

    let blockers = ant_hill_layout::small_blockers(origin, |position| replaceable(game, position));
    if !blockers.is_empty() {
        let mut options = ParticleOptions::simple(ParticleType::Dust);
        options.color = [1.0, 0.2, 0.2, 1.0];
        for block in blockers {
            let at = block.map(|axis| f64::from(axis) + 0.5);
            game.with_level(|_, level, _| {
                level.native_send_particles(&options, None, at, [0.25; 3], 0.0, 4)
            });
        }
        return;
    }

    let rotation = ant_hill_layout::choose_rotation(&mut *game.world.random);
    let layout = ant_hill_layout::cells(
        ant_hill_layout::positions(false, origin, rotation),
        &mut *game.world.random,
    );
    let dimension = game.dim.id();
    let owner = game.acting_player_uuid();
    for cell in layout {
        if let Some(state) = state(game, cell) {
            game.native_set_block(dimension, cell.position, state, UPDATE_ALL);
            game.native_with_mod_block_entity(cell.position, |entity| {
                entity
                    .nbt
                    .insert("Owner".into(), Tag::IntArray(owner.to_vec()));
            });
        }
    }
    let at = origin.map(|axis| f64::from(axis) + 0.5);
    game.native_play_sound_in_place("minecraft:block.rooted_dirt.place", at, 4, 1.0, 1.0);
    game.consume_held();
}

pub(super) const RELEASE_EVENTS: [&str; 6] = [
    "naturalist:ant_hill_release_down",
    "naturalist:ant_hill_release_up",
    "naturalist:ant_hill_release_north",
    "naturalist:ant_hill_release_south",
    "naturalist:ant_hill_release_west",
    "naturalist:ant_hill_release_east",
];

type Owner = Option<[i32; 4]>;

fn owner_at(game: &Game<'_>, [x, y, z]: [i32; 3]) -> Owner {
    use horizon_client::containers::BlockEntity;
    match game.world.block_entities.get(&(x, y, z)) {
        Some(BlockEntity::Mod(hill)) => owner_from_tag(&hill.nbt),
        _ => None,
    }
}

fn owner_from_tag(nbt: &horizon_world::nbt::Compound) -> Owner {
    match nbt.get(&"Owner".into()) {
        Some(Tag::IntArray(owner)) => owner.as_slice().try_into().ok(),
        _ => None,
    }
}

fn release_worker(game: &mut Game<'_>, [x, y, z]: [i32; 3], owner: Owner) -> Option<u32> {
    use horizon_client::mobs::naturalist_ant::{ANT, start_cooldown};
    use horizon_core::random::RandomSource;
    let at = [
        f64::from(x) + 0.25 + game.world.random.next_double() * 0.5,
        f64::from(y),
        f64::from(z) + 0.25 + game.world.random.next_double() * 0.5,
    ];
    let yaw = game.world.random.next_float() * 360.0;
    let id = game.spawn_mob_raw(ANT, at, false)?;
    let ant = game.mobs.iter_mut().find(|ant| ant.id == id)?;
    start_cooldown(ant);
    ant.extra.tame = owner.is_some();
    ant.extra.owner = owner;
    ant.yaw = yaw;
    ant.previous_yaw = yaw;
    ant.body_yaw = yaw;
    ant.head_yaw = yaw;
    Some(id)
}

pub(super) fn release_one(game: &mut Game<'_>, hill: [i32; 3], direction: usize) {
    use horizon_core::geometry::Direction;
    let owner = owner_at(game, hill);
    super::naturalist_ant_workers::hill_effects(game, hill);
    let offset = Direction::ALL[direction].normal();
    let exit = std::array::from_fn(|axis| hill[axis] + offset[axis]);
    release_worker(game, exit, owner);
}

fn release_exit(game: &mut Game<'_>, hill: [i32; 3]) -> Option<[i32; 3]> {
    use horizon_core::{geometry::Direction, random::RandomSource};
    let exits: Vec<_> = Direction::ALL
        .into_iter()
        .map(|direction| {
            let [x, y, z] = direction.normal();
            [hill[0] + x, hill[1] + y, hill[2] + z]
        })
        .filter(|&[x, y, z]| {
            let air = |y| game.res.facts().is_air(game.world.block(x, y, z));
            air(y) && !air(y - 1)
        })
        .collect();
    let roll = game.world.random.next_int_bound(exits.len() as i32).ok()?;
    Some(exits[roll as usize])
}

pub(super) fn check_defense(game: &mut Game<'_>, pos: [i32; 3], entity: &horizon_client::containers::ModBlockEntity) {
    use horizon_client::mobs::{Species, Who};
    use horizon_core::{geometry::Vec3, position::BlockPos};
    if entity.id != HILL
        || game
            .world
            .game_time
            .wrapping_add(BlockPos::new(pos[0], pos[1], pos[2]).pack())
            % 10
            != 0
    {
        return;
    }
    let Some(owner) = owner_from_tag(&entity.nbt) else {
        return;
    };
    let state = game.world.block(pos[0], pos[1], pos[2]);
    let Some(row) = game.pack.assets.registry.state(state) else {
        return;
    };
    if row.properties.get("open").map(String::as_str) != Some("true") {
        return;
    }
    let workers = row
        .properties
        .get("workers")
        .and_then(|count| count.parse::<u8>().ok())
        .unwrap_or(0);
    if workers == 0 {
        return;
    }
    let Some(player) = game
        .player_view()
        .filter(|player| player.uuid == owner)
        .or_else(|| {
            game.lan_guest_views()
                .into_iter()
                .find_map(|(_, entry)| (entry.uuid == owner).then_some(entry))
        })
    else {
        return;
    };
    let center = Vec3::new(
        pos[0] as f64 + 0.5,
        pos[1] as f64 + 0.5,
        pos[2] as f64 + 0.5,
    );
    if player.position.distance_squared(center) > 576.0 {
        return;
    }
    let now = game.world.game_time;
    let recent = |entry: &(Who, i64)| now - entry.1 < 10;
    let Some((Who::Mob(threat), _)) = player
        .hurt_by
        .filter(recent)
        .or_else(|| player.last_hurt.filter(recent))
    else {
        return;
    };
    if !game.mobs.iter().any(|mob| {
        mob.id == threat
            && mob.alive()
            && !mob.extra.tame
            && !matches!(
                mob.species(),
                Species::Creeper | Species::Ghast
            )
            && mob.position.distance_squared(center) <= 576.0
    }) {
        return;
    }
    let Some(exit) = release_exit(game, pos) else {
        return;
    };
    if !game.native_set_block_property(pos, "workers", "0") {
        return;
    }
    super::naturalist_ant_workers::hill_effects(game, pos);
    for _ in 0..workers {
        if let Some(id) = release_worker(game, exit, Some(owner))
            && let Some(ant) = game.mobs.iter_mut().find(|ant| ant.id == id)
        {
            ant.ai.target = Some(Who::Mob(threat));
        }
    }
}

pub(super) fn before_player_break(game: &mut Game<'_>, pos: [i32; 3], state: BlockStateId) {
    use horizon_client::mobs::{Who, naturalist_ant::{ANGER_TICKS, state_mut}};
    let Some(row) = game.pack.assets.registry.state(state) else {
        return;
    };
    if row.name != HILL {
        return;
    }
    let workers = row
        .properties
        .get("workers")
        .and_then(|count| count.parse::<u8>().ok())
        .unwrap_or(0);
    let queen = row.properties.get("has_queen").map(String::as_str) == Some("true");
    let owner = owner_at(game, pos);
    let player = game.acting_player_uuid();
    let anger_end = game.world.game_time + i64::from(ANGER_TICKS);
    let guest = game.lan_acting;
    let at = pos.map(|axis| f64::from(axis) + 0.5);
    game.native_play_sound_in_place("minecraft:block.azalea.break", at, 4, 0.9, 1.0);
    if workers > 0 || queen {
        let options = ParticleOptions::simple(ParticleType::Poof);
        game.with_level(|_, level, _| {
            level.native_send_particles(&options, None, at, [0.25; 3], 0.0, 8)
        });
    }
    for _ in 0..workers {
        if let Some(id) = release_worker(game, pos, owner)
            && owner != Some(player)
            && let Some(ant) = game.mobs.iter_mut().find(|ant| ant.id == id)
        {
            ant.ai.target = Some(Who::Player);
            ant.extra.angry_at = Some(Who::Player);
            ant.extra.anger = ANGER_TICKS;
            ant.extra.anger_end = anger_end;
            state_mut(ant).anger_actor = Some(player);
            if let Some(conn) = guest
                && let Some(host) = &mut game.lan
            {
                host.mob_players.insert(id, conn);
            }
        }
    }
}
