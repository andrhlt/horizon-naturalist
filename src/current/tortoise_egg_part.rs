use std::sync::atomic::{AtomicBool, Ordering};

use mod_api::{BlockPart, FallOnEvent, LandingWrite, LevelRandomHost, RandomSource};

static MOB_GRIEFING: AtomicBool = AtomicBool::new(true);

pub struct TortoiseEggPart;

pub(crate) fn set_mob_griefing(enabled: bool) {
    MOB_GRIEFING.store(enabled, Ordering::Relaxed);
}

#[mod_api::horizon_part]
impl BlockPart for TortoiseEggPart {
    fn fall_on(&self, host: &mut dyn LevelRandomHost, event: &mut FallOnEvent<'_>) {
        if host.state_block(event.state) != Some("naturalist:tortoise_egg")
            || event.kind == "naturalist:tortoise"
            || is_zombie(event.kind)
            || is_safe_egg_walker(event.kind)
            || event.kind == "minecraft:item"
            || (event.kind != "minecraft:player" && !MOB_GRIEFING.load(Ordering::Relaxed))
        {
            return;
        }

        let Some(mut random) = host.random() else {
            return;
        };
        let breaks_egg = random.next_int_bound(3).unwrap_or(1) == 0;
        let _ = host.set_random(random);
        if !breaks_egg {
            return;
        }

        let Some(state_name) = host.state_string(event.state) else {
            return;
        };
        let Some((block, properties)) = state_name.split_once('[') else {
            return;
        };
        let Some(properties) = properties.strip_suffix(']') else {
            return;
        };
        let mut eggs = None;
        let mut next_properties = Vec::new();
        for property in properties.split(',') {
            let Some((name, value)) = property.split_once('=') else {
                return;
            };
            if name == "eggs" {
                let Ok(count) = value.parse::<u8>() else {
                    return;
                };
                eggs = Some(count);
                next_properties.push(format!("eggs={}", count.saturating_sub(1)));
            } else {
                next_properties.push(property.to_owned());
            }
        }
        let Some(eggs) = eggs else {
            return;
        };
        if eggs <= 1 {
            event.request(LandingWrite::DestroyBlock { drop: false });
        } else {
            let next_name = format!("{block}[{}]", next_properties.join(","));
            if let Some(next) = host.block_state_id(&next_name) {
                event.request(LandingWrite::SetBlock(next));
            }
        }
    }
}

fn is_zombie(kind: &str) -> bool {
    matches!(
        kind,
        "minecraft:zombie"
            | "minecraft:zombie_villager"
            | "minecraft:husk"
            | "minecraft:drowned"
            | "minecraft:zombified_piglin"
    )
}

fn is_safe_egg_walker(kind: &str) -> bool {
    matches!(
        kind,
        "naturalist:butterfly"
            | "naturalist:bird"
            | "naturalist:dragonfly"
            | "naturalist:firefly"
            | "minecraft:bat"
            | "alexsmobs:shoebill"
    )
}

pub fn register(registrar: &mut dyn mod_api::Registrar) {
    let key = registrar.block_key("naturalist:tortoise_egg");
    registrar.block_part(key, TortoiseEggPart);
}
