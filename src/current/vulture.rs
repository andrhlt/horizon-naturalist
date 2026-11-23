//! The supported part of the Java Vulture meal loop: pick up rotten flesh and
//! consume one held item after the 600-tick hunger interval.
use mod_api::{AttachmentType, EntityRef, EquipmentSlot, HostTransaction, ItemPlace, ItemResource, Holder};

static MEAL_TICKS: AttachmentType<i32> = AttachmentType {
    namespace: "naturalist",
    name: "vulture_meal_ticks",
    default: || 0,
    codec: None,
    copy_on_death: false,
    sync: false,
};

pub fn tick(host: &mut dyn mod_api::ServerHost, vulture: EntityRef) {
    let holder = Holder::Entity(vulture.id());
    let Ok(mut ticks) = MEAL_TICKS.get(host, holder) else { return };
    let hand = ItemPlace::Equipment { entity: vulture.id(), slot: EquipmentSlot::MainHand };
    let held = hand.stack(host);
    if held.as_ref().is_some_and(|stack| stack.item().id(host) == "minecraft:rotten_flesh") {
        if ticks > 0 {
            ticks -= 1;
            let _ = MEAL_TICKS.set(host, holder, ticks);
        } else if let Some(stack) = held {
            let _ = replace_hand(host, &hand, stack, 1);
            let _ = MEAL_TICKS.set(host, holder, 600);
        }
        return;
    }
    let Some(level) = vulture.level(host) else { return };
    let Some(pos) = vulture.position(host) else { return };
    for entity in level.entities_in(host, [pos[0] - 2.0, pos[1] - 2.0, pos[2] - 2.0], [pos[0] + 2.0, pos[1] + 2.0, pos[2] + 2.0]).unwrap_or_default() {
        let Some(item) = host.item_entity(entity) else { continue };
        let Some(mut stack) = item.stack(host) else { continue };
        if stack.item().id(host) != "minecraft:rotten_flesh" { continue; }
        let moved = host.transaction(|host, tx| Ok::<u32, ()>(hand.insert(host, tx, &ItemResource::of(&stack), stack.count())))
            .unwrap_or(0);
        if moved == stack.count() {
            entity.discard(host);
        } else if moved > 0 {
            let _ = stack.set_count(stack.count() - moved);
            let _ = item.set_stack(host, stack);
        }
        if moved > 0 {
            let _ = MEAL_TICKS.set(host, holder, 600);
        }
        break;
    }
}

fn replace_hand(host: &mut dyn mod_api::ServerHost, hand: &ItemPlace, mut stack: mod_api::ItemStack, amount: u32) -> Result<(), ()> {
    let old = stack.count();
    if old <= amount { stack.set_count(1).map_err(|_| ())?; }
    else { stack.set_count(old - amount).map_err(|_| ())?; }
    host.transaction(|host, tx| {
        let existing = hand.stack(host).ok_or(())?;
        let resource = ItemResource::of(&existing);
        if hand.extract(host, tx, &resource, old) != old { return Err(()); }
        if old > amount && hand.insert(host, tx, &ItemResource::of(&stack), old - amount) != old - amount { return Err(()); }
        Ok(())
    })
}
