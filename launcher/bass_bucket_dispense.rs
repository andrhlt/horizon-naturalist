fn bass_bucket_dispense(game: &mut Game<'_>, pos: Pos, facing: Direction, stack: &ItemStack) -> std::ops::ControlFlow<Result<Dispensed>> {
    use std::ops::ControlFlow;
    if !matches!(stack.item.as_ref(), "naturalist:bass_bucket" | "naturalist:catfish_bucket" | "naturalist:jellyfish_bucket" | "naturalist:ray_bucket" | "naturalist:anglerfish_bucket" | "naturalist:starfish_bucket" | "naturalist:giant_isopod_bucket") {
        return ControlFlow::Continue(());
    }
    ControlFlow::Break(game.dispense_filled_bucket(pos, facing, stack.clone()))
}
