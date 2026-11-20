use super::{BlockFeatureError, FeatureBlockBindings};
use crate::{
    block_properties::BoundBlockProperty,
    chunk::BlockState,
    registry::BlockStateId,
    worldgen::{
        DecodeError, ant_hill_layout as layout, feature::FeatureWorld,
    },
};
use horizon_core::random::RandomSource;
use std::{collections::BTreeSet, sync::Arc};

const BLOCK: &str = "naturalist:ant_hill";

#[derive(Clone, Debug)]
pub struct HillFeature {
    big: bool,
    states: [BlockStateId; 5],
    substrate: Arc<BTreeSet<BlockStateId>>,
    leaves: Arc<BTreeSet<BlockStateId>>,
    hill: Arc<BTreeSet<BlockStateId>>,
    replaceable: BoundBlockProperty,
}

pub fn size(name: &str) -> Option<bool> {
    match name {
        "naturalist:ant_hill_small" => Some(false),
        "naturalist:ant_hill_big" => Some(true),
        _ => None,
    }
}

pub fn build(big: bool, bindings: &FeatureBlockBindings<'_>) -> Result<HillFeature, DecodeError> {
    let state = |open: bool, workers, queen: bool| {
        bindings
            .registry()
            .resolve(&BlockState {
                name: BLOCK.to_owned(),
                properties: [
                    ("open".to_owned(), open.to_string()),
                    ("workers".to_owned(), format!("{workers}")),
                    ("has_queen".to_owned(), queen.to_string()),
                ]
                .into(),
            })
            .map_err(|error| DecodeError::Registry(error.to_string()))
    };
    let empty = Arc::new(BTreeSet::new());
    let hill = HillFeature {
        big,
        states: [
            state(false, 0, false)?,
            state(true, 0, false)?,
            state(true, 1, false)?,
            state(true, 2, false)?,
            state(true, 0, true)?,
        ],
        substrate: bindings
            .tags
            .get("minecraft:substrate_overworld")
            .cloned()
            .unwrap_or_else(|| empty.clone()),
        leaves: bindings
            .tags
            .get("minecraft:leaves")
            .cloned()
            .unwrap_or_else(|| empty.clone()),
        hill: bindings.blocks.get(BLOCK).cloned().unwrap_or(empty),
        replaceable: bindings
            .physical_properties
            .as_ref()
            .ok_or(DecodeError::Field("ant hill replaceable property"))?[1]
            .clone(),
    };
    Ok(hill)
}

impl HillFeature {
    pub fn place<W: FeatureWorld>(
        &self,
        world: &mut W,
        random: &mut impl RandomSource,
        origin: [i32; 3],
        mut block_entity: impl FnMut(&mut W, [i32; 3]) -> Result<(), BlockFeatureError<W::Error>>,
    ) -> Result<bool, BlockFeatureError<W::Error>> {
        let [x, y, z] = origin;
        let under = world
            .block([x, y.wrapping_sub(1), z])
            .map_err(BlockFeatureError::World)?;
        if !self.substrate.contains(&under) {
            return Ok(false);
        }
        let rotation = layout::choose_rotation(random);
        for position in layout::positions(self.big, origin, rotation) {
            let candidate = world.block(position).map_err(BlockFeatureError::World)?;
            if !self.replaceable.contains(candidate)
                && !self.leaves.contains(&candidate)
                && !self.hill.contains(&candidate)
            {
                return Ok(false);
            }
        }
        for cell in layout::cells(layout::positions(self.big, origin, rotation), random) {
            let index = if cell.queen {
                4
            } else if cell.open {
                usize::from(cell.workers) + 1
            } else {
                0
            };
            if world
                .set_block(cell.position, self.states[index], 3)
                .map_err(BlockFeatureError::World)?
            {
                block_entity(world, cell.position)?;
            }
        }
        Ok(true)
    }
}
