# Horizon source notice

`src/current/deer_breed_overwrite.rs` derives from `Game::breed` in
`crates/horizon-client/src/game/mobs/creatures/mod.rs` at Horizon commit
`0a875da91dbb0fdcb7ba8c42c11341aa229af51a`, licensed under LGPL-3.0-or-later;
see [LICENSE-Horizon](LICENSE-Horizon). The adaptation records the inherited
variant of an admitted Deer child using the existing parent RNG coin.

`src/current/ant_sugar.rs` also adapts a short interaction sequence from
Horizon `Game::interact_mob` in the same source file and commit. It applies
the corresponding behavior to Naturalist ants.

Naturalist Java-derived source and original resources have separate notices.

`src/current/ant_carried_save.rs` derives from `Game::save_item` in
`crates/horizon-client/src/game/world/persist.rs` at the same Horizon commit,
and adds Naturalist's carried-food entity ID and save fields.
