fn deer_variant_bred_pair(game: &mut Game<'_>, parent: u32, partner: u32) {
    super::naturalist_deer_variants::record_breeding_partner(game, parent, partner);
}
