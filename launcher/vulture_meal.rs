use super::{Eater, Mob, MobEvent, MobWorld, teleport_randomly};
use crate::{
    item_sync::{consumable, use_remainder},
    items::{ConsumeEffect, ItemStack},
};
use horizon_core::{position::BlockPos, random::RandomSource};
use horizon_world::{consumable::UseAnimation, registry::BlockStateId};

pub fn finish(
    world: &MobWorld,
    mob: &mut Mob,
    events: &mut Vec<MobEvent>,
    mut stack: ItemStack,
) -> Option<ItemStack> {
    let Some(items) = world.items else {
        return Some(stack);
    };
    let Some(consumable) = consumable(&stack, items) else {
        return Some(stack);
    };

    let _ = consumable.emit(&mut Eater { m: mob, events }, (), stack.clone(), 16);
    let pitch = mob.random.triangle_float(1.0, 0.4);
    events.push(MobEvent::SoundId(
        consumable.sound.clone(),
        mob.position.components(),
        1.0,
        pitch,
    ));
    if let Some(data) = stack.extra.as_deref() {
        for (id, duration) in &data.stew_effects {
            if let Some(effect) = crate::brewing::effect(id) {
                mob.add_effect(effect.id, *duration, 0);
            }
        }
    }
    for effect in &consumable.effects {
        match effect {
            ConsumeEffect::TeleportRandomly(teleport) => {
                let start = events.len();
                teleport_randomly(world, mob, events, teleport);
                for event in &mut events[start..] {
                    if let MobEvent::Sound(sound, ..) = event
                        && *sound == "entity.fox.teleport"
                    {
                        *sound = "item.chorus_fruit.teleport";
                    }
                }
            }
            ConsumeEffect::PlaySound(sound) => {
                let block = BlockPos::containing(mob.position);
                let at = [block.x, block.y, block.z].map(|axis| f64::from(axis) + 0.5);
                events.push(MobEvent::SoundId(sound.clone(), at, 1.0, 1.0));
            }
            other => mob.apply_consume_effect(other),
        }
    }
    events.push(MobEvent::GameEvent {
        event: if consumable.animation == UseAnimation::Drink {
            "minecraft:drink"
        } else {
            "minecraft:eat"
        },
        at: mob.position.components(),
        source: mob.id,
        state: BlockStateId(0),
    });

    stack.count -= 1;
    if stack.count > 0 {
        return Some(stack);
    }
    let remainder = items.get(&stack.item).and_then(|info| info.use_remainder.clone());
    use_remainder(&stack, remainder).map(|item| ItemStack::new(&item, 1))
}
