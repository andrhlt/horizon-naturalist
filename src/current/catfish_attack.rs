use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use horizon_client::mobs::Damage;

static BITES: LazyLock<Mutex<HashMap<u32, (u32, u8)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn set_cooldown(game: &mut super::Game<'_>, entity: u32, ticks: i32) {
    use horizon_core::nbt::Tag;
    let raw = game
        .persist
        .raw
        .entry(entity)
        .or_insert_with(|| Tag::Compound(Default::default()));
    if let Tag::Compound(fields) = raw {
        fields.insert("HuntingCooldown".into(), Tag::Int(ticks));
    }
}

fn cooldown(game: &super::Game<'_>, entity: u32) -> i32 {
    use horizon_core::nbt::Tag;
    game.persist
        .raw
        .get(&entity)
        .and_then(|raw| raw.get("HuntingCooldown"))
        .and_then(|tag| match tag {
            Tag::Int(value) => Some(*value),
            Tag::Short(value) => Some(i32::from(*value)),
            Tag::Byte(value) => Some(i32::from(*value)),
            _ => None,
        })
        .unwrap_or(0)
}

pub(super) fn attack(
    game: &mut super::Game<'_>,
    target: u32,
    source: Damage,
) -> std::ops::ControlFlow<bool> {
    use horizon_client::items::ItemStack;
    use horizon_client::mobs::Who;
    use std::ops::ControlFlow;

    let Some(Who::Mob(attacker)) = source.attacker else {
        return ControlFlow::Continue(());
    };
    let Some(predator) = game
        .mobs
        .iter()
        .position(|mob| mob.id == attacker && mob.kind.id == "naturalist:catfish" && mob.alive())
    else {
        return ControlFlow::Continue(());
    };
    let Some(prey) = game.mobs.iter().position(|mob| {
        mob.id == target
            && mob.alive()
            && (mob.kind.id == "naturalist:bass"
                || matches!(
                    mob.kind.id.as_str(),
                    "minecraft:cod"
                        | "minecraft:salmon"
                        | "minecraft:tropical_fish"
                        | "minecraft:pufferfish"
                ))
    }) else {
        return ControlFlow::Continue(());
    };
    if cooldown(game, attacker) > 0 {
        return ControlFlow::Break(true);
    }

    set_cooldown(game, attacker, 2400);
    if let Ok(mut bites) = BITES.lock() {
        bites.insert(attacker, (target, 8));
    }
    // Java's Catfish bite is a delayed devour; the contact itself deals no
    // damage and the prey remains alive until the eighth attacker tick.
    let _ = prey;
    ControlFlow::Break(true)
}

pub(super) fn tick(game: &mut super::Game<'_>, entity: u32, kind: &str) {
    if kind != "naturalist:catfish" {
        return;
    }
    let ticks = cooldown(game, entity);
    if ticks > 0 {
        set_cooldown(game, entity, ticks - 1);
    }
    let due = if let Ok(mut bites) = BITES.lock() {
        match bites.get_mut(&entity) {
            Some((prey, remaining)) if *remaining <= 1 => {
                let prey = *prey;
                bites.remove(&entity);
                Some(prey)
            }
            Some((_, remaining)) => {
                *remaining -= 1;
                None
            }
            None => None,
        }
    } else {
        None
    };
    let Some(prey_id) = due else { return };
    let Some(prey) = game.mobs.iter().position(|mob| mob.id == prey_id && mob.alive()) else {
        return;
    };
    let at = game.mobs[prey].position;
    game.mobs[prey].removed = true;
    game.spawn_at_location(at, 0.0, horizon_client::items::ItemStack::new("minecraft:bone_meal", 1));
}
