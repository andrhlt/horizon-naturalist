fn hedgehog_offspring(game: &mut Game<'_>, parent: u32, child: u32) -> std::ops::ControlFlow<()> {
    let Some(parent) = game.mobs.iter().find(|mob| mob.id == parent
        && mob.kind.id == "naturalist:hedgehog") else {
        return std::ops::ControlFlow::Continue(());
    };
    let color = if parent.extra.color <= 15 { parent.extra.color } else { 14 };
    let owner = parent.extra.owner;
    if let Some(young) = game.mobs.iter_mut().find(|mob| mob.id == child) {
        young.extra.color = color;
        young.extra.owner = owner;
        young.extra.tame = owner.is_some();
    }
    std::ops::ControlFlow::Break(())
}
