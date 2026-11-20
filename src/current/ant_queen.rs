pub struct QueenAnt;
#[mod_api::horizon_part]
impl mod_api::ItemPart for QueenAnt {
    fn use_on(
        &self,
        host: mod_api::HostSide<'_>,
        _: &mut mod_api::UseOnEvent<'_>,
    ) -> Option<mod_api::InteractionResult> {
        match host {
            mod_api::HostSide::Client(_) => Some(mod_api::InteractionResult::Success),
            mod_api::HostSide::Server(_) => None,
        }
    }
}
pub fn register(registrar: &mut dyn mod_api::Registrar) {
    let item = registrar.item_key("naturalist:queen_ant");
    registrar.item_part(item, QueenAnt);
}
