fn deer_social_snapshot(mut seen: Seen, mob: &Mob) -> Seen {
    use super::naturalist_deer_social::animal_marker;

    if let Some(marker) = animal_marker(&mob.kind.id) {
        seen.variant = marker;
    }
    use super::goals::naturalist_tortoise_goals::KIND as TORTOISE;
    if mob.kind.id == TORTOISE {
        seen.effects.push(TORTOISE);
    }
    use super::nav::naturalist_giant_isopod::KIND as ISOPOD;
    if mob.kind.id == ISOPOD {
        seen.effects.push(ISOPOD);
    }
    use super::nav::naturalist_shark_state as shark;
    if shark::is_prey(&mob.ai.nav) {
        seen.effects.push(shark::PREY);
    }
    use super::nav::naturalist_blobfish_state as blobfish;
    match mob.kind.id.as_str() {
        "naturalist:blobfish" => seen.variant = blobfish::SNAPSHOT,
        "naturalist:jellyfish" => seen.variant = super::nav::naturalist_jellyfish_state::SNAPSHOT,
        "naturalist:starfish" => seen.variant = super::nav::naturalist_catfish_state::STARFISH_SNAPSHOT,
        "naturalist:clam" => seen.variant = super::nav::naturalist_catfish_state::CLAM_SNAPSHOT,
        "naturalist:anglerfish" => seen.variant = super::nav::naturalist_catfish_state::ANGLERFISH_SNAPSHOT,
        "naturalist:ray" => seen.variant = super::nav::naturalist_piranha_state::RAY_SNAPSHOT,
        "naturalist:crab" => seen.variant = blobfish::CRAB_PREY,
        _ => {}
    }
    if mob.kind.id == super::naturalist_hedgehog::KIND {
        seen.effects.push(super::naturalist_hedgehog::KIND);
    }
    if mob.kind.id == super::naturalist_ostrich::KIND {
        seen.effects.push(super::naturalist_ostrich::KIND);
    }
    if mob.kind.id == super::naturalist_ant::ANT {
        seen.effects.push(super::naturalist_ant::ANT);
    }
    if mob.kind.id == super::naturalist_giraffe::KIND {
        seen.effects.push(super::naturalist_giraffe::KIND);
    }
    if mob.kind.id == "naturalist:piranha" {
        seen.variant = super::nav::naturalist_piranha_state::SNAPSHOT;
    }
    if mob.kind.id == "naturalist:catfish" {
        seen.variant = super::nav::naturalist_catfish_state::SNAPSHOT;
    }
    if mob.kind.id == "naturalist:bass" {
        seen.variant = super::nav::naturalist_bass_state::SMALLEST_SNAPSHOT + mob.extra.variant.min(2);
    }
    if super::nav::naturalist_deer_predator::is_predator(&mob.ai.nav) {
        // The stock snapshot has no mod fields. The marker is local to the
        // goal snapshot and never adds a real effect to the entity.
        seen.effects.push(super::nav::naturalist_deer_predator::TAG);
    }
    if super::nav::naturalist_deer_predator::is_turkey_hostile(&mob.ai.nav) {
        seen.effects.push(super::nav::naturalist_deer_predator::TURKEY_HOSTILES);
    }
    use super::naturalist_scorpion::{self, DESERT, JUNGLE, PREY};
    match mob.kind.id.as_str() {
        DESERT => seen.effects.push(DESERT),
        JUNGLE => seen.effects.push(JUNGLE),
        _ => {}
    }
    if naturalist_scorpion::is_prey(&mob.ai.nav) {
        seen.effects.push(PREY);
    }
    use super::naturalist_komodo;
    if mob.kind.id == naturalist_komodo::KIND {
        seen.effects.push(naturalist_komodo::KIND);
    }
    if naturalist_komodo::is_prey(&mob.ai.nav) {
        seen.effects.push(naturalist_komodo::PREY);
    }
    use super::naturalist_rhino;
    if mob.kind.id == naturalist_rhino::KIND {
        seen.effects.push(naturalist_rhino::KIND);
    }
    if mob.extra.head.as_deref() == Some("minecraft:carved_pumpkin") {
        seen.effects.push(naturalist_rhino::PUMPKIN);
    }
    use super::naturalist_bears;
    match mob.kind.id.as_str() {
        naturalist_bears::BEAR => seen.effects.push(naturalist_bears::BEAR),
        naturalist_bears::BLACK_BEAR => seen.effects.push(naturalist_bears::BLACK_BEAR),
        _ => {}
    }
    if naturalist_bears::state(&mob.ai.nav).prey {
        seen.effects.push(naturalist_bears::PREY);
    }
    use super::nav::naturalist_frog_food;
    if naturalist_frog_food::includes(&mob.ai.nav) {
        seen.effects.push(naturalist_frog_food::FOOD);
    }
    if mob.kind.id == naturalist_frog_food::FIREFLY {
        seen.effects.push(naturalist_frog_food::FIREFLY);
    }
    if mob.kind.id == "naturalist:rat" {
        seen.effects.push("naturalist:rat");
    }
    use super::goals::naturalist_snake_goals as snake;
    if mob.kind.id == snake::KIND {
        seen.effects.push(snake::KIND);
    }
    if snake::state(&mob.ai.nav).prey {
        seen.effects.push(snake::PREY);
    }
    use super::goals::naturalist_cat_goals as tiger;
    if mob.kind.id == tiger::KIND {
        seen.effects.push(tiger::KIND);
    }
    if tiger::is_tiger_prey(mob) {
        seen.effects.push(tiger::PREY);
    }
    use super::goals::naturalist_lion_goals as lion;
    if mob.kind.id == lion::KIND { seen.effects.push(lion::KIND); }
    if tiger::is_lion_prey(mob) { seen.effects.push(lion::PREY); }
    if mob.kind.id == super::nav::naturalist_hippo_state::KIND {
        seen.effects.push(super::nav::naturalist_hippo_state::KIND);
    }
    if mob.kind.id == super::nav::naturalist_whale_state::KIND {
        seen.effects.push(super::nav::naturalist_whale_state::KIND);
    }
    use super::goals::naturalist_alligator_goals as alligator;
    if mob.kind.id == alligator::KIND {
        seen.effects.push(alligator::KIND);
    }
    if matches!(mob.kind.id.as_str(), "naturalist:deer" | "naturalist:snake" |
        "naturalist:duck" | "naturalist:catfish" | "minecraft:frog" |
        "alexsmobs:catfish" | "alexsmobs:shoebill" | "alexsmobs:terrapin" | "alexsmobs:mudskipper")
    {
        seen.effects.push(alligator::PREY);
    }
    match mob.kind.id.as_str() {
        super::nav::naturalist_elephants::ELEPHANT => seen.effects.push(super::nav::naturalist_elephants::ELEPHANT),
        super::nav::naturalist_elephants::MAMMOTH => seen.effects.push(super::nav::naturalist_elephants::MAMMOTH),
        _ => {}
    }
    if mob.kind.flags.undead {
        seen.effects.push(super::goals::naturalist_mole_goals::UNDEAD);
    }
    if mob.kind.id == super::goals::naturalist_mole_goals::MOLE {
        seen.effects.push(super::goals::naturalist_mole_goals::MOLE);
        if super::goals::naturalist_mole_goals::can_be_dug_out(mob) {
            seen.effects.push(super::goals::naturalist_mole_goals::DIGGABLE);
        }
    } else if mob.kind.id == super::goals::naturalist_mole_goals::TRAIL {
        seen.effects.push(super::goals::naturalist_mole_goals::TRAIL);
    }
    seen
}
