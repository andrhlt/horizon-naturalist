//! Marks the Anglerfish bucket for server handled release and dispenser use.
pub struct AnglerfishBucket;

#[mod_api::horizon_part]
impl mod_api::ItemPart for AnglerfishBucket {
    fn empty_contents(
        &self,
        _: &mut dyn mod_api::ServerHost,
        _: &mut mod_api::EmptyContentsEvent<'_>,
    ) -> Option<bool> {
        None
    }

    fn use_item(
        &self,
        host: mod_api::HostSide<'_>,
        _: &mut mod_api::UseItemEvent<'_>,
    ) -> Option<mod_api::InteractionResult> {
        match host {
            mod_api::HostSide::Client(_) => Some(mod_api::InteractionResult::Success),
            mod_api::HostSide::Server(_) => None,
        }
    }
}

pub fn register(registrar: &mut dyn mod_api::Registrar) {
    let item = registrar.item_key("naturalist:anglerfish_bucket");
    registrar.item_part(item, AnglerfishBucket);
}
