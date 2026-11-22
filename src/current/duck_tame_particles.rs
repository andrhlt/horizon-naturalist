fn duck_tame_particles(game: &mut Game<'_>, id: u32, event: i8) {
    if !game.mobs.iter().any(|mob| mob.id == id && mob.kind.id == "naturalist:duck") { return; }
    let kind = match event { 6 => "smoke", 7 => "heart", _ => return };
    game.particles_around_mob(id, kind, horizon_client::particles::Burst::TAMING);
}
