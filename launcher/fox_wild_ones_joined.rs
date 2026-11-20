fn fox_wild_ones_joined(game: &mut Game<'_>, from: usize, natural: bool) {
    use horizon_client::mobs::fox::naturalist_wild_ones as wild_ones;

    if natural || wild_ones::PlayerEgg::active() {
        for mob in game.mobs.iter_mut().skip(from) {
            wild_ones::equip(mob);
        }
    }
}
