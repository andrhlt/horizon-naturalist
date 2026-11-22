use super::Game;
mod ant_hill_layout {
    use horizon_core::random::RandomSource;

    const SMALL: [[i32; 3]; 10] = [
        [0, 0, -1],
        [-1, 0, 0],
        [0, 0, 0],
        [1, 0, 0],
        [-1, 0, 1],
        [0, 0, 1],
        [-1, 1, 0],
        [0, 1, 0],
        [0, 1, 1],
        [0, 2, 0],
    ];
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct HillCell {
        pub position: [i32; 3],
        pub open: bool,
        pub workers: u8,
        pub queen: bool,
    }

    fn rotate([x, y, z]: [i32; 3], quarter_turns: usize) -> [i32; 3] {
        match quarter_turns {
            0 => [x, y, z],
            1 => [-z, y, x],
            2 => [-x, y, -z],
            _ => [z, y, -x],
        }
    }

    pub fn positions(base: [i32; 3], quarter_turns: usize) -> impl Iterator<Item = [i32; 3]> {
        SMALL.iter().map(move |&offset| {
            let relative = rotate(offset, quarter_turns);
            std::array::from_fn(|axis| base[axis].wrapping_add(relative[axis]))
        })
    }

    pub fn choose_rotation(random: &mut impl RandomSource) -> usize {
        random.next_int_bound(4).expect("nonzero rotation count") as usize
    }

    pub fn small_blockers(
        base: [i32; 3],
        mut replaceable: impl FnMut([i32; 3]) -> bool,
    ) -> Vec<[i32; 3]> {
        let mut blockers = Vec::new();
        for position in (0..4).flat_map(|rotation| positions(base, rotation)) {
            if !replaceable(position) && !blockers.contains(&position) {
                blockers.push(position);
            }
        }
        blockers
    }

    pub fn cells(
        positions: impl Iterator<Item = [i32; 3]>,
        random: &mut impl RandomSource,
    ) -> Vec<HillCell> {
        let mut closed: Vec<_> = positions.collect();
        let count = 4 + random.next_int_bound(3).expect("nonzero hole count") as usize;
        let mut holes = Vec::with_capacity(count);
        for _ in 0..count {
            let index = random
                .next_int_bound(closed.len() as i32)
                .expect("more positions than holes");
            holes.push(closed.remove(index as usize));
        }
        let queen = random
            .next_int_bound(count as i32)
            .expect("holes are nonempty") as usize;
        let mut result: Vec<_> = closed
            .into_iter()
            .map(|position| HillCell {
                position,
                open: false,
                workers: 0,
                queen: false,
            })
            .collect();
        result.extend(holes.into_iter().enumerate().map(|(i, position)| {
            let workers = if i == queen {
                0
            } else {
                match random.next_int_bound(100).expect("nonzero roll range") {
                    0..25 => 0,
                    25..80 => 1,
                    _ => 2,
                }
            };
            HillCell {
                position,
                open: true,
                workers,
                queen: i == queen,
            }
        }));
        result
    }
}
use ant_hill_layout::HillCell;
use horizon_client::particles::{ParticleOptions, ParticleType};
use horizon_core::nbt::Tag;
use horizon_world::{chunk::BlockState, level::flags::UPDATE_ALL, registry::BlockStateId};
use mod_api::{Host, SoundSource};

pub(super) const HILL: &str = "naturalist:ant_hill";

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

pub(super) fn place_queen(game: &mut Game<'_>, clicked: [i32; 3]) -> bool {
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
        return false;
    }

    let rotation = ant_hill_layout::choose_rotation(&mut *game.world.random);
    let layout = ant_hill_layout::cells(
        ant_hill_layout::positions(origin, rotation),
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
    if let Some(level) = game.level(game.dim.id()) {
        level.play_sound(
            game,
            "minecraft:block.rooted_dirt.place",
            at,
            SoundSource::Blocks,
            1.0,
            1.0,
        );
    }
    game.consume_held();
    true
}
