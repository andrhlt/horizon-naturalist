fn deer_placement_substrate(
    level: &GameLevel<'_, '_>,
    tag: &str,
    block: &str,
) -> std::ops::ControlFlow<bool> {
    use super::naturalist_deer_placement::{Substrate, substrate};
    use std::ops::ControlFlow;

    let species = substrate(tag);
    if species == Substrate::Vanilla {
        return ControlFlow::Continue(());
    }
    if species == Substrate::Scorpion {
        return ControlFlow::Break(
            level
                .w
                .block_tag("naturalist:scorpions_spawnable_on", block),
        );
    }
    if species == Substrate::Mammoth {
        return ControlFlow::Break(["minecraft:substrate_overworld", "minecraft:snow", "minecraft:ice"]
            .iter().any(|tag| level.w.block_tag(tag, block)));
    }
    if species == Substrate::Hippo {
        use horizon_client::blocks::Fluid;
        use super::naturalist_deer_placement::{Pass, hippo_position};

        let Some((pass, [x, ground, z])) = hippo_position() else {
            return ControlFlow::Break(false);
        };
        let y = ground + 1;
        let world = &level.w;
        if !world.block_tag("minecraft:substrate_overworld", block)
            || world.sky_light(x, y, z).max(world.block_light(x, y, z)) <= 8
        {
            return ControlFlow::Break(false);
        }
        let (mut west, mut east, mut north, mut south) = (x - 16, x + 16, z - 16, z + 16);
        if pass == Pass::Generation {
            west = west.max(x & !15);
            east = east.min(x | 15);
            north = north.max(z & !15);
            south = south.min(z | 15);
        }
        if !world.world.may_hold([west, y - 1, north], [east, y + 1, south], |state| {
            world.facts.get(state).fluid == Fluid::Water
        }) {
            return ControlFlow::Break(false);
        }
        return ControlFlow::Break((west..=east).any(|check_x| (y - 1..=y + 1).any(|check_y| {
            (north..=south).any(|check_z| world.fluid(check_x, check_y, check_z) == Fluid::Water)
        })));
    }
    if species == Substrate::Crab {
        return ControlFlow::Break(matches!(block, "minecraft:stone" | "minecraft:gravel")
            || level.w.block_tag("minecraft:sand", block)
            || level.w.block_tag("minecraft:substrate_overworld", block));
    }
    if species == Substrate::Komodo {
        return ControlFlow::Break(
            level
                .w
                .block_tag("naturalist:komodo_dragons_spawnable_on", block),
        );
    }
    let insect = match species {
        Substrate::Butterfly => Some("naturalist:butterflies_spawnable_on"),
        Substrate::Dragonfly => Some("naturalist:dragonflies_spawnable_on"),
        Substrate::Vulture => Some("naturalist:vultures_spawnable_on"),
        _ => None,
    };
    if let Some(on) = insect {
        return ControlFlow::Break(level.w.block_tag(on, block));
    }
    let water = species == Substrate::Duck && block == "minecraft:water";
    ControlFlow::Break(water || level.w.block_tag("minecraft:substrate_overworld", block))
}
