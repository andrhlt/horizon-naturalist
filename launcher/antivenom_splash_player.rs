fn antivenom_splash_player(game: &mut Game<'_>, effects: &[(&str, i32, i32)], at: Vec3, age: u32) {
    if effects.iter().any(|(id, ..)| *id == super::naturalist_antivenom::ID) {
        naturalist_antivenom_splash::clear_player_at(game, at, age);
    }
}
