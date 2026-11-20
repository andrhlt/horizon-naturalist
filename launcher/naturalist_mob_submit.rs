fn naturalist_mob_submit(
    game: &Game<'_>,
    mob: &NativeMobRender<'_>,
    draw: &mut BlockEntityDraw,
    drawn: &mut bool,
) {
    if *drawn || !mob.kind.starts_with("naturalist:") {
        return;
    }
    *drawn = match mob.kind {
        "naturalist:ant" => game.draw_naturalist_ant(mob, draw),
        "naturalist:butterfly" => game.draw_naturalist_butterfly(mob, draw),
        "naturalist:caterpillar" => game.draw_naturalist_caterpillar(mob, draw),
        "naturalist:dragonfly" => game.draw_naturalist_dragonfly(mob, draw),
        "naturalist:firefly" => game.draw_naturalist_firefly(mob, draw),
        "naturalist:snail" => game.draw_naturalist_snail(mob, draw),
        "naturalist:vulture" => game.draw_naturalist_vulture(mob, draw),
        _ => {
            let Some(entity) = game.mobs.iter().find(|entity| entity.id == mob.id) else {
                return;
            };
            match mob.kind {
                "naturalist:ostrich" => game.draw_naturalist_ostrich(entity, mob, draw),
                "naturalist:alligator" => game.draw_naturalist_alligator(entity, mob, draw),
                "naturalist:anglerfish" => game.draw_naturalist_anglerfish(entity, mob, draw),
                "naturalist:bass" => game.draw_naturalist_bass(entity, mob, draw),
                "naturalist:bear" | "naturalist:black_bear" => game.draw_naturalist_bear(entity, mob, draw),
                "naturalist:bird" => game.draw_naturalist_bird(entity, mob, draw),
                "naturalist:blobfish" => game.draw_naturalist_blobfish(entity, mob, draw),
                "naturalist:boar" | "naturalist:turkey" | "naturalist:zebra"
                | "naturalist:desert_scorpion" | "naturalist:jungle_scorpion"
                | "naturalist:lizard" | "naturalist:lizard_tail"
                | "naturalist:komodo_dragon" | "naturalist:rhino" => {
                    game.draw_naturalist_land_animal(entity, mob, draw)
                }
                "naturalist:capybara" => game.draw_naturalist_capybara(entity, mob, draw),
                "naturalist:catfish" => game.draw_naturalist_catfish(entity, mob, draw),
                "naturalist:clam" => game.draw_naturalist_clam(entity, mob, draw),
                "naturalist:crab" => game.draw_naturalist_crab(entity, mob, draw),
                "naturalist:deer" => game.draw_naturalist_deer(entity, mob, draw),
                "naturalist:duck" => game.draw_naturalist_duck(entity, mob, draw),
                "naturalist:giant_isopod" => game.draw_naturalist_giant_isopod(entity, mob, draw),
                "naturalist:giraffe" => game.draw_naturalist_giraffe(entity, mob, draw),
                "naturalist:elephant" | "naturalist:mammoth" => game.draw_naturalist_elephant(entity, mob, draw),
                "naturalist:whale" => game.draw_naturalist_whale(entity, mob, draw),
                "naturalist:great_white_shark" => game.draw_naturalist_shark(entity, mob, draw),
                "naturalist:hedgehog" => game.draw_naturalist_hedgehog(entity, mob, draw),
                "naturalist:hippo" => game.draw_naturalist_hippo(entity, mob, draw),
                "naturalist:jellyfish" => game.draw_naturalist_jellyfish(entity, mob, draw),
                "naturalist:piranha" => game.draw_naturalist_piranha(entity, mob, draw),
                "naturalist:ray" => game.draw_naturalist_ray(entity, mob, draw),
                "naturalist:mole" => game.draw_naturalist_mole(entity, mob, draw),
                "naturalist:dirt_trail" => game.draw_naturalist_dirt_trail(entity, mob, draw),
                "naturalist:snake" => game.draw_naturalist_snake(entity, mob, draw),
                "naturalist:starfish" => game.draw_naturalist_starfish(entity, mob, draw),
                "naturalist:lion" => game.draw_naturalist_lion(entity, mob, draw),
                "naturalist:tiger" => game.draw_naturalist_tiger(entity, mob, draw),
                "naturalist:tortoise" => game.draw_naturalist_tortoise(entity, mob, draw),
                _ => false,
            }
        }
    };
}
