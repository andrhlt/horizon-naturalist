use super::{Game, native_block_entity_draw::BlockEntityDraw};
use horizon_client::{
    containers::BlockEntity,
    items::ItemStack,
    mobs::{Who, render::DYE_NAMES},
    models::catalog::{BakedFace, BakedModel},
};
use horizon_world::{
    nbt::Tag,
    registry::{BlockRegistry, BlockStateId},
};

pub(super) const SHELL: &str = "naturalist:snail_shell";

fn potted_state(game: &Game<'_>, plant: &str) -> Option<BlockStateId> {
    let plant = plant.strip_prefix("minecraft:")?;
    let pot = match plant {
        "cactus" | "bamboo" => return None,
        "azalea" => "azalea_bush",
        "flowering_azalea" => "flowering_azalea_bush",
        _ => plant,
    };
    game.pack
        .assets
        .registry
        .default_state(&format!("minecraft:potted_{pot}"))
}

pub(super) fn fill(game: &mut Game<'_>, stack: &ItemStack, pos: [i32; 3]) -> bool {
    let state = game.world.block(pos[0], pos[1], pos[2]);
    if game.res.facts().get(state).name.as_ref() != SHELL
        || potted_state(game, &stack.item).is_none()
    {
        return false;
    }
    let Some(plant) = game.pack.assets.registry.default_state(&stack.item) else {
        return false;
    };
    let mut flower = stack.clone();
    flower.count = 1;
    let flower = horizon_client::save::item_to_nbt(&flower);
    if !game
        .native_with_mod_block_entity(pos, |entity| {
            if entity.nbt.contains_key(&"Flower".into()) {
                return false;
            }
            entity.nbt.insert("Flower".into(), flower);
            true
        })
        .unwrap_or(false)
    {
        return false;
    }

    game.native_block_entity_sync(pos);
    game.consume_held();
    let sounds = game.res.facts().get(plant).sounds.clone();
    let at = pos.map(|n| f64::from(n) + 0.5);
    game.native_play_sound_in_place(
        &sounds.place,
        at,
        4,
        (sounds.volume + 1.0) / 2.0,
        sounds.pitch * 0.8,
    );
    let context = game.who_context(Some(Who::Player), Some(state));
    game.post_game_event("minecraft:block_change", at, context);
    true
}

pub(super) fn loot(
    registry: &BlockRegistry,
    state: BlockStateId,
    flower: Option<&Tag>,
) -> Option<Vec<ItemStack>> {
    let row = registry.state(state)?;
    if row.name.as_str() != SHELL {
        return None;
    }
    let color = row
        .properties
        .get("color")
        .and_then(|color| DYE_NAMES.iter().position(|name| *name == color.as_str()))
        .unwrap_or(12);
    let mut shell = ItemStack::new(SHELL, 1);
    shell.data_mut().set_other(
        "minecraft:custom_data",
        Some(Tag::compound([("Color", Tag::Int(color as i32))])),
    );
    let mut drops = vec![shell];
    if let Some(flower) = flower.and_then(horizon_client::save::item_from_nbt) {
        drops.push(flower);
    }
    Some(drops)
}

pub(super) fn flower(entity: Option<&BlockEntity>) -> Option<&Tag> {
    match entity {
        Some(BlockEntity::Mod(entity)) => entity.nbt.get(&"Flower".into()),
        _ => None,
    }
}

pub(super) fn drops(
    game: &Game<'_>,
    pos: [i32; 3],
    state: BlockStateId,
    drops: &mut Vec<ItemStack>,
) {
    if let Some(shell) = loot(
        game.pack.assets.registry,
        state,
        flower(game.world.block_entities.get(&(pos[0], pos[1], pos[2]))),
    ) {
        *drops = shell;
    }
}

pub(super) fn draw(
    game: &Game<'_>,
    state: BlockStateId,
    flower: Option<&Tag>,
    draw: &mut BlockEntityDraw,
) {
    let Some(row) = game.pack.assets.registry.state(state) else {
        return;
    };
    let rotation = row
        .properties
        .get("rotation")
        .and_then(|value| value.parse::<f32>().ok())
        .unwrap_or(0.0);
    let color = row.properties.get("color").map_or("brown", String::as_str);

    draw.push();
    draw.translate([0.5, 0.5, 0.5]);
    draw.rotate_about([0.0, 1.0, 0.0], -rotation * 22.5, [0.0; 3]);
    draw.translate([0.0, -0.28125, 0.15625]);
    draw.rotate_about([1.0, 0.0, 0.0], 90.0, [0.0; 3]);
    draw.translate([-0.5, -0.21875, -0.65625]);
    game.native_draw_block(draw, SHELL, &[("color", color), ("rotation", "0")]);
    draw.pop();

    if let Some(flower) = flower.and_then(horizon_client::save::item_from_nbt)
        && let Some(potted) = potted_state(game, &flower.item)
        && let Ok(models) = game.pack.render.model_catalog.collect_seeded(potted, 42)
    {
        draw.push();
        draw.translate([0.0, 0.15, 0.0]);
        let models = models
            .into_iter()
            .map(|(_, model)| BakedModel {
                ambient_occlusion: model.ambient_occlusion,
                particle: model.particle,
                faces: model
                    .faces
                    .iter()
                    .filter(|face| {
                        !matches!(
                            face.quad.material.sprite.as_str(),
                            "minecraft:block/flower_pot"
                                | "minecraft:block/dirt"
                                | "minecraft:block/potted_azalea_bush_side"
                                | "minecraft:block/potted_azalea_bush_top"
                                | "minecraft:block/potted_flowering_azalea_bush_side"
                                | "minecraft:block/potted_flowering_azalea_bush_top"
                        )
                    })
                    .map(|face| {
                        BakedFace::new(
                            face.quad.clone(),
                            face.layer,
                            face.opaque,
                            face.translucent,
                            face.fusion,
                        )
                    })
                    .collect(),
                cube: None,
            })
            .collect::<Vec<_>>();
        let light = draw.light;
        let pose = draw.pose();
        game.baked_model_vertices(
            &models,
            pose,
            |_| light,
            true,
            |_| None,
            draw.vertices_mut(),
        );
        draw.pop();
    }
    draw.keep();
}
