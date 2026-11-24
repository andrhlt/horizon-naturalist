//! Registers Bass with the shared supported aquatic bucket release path.

pub fn register(registrar: &mut dyn mod_api::Registrar) {
    let item = registrar.item_key("naturalist:bass_bucket");
    registrar.item_part(
        item,
        super::ocean_bucket_item::OceanBucket("naturalist:bass"),
    );
}
