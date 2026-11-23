//! Whale breeding is available only when two in-love whales are both in water.

use mod_api::{GoalSpec, ServerHost, Tag};

const BREED_RANGE: f64 = 8.0;
fn pair_can_breed(current_wet: bool, current_loved: bool, mate_is_wet_and_loved: bool) -> bool {
    current_wet && current_loved && mate_is_wet_and_loved
}
pub(crate) const FOOD_ITEMS: [&str; 6] = [
    "minecraft:cod",
    "minecraft:cooked_cod",
    "minecraft:salmon",
    "minecraft:cooked_salmon",
    "naturalist:crab_meat",
    "naturalist:cooked_crab_meat",
];

fn is_food(item: &str) -> bool {
    FOOD_ITEMS.contains(&item)
}

fn injured(host: &dyn mod_api::Host, id: i32) -> Option<mod_api::LivingRef> {
    let entity = host.entity(u32::try_from(id).ok()?)?;
    let living = host.living(entity)?;
    (living.health(host)? < living.max_health(host)?).then_some(living)
}

pub(crate) fn interact(
    host: mod_api::HostSide<'_>,
    event: &mut mod_api::EntityInteractEvent<'_>,
) -> Option<mod_api::InteractionResult> {
    let Some(stack) = event.stack else {
        return None;
    };
    let item = stack.item().id(match &host {
        mod_api::HostSide::Server(host) => &**host,
        mod_api::HostSide::Client(host) => &**host,
    });
    if event.kind != "naturalist:whale" || !is_food(&item) {
        return None;
    }
    match host {
        // A client can't query remote mob health through this API. Claim the
        // known Whale-food click locally; the server decides repair vs feed.
        mod_api::HostSide::Client(_) => Some(mod_api::InteractionResult::Success),
        mod_api::HostSide::Server(host) => {
            let living = injured(host, event.entity)?;
            let player = host.players().acting_player()?;
            let slot = match event.hand {
                mod_api::Hand::Main => player.selected_slot(host)? as u32,
                mod_api::Hand::Off => 40,
            };
            let stack = player
                .inventory_slots(host)?
                .into_iter()
                .find(|(candidate, _)| *candidate == slot)
                .map(|(_, stack)| stack)?;
            if !is_food(&stack.item().id(host)) || stack.count() == 0 {
                return None;
            }
            let creative = player.game_mode(host) == Some(mod_api::GameMode::Creative);
            if !creative {
                let mut remaining = stack.clone();
                let replacement = if stack.count() > 1 {
                    remaining.set_count(stack.count() - 1).ok()?;
                    Some(remaining)
                } else {
                    None
                };
                player.set_inventory_slot(host, slot, replacement).ok()?;
            }
            if living.heal(host, 4.0) {
                Some(mod_api::InteractionResult::SuccessServer)
            } else {
                if !creative {
                    let _ = player.set_inventory_slot(host, slot, Some(stack));
                }
                None
            }
        }
    }
}

fn in_love(host: &mut dyn ServerHost, entity: mod_api::EntityRef) -> bool {
    entity
        .nbt(host)
        .and_then(|data| match data.get(&"InLove".into()) {
            Some(Tag::Int(ticks)) => Some(*ticks > 0),
            _ => None,
        })
        .unwrap_or(false)
}

fn eligible_pair(host: &mut dyn ServerHost, entity: mod_api::EntityRef) -> bool {
    let current_wet = entity.is_in_water(host) == Some(true);
    let current_loved = in_love(host, entity);
    if !current_wet || !current_loved {
        return false;
    }
    let (Some(level), Some(position)) = (entity.level(host), entity.position(host)) else {
        return false;
    };
    let radius = BREED_RANGE;
    level
        .entities_in(
            host,
            [position[0] - radius, position[1] - radius, position[2] - radius],
            [position[0] + radius, position[1] + radius, position[2] + radius],
        )
        .is_some_and(|entities| {
            let mate_is_wet_and_loved = entities.into_iter().any(|mate| {
                mate != entity
                    && mate
                        .kind(host)
                        .is_some_and(|kind| kind.id(host) == "naturalist:whale")
                    && mate.is_in_water(host) == Some(true)
                    && in_love(host, mate)
            });
            pair_can_breed(current_wet, current_loved, mate_is_wet_and_loved)
        })
}

/// The Cow template supplies a generic BreedGoal. Keep it present only when
/// Java's aquatic pair precondition is met, including the prospective mate.
pub(crate) fn update_breeding_goal(host: &mut dyn ServerHost, id: u32) {
    let Some(entity) = host.entity(id) else { return };
    let Some(mob) = host.mob(entity) else { return };
    let has_goal = mob
        .goals(host)
        .is_some_and(|goals| goals.iter().any(|goal| goal.id == "breed"));
    if eligible_pair(host, entity) {
        if !has_goal
            && !mob.add_goal(host, 2, &GoalSpec::Breed { speed: 1.0 })
        {
            eprintln!("Naturalist: failed to add Whale breed goal");
        }
    } else if has_goal {
        mob.remove_goal(host, "breed");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pinned_whale_food_list_has_six_items_and_excludes_other_fish() {
        assert_eq!(super::FOOD_ITEMS.len(), 6);
        assert!(!super::FOOD_ITEMS.contains(&"minecraft:tropical_fish"));
        assert!(!super::FOOD_ITEMS.contains(&"minecraft:pufferfish"));
        assert!(!super::FOOD_ITEMS.contains(&"minecraft:beef"));
        assert!(super::is_food("minecraft:cod"));
        assert!(!super::is_food("minecraft:tropical_fish"));
    }

    #[test]
    fn whale_breeding_requires_both_adults_in_water_and_in_love() {
        assert!(!super::pair_can_breed(false, true, true));
        assert!(!super::pair_can_breed(true, false, true));
        assert!(!super::pair_can_breed(true, true, false));
        assert!(super::pair_can_breed(true, true, true));
    }
}
