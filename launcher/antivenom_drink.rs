fn antivenom_drink(game: &mut Game<'_>, item: &ItemStack) {
    use super::naturalist_antivenom as antivenom;
    if item.item.as_ref() == "minecraft:potion"
        && brewing::contents_effects(item).iter().any(|(effect, ..)| *effect == antivenom::ID)
    {
        antivenom::clear_player(game);
    }
}
