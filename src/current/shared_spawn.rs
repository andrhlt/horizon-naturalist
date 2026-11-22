//! Shared Naturalist natural-spawn initialization.
//!
//! Naturalist's Bird is backed by Horizon's Parrot implementation for flight,
//! but its Java finalizer owns the variant pool and never creates a baby in a
//! natural group.  The generic Parrot finalizer otherwise draws a vanilla
//! variant and a baby roll, shifting every later draw in a fixed seed.

use horizon_client::mobs::finalize::{Ageable, Group};
use horizon_core::random::RandomSource;

const BIRD_POOLS: &[(&str, &[(&str, i32)])] = &[
    ("minecraft:badlands", &[("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3)]),
    ("minecraft:beach", &[("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3)]),
    ("minecraft:birch_forest", &[("naturalist:american_robin", 3), ("naturalist:carolina_chickadee", 1), ("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3), ("naturalist:tufted_titmouse", 1)]),
    ("minecraft:cherry_grove", &[("naturalist:american_robin", 3), ("naturalist:blue_jay", 3), ("naturalist:stellers_jay", 1), ("naturalist:white_throated_sparrow", 1)]),
    ("minecraft:dappled_forest", &[("naturalist:american_robin", 3), ("naturalist:carolina_chickadee", 1), ("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3), ("naturalist:tufted_titmouse", 1)]),
    ("minecraft:dark_forest", &[("naturalist:american_robin", 3), ("naturalist:carolina_chickadee", 1), ("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3), ("naturalist:tufted_titmouse", 1)]),
    ("minecraft:desert", &[("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3)]),
    ("minecraft:eroded_badlands", &[("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3)]),
    ("minecraft:flower_forest", &[("naturalist:american_robin", 3), ("naturalist:carolina_chickadee", 1), ("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3), ("naturalist:tufted_titmouse", 1)]),
    ("minecraft:forest", &[("naturalist:american_robin", 3), ("naturalist:carolina_chickadee", 1), ("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3), ("naturalist:tufted_titmouse", 1)]),
    ("minecraft:frozen_peaks", &[("naturalist:american_robin", 3), ("naturalist:blue_jay", 3), ("naturalist:stellers_jay", 1)]),
    ("minecraft:grove", &[("naturalist:american_robin", 3), ("naturalist:blue_jay", 3), ("naturalist:carolina_chickadee", 1), ("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3), ("naturalist:stellers_jay", 1), ("naturalist:tufted_titmouse", 1)]),
    ("minecraft:ice_spikes", &[("naturalist:blue_jay", 3)]),
    ("minecraft:jagged_peaks", &[("naturalist:american_robin", 3), ("naturalist:blue_jay", 3), ("naturalist:stellers_jay", 1)]),
    ("minecraft:mangrove_swamp", &[("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3)]),
    ("minecraft:meadow", &[("naturalist:american_robin", 3), ("naturalist:blue_jay", 3), ("naturalist:stellers_jay", 1)]),
    ("minecraft:old_growth_birch_forest", &[("naturalist:american_robin", 3), ("naturalist:carolina_chickadee", 1), ("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3), ("naturalist:tufted_titmouse", 1)]),
    ("minecraft:old_growth_pine_taiga", &[("naturalist:blue_jay", 3)]),
    ("minecraft:old_growth_spruce_taiga", &[("naturalist:blue_jay", 3)]),
    ("minecraft:pale_garden", &[("naturalist:american_robin", 3), ("naturalist:carolina_chickadee", 1), ("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3), ("naturalist:tufted_titmouse", 1)]),
    ("minecraft:plains", &[("naturalist:american_robin", 3), ("naturalist:white_throated_sparrow", 1)]),
    ("minecraft:savanna", &[("naturalist:carolina_chickadee", 1), ("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3), ("naturalist:tufted_titmouse", 1)]),
    ("minecraft:savanna_plateau", &[("naturalist:carolina_chickadee", 1), ("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3), ("naturalist:tufted_titmouse", 1)]),
    ("minecraft:snowy_beach", &[("naturalist:blue_jay", 3)]),
    ("minecraft:snowy_plains", &[("naturalist:blue_jay", 3)]),
    ("minecraft:snowy_slopes", &[("naturalist:american_robin", 3), ("naturalist:blue_jay", 3), ("naturalist:stellers_jay", 1)]),
    ("minecraft:snowy_taiga", &[("naturalist:blue_jay", 3)]),
    ("minecraft:stony_peaks", &[("naturalist:american_robin", 3), ("naturalist:blue_jay", 3), ("naturalist:stellers_jay", 1)]),
    ("minecraft:sunflower_plains", &[("naturalist:american_robin", 3), ("naturalist:white_throated_sparrow", 1)]),
    ("minecraft:swamp", &[("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3)]),
    ("minecraft:taiga", &[("naturalist:blue_jay", 3)]),
    ("minecraft:windswept_forest", &[("naturalist:blue_jay", 3), ("naturalist:stellers_jay", 1)]),
    ("minecraft:windswept_gravelly_hills", &[("naturalist:blue_jay", 3), ("naturalist:stellers_jay", 1)]),
    ("minecraft:windswept_hills", &[("naturalist:blue_jay", 3), ("naturalist:stellers_jay", 1)]),
    ("minecraft:windswept_savanna", &[("naturalist:carolina_chickadee", 1), ("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3), ("naturalist:tufted_titmouse", 1)]),
    ("minecraft:wooded_badlands", &[("naturalist:northern_cardinal", 3), ("naturalist:red_winged_blackbird", 3)]),
];

fn bird_pool(biome: &str) -> Option<&'static [(&'static str, i32)]> {
    BIRD_POOLS
        .iter()
        .find(|(name, _)| *name == biome)
        .map(|(_, pool)| *pool)
}

fn choose_bird_variant(game: &mut crate::game::Game<'_>, mob: &horizon_client::mobs::Mob) {
    let Some(pool) = bird_pool(&game.biome_at(mob.position.components())) else { return };
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
