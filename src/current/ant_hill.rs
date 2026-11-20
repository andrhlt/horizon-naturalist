use mod_api::{BlockEntityEvent, BlockEntityPart, Compound, LevelSide, Tag};

#[derive(Clone)]
pub struct AntHill;

#[mod_api::horizon_part]
impl mod_api::BlockPart for AntHill {}

#[mod_api::horizon_part]
impl BlockEntityPart for AntHill {
    fn block_entity(&self, _: LevelSide<'_>, _: &mut BlockEntityEvent<'_>) -> Option<Compound> {
        Some(Compound::from([(
            "Storage".into(),
            Tag::List(Vec::new().into()),
        )]))
    }
}

pub fn register(registrar: &mut dyn mod_api::Registrar) {
    let kind = registrar
        .block_entity_type("ant_hill")
        .blocks(&["ant_hill"])
        .build();
    registrar
        .block_entity_part(&kind, AntHill)
        .expect("register ant hill block entity");
}
