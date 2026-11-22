fn duck_breed_tick(game: &mut Game<'_>, entity: u32, kind: &str) {
    use mod_api::Host;
    if !matches!(kind, "naturalist:duck" | "minecraft:chicken") { return; }
    let Some(index)=game.mobs.iter().position(|m|m.id==entity) else { return; };
    if game.mobs[index].ai.goals.has("NaturalistDuckChickenBreed") { return; }
    let Some(entity_ref)=game.entity(entity) else { return; };
    let Some(handle)=game.mob(entity_ref) else { return; };
    handle.remove_goal(game,"breed");
    if let Some(m)=game.mobs.iter_mut().find(|m|m.id==entity) { crate::game::naturalist_duck_breed::add(m); }
}
