fn deer_scale_attributes(game: &mut Game<'_>, target: Target, attribute: &str) {
    let Target::Mob(id) = target else { return };
    if attribute != "minecraft:scale" || game.cmds.attributes.contains_key(&target) {
        return;
    }
    let Some(mob) = game.mobs.iter().find(|mob| mob.id == id && mob.kind.id.starts_with("naturalist:")) else {
        return;
    };
    let Some(mut attributes) = game.motion.attributes.create("minecraft:cow") else { return };
    if let Some(key) = attributes.catalog().id("minecraft:step_height") {
        attributes.edit(key, |state, observer| state.set_base_value(mob.kind.step_height, observer));
    }
    game.cmds.attributes.insert(target, attributes);
}
