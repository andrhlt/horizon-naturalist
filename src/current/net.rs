//! Java BugNetItem fallback recipes and full-NBT caught entity items.
#[derive(Clone)]
struct CaptureNet;
pub fn register(r: &mut dyn mod_api::Registrar) {
    r.register_recipe_type("naturalist:net");
    r.register_raw_recipe_serializer("naturalist:net", "naturalist:net");
    let net = r.item_key("naturalist:capture_net");
    r.item_part(net, CaptureNet);
    for id in [
        "bee",
        "silverfish",
        "endermite",
        "dragonfly",
        "firefly",
        "butterfly",
        "caterpillar",
        "scorpion",
        "snail",
    ] {
        let key = r.item_key(&format!("naturalist:{id}"));
        r.item_part(key, CaughtItem);
    }
    r.particle_key("naturalist:capture_net_swing");
    r.particle_key("minecraft:poof");
}
#[mod_api::horizon_part]
impl mod_api::ItemPart for CaptureNet {
    fn item_interact_mob(
        &self,
        _: mod_api::HostSide<'_>,
        _: &mut mod_api::ItemInteractMobEvent<'_>,
    ) -> Option<mod_api::InteractionResult> {
        None
    }
    fn use_item(
        &self,
        host: mod_api::HostSide<'_>,
        _: &mut mod_api::UseItemEvent<'_>,
    ) -> Option<mod_api::InteractionResult> {
        Some(match host {
            mod_api::HostSide::Server(_) => mod_api::InteractionResult::SuccessServer,
            mod_api::HostSide::Client(_) => mod_api::InteractionResult::Success,
        })
    }
}

#[derive(Clone)]
struct CaughtItem;
#[mod_api::horizon_part]
impl mod_api::ItemPart for CaughtItem {
    fn use_item(
        &self,
        _: mod_api::HostSide<'_>,
        _: &mut mod_api::UseItemEvent<'_>,
    ) -> Option<mod_api::InteractionResult> {
        None
    }
}
