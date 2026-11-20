fn jellyfish_translucent(game: &Game<'_>, mob: &Mob, partial: f32, camera: DVec3, light: u32, out: &mut Vec<Vertex>) -> std::ops::ControlFlow<bool> {
    if mob.kind.id != "naturalist:jellyfish" {
        return std::ops::ControlFlow::Continue(());
    }
    let start = out.len();
    let drawn = game.mod_entity_draw(mob, partial, camera, light, out, |render, draw| {
        let mut drawn = false;
        game.native_mob_submit(render, draw, &mut drawn);
        drawn
    });
    if drawn {
        for vertex in &mut out[start..] {
            vertex.layer |= horizon_client::gfx::OIT_ENTITY;
        }
    }
    std::ops::ControlFlow::Break(drawn)
}
