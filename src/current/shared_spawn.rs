//! Shared Naturalist natural-spawn initialization.
//!
//! Naturalist's Bird is backed by Horizon's Parrot implementation for flight,
//! but its Java finalizer owns the variant pool and never creates a baby in a
//! natural group.  The generic Parrot finalizer otherwise draws a vanilla
//! variant and a baby roll, shifting every later draw in a fixed seed.

use horizon_client::mobs::finalize::{Ageable, Group};
use horizon_core::random::RandomSource;

const BIRD_POOLS: &[(&str, &[(&str, i32)])] = &[
    ("taiga", &[("naturalist:blue_jay", 180)]),
    ("desert", &[("naturalist:northern_cardinal", 92), ("naturalist:red_winged_blackbird", 88)]),
    ("plains", &[("naturalist:american_robin", 147), ("naturalist:white_throated_sparrow", 33)]),
    ("forest", &[
        ("naturalist:american_robin", 51),
        ("naturalist:carolina_chickadee", 10),
        ("naturalist:northern_cardinal", 52),
        ("naturalist:red_winged_blackbird", 55),
        ("naturalist:tufted_titmouse", 12),
    ]),
    ("mushroom_fields", &[("naturalist:american_robin", 180)]),
];

fn bird_pool(biome: &str) -> &'static [(&'static str, i32)] {
    BIRD_POOLS
        .iter()
        .find(|(tag, _)| biome.ends_with(tag) || biome.contains(tag))
        .map_or(BIRD_POOLS[0].1, |(_, pool)| *pool)
}

fn choose_bird_variant(game: &mut crate::game::Game<'_>, mob: &horizon_client::mobs::Mob) {
    let pool = bird_pool(&game.biome_at(mob.position.components()));
    let total: i32 = pool.iter().map(|(_, weight)| weight).sum();
    let Ok(mut roll) = game.world.random.next_int_bound(total) else { return };
    for &(variant, weight) in pool {
        if roll < weight {
            let raw = game
                .persist
                .raw
                .entry(mob.id)
                .or_insert_with(|| horizon_core::nbt::Tag::Compound(Default::default()));
            if let horizon_core::nbt::Tag::Compound(fields) = raw {
                fields.insert("Variant".into(), horizon_core::nbt::Tag::String(variant.into()));
            }
            return;
        }
        roll -= weight;
    }
}

pub fn before_grouped(
    game: &mut crate::game::Game<'_>,
    mob: &horizon_client::mobs::Mob,
    group: &mut Option<Group>,
) {
    if mob.kind.id != "naturalist:bird" {
        return;
    }
    choose_bird_variant(game, mob);
    // Java BirdEntity is explicitly non-ageable. Supplying an existing group
    // keeps Horizon's generic Parrot finalizer from drawing its 5% baby roll.
    *group = Some(Group::Ageable(Ageable::chance(0.0)));
}
