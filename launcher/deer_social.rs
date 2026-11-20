use super::{Mob, Seen, Species};

// Several Naturalist species share a stock template. Mark only snapshots
// so the variant used for storage and rendering is left alone.
pub(super) const ZEBRA_MARKER: u8 = u8::MAX - 7;

pub(super) fn animal_marker(kind: &str) -> Option<u8> {
    match kind {
        "naturalist:deer" | "naturalist:duck" | "naturalist:boar" => Some(u8::MAX),
        "naturalist:turkey" | "naturalist:capybara" => Some(u8::MAX - 1),
        "naturalist:desert_scorpion" | "naturalist:jungle_scorpion" | "naturalist:lizard" | "naturalist:lizard_tail" => Some(u8::MAX - 2),
        "naturalist:komodo_dragon" => Some(u8::MAX - 3),
        "naturalist:rhino" => Some(u8::MAX - 4),
        "naturalist:bear" => Some(u8::MAX - 5),
        "naturalist:black_bear" => Some(u8::MAX - 6),
        "naturalist:zebra" => Some(ZEBRA_MARKER),
        "naturalist:lion" => Some(u8::MAX - 12),
        "naturalist:snake" => Some(u8::MAX - 13),
        "naturalist:tiger" => Some(u8::MAX - 11),
        "naturalist:giraffe" => Some(u8::MAX - 8),
        "naturalist:snail" => Some(super::nav::naturalist_blobfish_state::SNAIL_PREY),
        _ => None,
    }
}

pub(super) fn same_farm_template_animal(mob: &Mob, other: &Seen) -> bool {
    use crate::mobs::{
        goals::{
            naturalist_alligator_goals::KIND as ALLIGATOR,
            naturalist_cat_goals::KIND as TIGER,
            naturalist_lion_goals::KIND as LION,
            naturalist_mole_goals::{MOLE, TRAIL},
            naturalist_rat_goals::KIND as RAT,
            naturalist_snake_goals::KIND as SNAKE,
            naturalist_tortoise_goals::KIND as TORTOISE,
        },
        nav::{
            naturalist_elephants::{ELEPHANT, MAMMOTH},
            naturalist_giant_isopod::KIND as ISOPOD,
            naturalist_hippo_state::KIND as HIPPO,
            naturalist_whale_state::KIND as WHALE,
        },
    };

    for kind in [
        WHALE, RAT, LION, TIGER, TORTOISE, ELEPHANT, MAMMOTH, ISOPOD, HIPPO, ALLIGATOR, MOLE, SNAKE, TRAIL,
    ] {
        let local = mob.kind.id == kind;
        let other_has_kind = other.effects.contains(&kind);
        if local || other_has_kind {
            return local && other_has_kind;
        }
    }
    matches!(mob.species(), Species::Cow | Species::Chicken | Species::Pig)
        && mob.species() == other.species
        && animal_marker(&mob.kind.id) == Some(other.variant).filter(|&variant| variant >= u8::MAX - 9)
}
