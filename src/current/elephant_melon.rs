//! Java Elephant owner's melon healing interaction.
use mod_api::{Host, HostSide, InteractionResult, ItemInteractMobEvent, Tag};

#[derive(Clone)]
struct ElephantMelon;

#[mod_api::horizon_part]
impl mod_api::ItemPart for ElephantMelon {
    fn item_interact_mob(
        &self,
        side: HostSide<'_>,
        event: &mut ItemInteractMobEvent<'_>,
    ) -> Option<InteractionResult> {
        let HostSide::Server(host) = side else {
            return None;
        };
        if !matches!(event.kind, "naturalist:elephant" | "naturalist:mammoth")
            || mod_api::__private::item_key_id(event.stack.item()) != Some("minecraft:melon_slice")
        {
            return None;
        }

        let player = host.players().acting_player()?;
        let entity = host.entity(event.entity as u32)?;
        let data = entity.nbt(host)?;
        if !matches!(data.get(&"Tame".into()), Some(Tag::Byte(value)) if *value != 0) {
            return None;
        }
        let Some(Tag::IntArray(owner)) = data.get(&"Owner".into()) else {
            return None;
        };
        let uuid = player.uuid();
        let owner_uuid = [
            (uuid >> 96) as u32 as i32,
            (uuid >> 64) as u32 as i32,
            (uuid >> 32) as u32 as i32,
            uuid as u32 as i32,
        ];
        if owner.as_slice() != owner_uuid {
            return None;
        }

        let living = host.living(entity)?;
        let (health, max_health) = (living.health(host)?, living.max_health(host)?);
        if health >= max_health {
            return None;
        }

        let creative = player.is_creative(host).unwrap_or(false);
        if !creative {
            let stack = event.stack.clone();
            let player_living = host.living(player.entity())?;
            let _ = player_living.set_hand_item(host, mod_api::Hand::Main, stack.shrink(1));
        }
        let _ = living.set_health(host, (health + 4.0).min(max_health));
        Some(InteractionResult::SuccessServer)
    }
}

pub fn register(registrar: &mut dyn mod_api::Registrar) {
    let melon = registrar.item_key("minecraft:melon_slice");
    registrar.item_part(melon, ElephantMelon);
}
