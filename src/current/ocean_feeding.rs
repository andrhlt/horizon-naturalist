//! Java-matched fish healing and Whale feeding interactions.

fn profile(kind: &str) -> Option<(&'static [&'static str], f32)> {
    match kind {
        "naturalist:whale" => Some((&crate::whale_behavior::FOOD_ITEMS, 4.0)),
        "naturalist:great_white_shark" => Some((&SHARK_FOOD, 2.0)),
        _ => None,
    }
}

const SHARK_FOOD: [&str; 5] = [
    "minecraft:cod",
    "minecraft:cooked_cod",
    "minecraft:salmon",
    "minecraft:cooked_salmon",
    "minecraft:tropical_fish",
];

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
    let Some((foods, heal_amount)) = profile(event.kind) else {
        return None;
    };
    let item = stack.item().id(match &host {
        mod_api::HostSide::Server(host) => &**host,
        mod_api::HostSide::Client(host) => &**host,
    });
    if !foods.contains(&item.as_str()) {
        return None;
    }
    match host {
        // Remote mob health is not readable on the client. Claim recognized
        // food locally and let the server choose healing or normal Whale feed.
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
            if !foods.contains(&stack.item().id(host).as_str()) || stack.count() == 0 {
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
            if living.heal(host, heal_amount) {
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

#[cfg(test)]
mod tests {
    use super::{profile, SHARK_FOOD};

    #[test]
    fn shark_food_matches_java_healing_items_without_breeding_items() {
        let (foods, heal) = profile("naturalist:great_white_shark").unwrap();
        assert_eq!(heal, 2.0);
        assert_eq!(foods, SHARK_FOOD);
        assert!(!foods.contains(&"minecraft:pufferfish"));
        assert!(!foods.contains(&"minecraft:beef"));
        assert!(!foods.contains(&"minecraft:tropical_fish_bucket"));
        assert_eq!(profile("naturalist:whale").unwrap().1, 4.0);
    }
}
