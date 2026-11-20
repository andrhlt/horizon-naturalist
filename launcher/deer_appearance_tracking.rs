fn deer_appearance_tracking(game: &Game<'_>, host: &mut LanHost) {
    // Source mixins do not register a dynamic mod object. Let the stock
    // tracker send attachment snapshots to guests who start seeing deer.
    host.hub.sync_attachments |= game.res.mob_types().contains_key("naturalist:deer")
        || game.res.mob_types().contains_key("naturalist:duck");
}
