//! The Giant Isopod's short hide response and shell protection.

use mod_api::{AttributeModifier, AttributeOperation, Identifier, ModifierLifetime, Tag};

const HIDE_TICKS: i32 = 10;
const PLAYER_RANGE_SQUARED: f64 = 25.0;
const SHELL_ARMOR_MODIFIER: &str = "naturalist:isopod_shell_armor";
const SHELL_KNOCKBACK_MODIFIER: &str = "naturalist:isopod_shell_knockback";

pub(crate) fn should_hide(
    adult: bool,
    distance_squared: f64,
    creative_or_spectator: bool,
    invisible: bool,
    visible: bool,
) -> bool {
    adult
        && !creative_or_spectator
        && !invisible
        && visible
        && distance_squared < PLAYER_RANGE_SQUARED
}

fn age(data: &mod_api::Compound) -> i32 {
    match data.get(&"Age".into()) {
        Some(Tag::Int(value)) => *value,
        Some(Tag::Short(value)) => i32::from(*value),
        Some(Tag::Byte(value)) => i32::from(*value),
        _ => 0,
    }
}

fn hiding_ticks(data: &mod_api::Compound) -> i32 {
    match data.get(&"NaturalistHidingTicks".into()) {
        Some(Tag::Int(value)) => *value,
        _ => 0,
    }
}

fn with_hiding_ticks(data: &mod_api::Compound, ticks: i32) -> mod_api::Compound {
    let mut next = data.clone();
    next.insert("NaturalistHidingTicks".into(), Tag::Int(ticks.max(0)));
    next
}

fn threatened_by_player(host: &mut dyn mod_api::ServerHost, entity: mod_api::EntityRef) -> bool {
    let (Some(level), Some(position), Some(data)) =
        (entity.level(host), entity.position(host), entity.nbt(host))
    else {
        return false;
    };
    let is_adult = age(&data) >= 0;
    if !is_adult {
        return false;
    }
    level.online_players(host).is_some_and(|players| {
        players.into_iter().any(|player| {
            let player_entity = player.entity();
            let Some(player_position) = player_entity.position(host) else {
                return false;
            };
            let dx = position[0] - player_position[0];
            let dy = position[1] - player_position[1];
            let dz = position[2] - player_position[2];
            let distance_squared = dx * dx + dy * dy + dz * dz;
            let creative = player.game_mode(host).is_some_and(|mode| {
                matches!(
                    mode,
                    mod_api::GameMode::Creative | mod_api::GameMode::Spectator
                )
            });
            let invisible = player_entity.is_invisible(host) == Some(true);
            let visible = host
                .living(player_entity)
                .and_then(|living| living.has_line_of_sight(host, entity))
                == Some(true);
            should_hide(is_adult, distance_squared, creative, invisible, visible)
        })
    })
}

fn update_shell(host: &mut dyn mod_api::ServerHost, entity: mod_api::EntityRef, active: bool) {
    let Some(living) = host.living(entity) else {
        return;
    };
    if active {
        for (attribute, id, amount) in [
            ("minecraft:armor", SHELL_ARMOR_MODIFIER, 1.0),
            (
                "minecraft:knockback_resistance",
                SHELL_KNOCKBACK_MODIFIER,
                0.75,
            ),
        ] {
            let Some(modifier_id) = Identifier::parse(id).ok() else {
                continue;
            };
            let exists = living
                .attribute_modifiers(host, attribute)
                .is_some_and(|modifiers| modifiers.iter().any(|value| value.id == modifier_id));
            if !exists {
                living.add_attribute_modifier(
                    host,
                    attribute,
                    AttributeModifier::new(modifier_id, amount, AttributeOperation::AddValue),
                    ModifierLifetime::Transient,
                );
            }
        }
    } else {
        for (attribute, id) in [
            ("minecraft:armor", SHELL_ARMOR_MODIFIER),
            ("minecraft:knockback_resistance", SHELL_KNOCKBACK_MODIFIER),
        ] {
            let Some(modifier_id) = Identifier::parse(id).ok() else {
                continue;
            };
            living.remove_attribute_modifier(host, attribute, &modifier_id);
        }
    }
}

/// Seed the ten-tick hide window before the mob's first AI step.
pub(crate) fn joined(host: &mut dyn mod_api::ServerHost, id: u32) {
    let Some(entity) = host.entity(id) else {
        return;
    };
    let active = threatened_by_player(host, entity);
    let Some(data) = entity.nbt(host) else {
        return;
    };
    if active {
        let next = with_hiding_ticks(&data, HIDE_TICKS);
        entity.merge_nbt(host, next);
    }
    update_shell(host, entity, active);
}

/// Refresh the threat timer and keep shell modifiers in step with it.
pub(crate) fn tick(host: &mut dyn mod_api::ServerHost, id: u32) {
    let Some(entity) = host.entity(id) else {
        return;
    };
    let Some(data) = entity.nbt(host) else {
        return;
    };
    let current = hiding_ticks(&data);
    let next = if threatened_by_player(host, entity) {
        HIDE_TICKS
    } else {
        current.saturating_sub(1).max(0)
    };
    if next != current {
        entity.merge_nbt(host, with_hiding_ticks(&data, next));
    }
    update_shell(host, entity, next > 0);
}

/// Apply the Java shell's 50% damage cap after ordinary reductions.
pub(crate) fn cap_shell_damage(container: &mut mod_api::DamageContainer) {
    let cap = container.original() * 0.5;
    container.set_new_damage(container.new_damage().min(cap));
}

#[cfg(test)]
mod tests {
    use super::{cap_shell_damage, should_hide};
    use mod_api::DamageContainer;

    #[test]
    fn hide_requires_an_adult_and_a_close_visible_noncreative_player() {
        assert!(should_hide(true, 1.0, false, false, true));
        assert!(!should_hide(false, 1.0, false, false, true));
        assert!(!should_hide(true, 25.0, false, false, true));
        assert!(!should_hide(true, 1.0, true, false, true));
        assert!(!should_hide(true, 1.0, false, true, true));
        assert!(!should_hide(true, 1.0, false, false, false));
    }

    #[test]
    fn shell_caps_damage_after_ordinary_reductions() {
        let mut container = DamageContainer::new(4.0);
        container.set_new_damage(2.8);
        cap_shell_damage(&mut container);
        assert_eq!(container.new_damage(), 2.0);

        let mut already_below_cap = DamageContainer::new(4.0);
        already_below_cap.set_new_damage(1.5);
        cap_shell_damage(&mut already_below_cap);
        assert_eq!(already_below_cap.new_damage(), 1.5);
    }
}
