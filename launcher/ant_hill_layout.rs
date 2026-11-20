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
const BIG: [[i32; 3]; 15] = [
    [-1, 0, -1],
    [0, 0, -1],
    [-1, 0, 0],
    [0, 0, 0],
    [1, 0, 0],
    [-1, 0, 1],
    [0, 0, 1],
    [1, 0, 1],
    [-1, 1, 0],
    [0, 1, 0],
    [1, 1, 0],
    [0, 1, 1],
    [0, 2, 0],
    [0, 2, 1],
    [0, 3, 0],
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

pub fn positions(
    big: bool,
    base: [i32; 3],
    quarter_turns: usize,
) -> impl Iterator<Item = [i32; 3]> {
    let offsets = if big { &BIG[..] } else { &SMALL[..] };
    offsets.iter().map(move |&offset| {
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
    for position in (0..4).flat_map(|rotation| positions(false, base, rotation)) {
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
