fn glow_goop_navigation(
    plan: &Plan<'_>,
    list: &mut Vec<J>,
    ids: bool,
    gameplay: bool,
) -> std::ops::ControlFlow<Result<()>> {
    use std::{cell::Cell, ops::ControlFlow};
    thread_local! {
        static REWRITING: Cell<bool> = const { Cell::new(false) };
    }
    // mob-data.json has positional [path type, flags, collision top] rows.
    if ids || gameplay || REWRITING.get()
        || !matches!(list.first(), Some(J::Arr(row)) if matches!(row.as_slice(),
            [J::Str(kind), J::Num(_), J::Num(_)] if kind == "OPEN"))
    {
        return ControlFlow::Continue(());
    }
    let Some(block) = plan.blocks.iter().find(|block| block.decl.id == "naturalist:glow_goop") else {
        return ControlFlow::Continue(());
    };
    let Some(&dry) = plan.defaults.get(&block.decl.template) else {
        return ControlFlow::Continue(());
    };
    REWRITING.set(true);
    let result = plan.states(list, ids, gameplay);
    REWRITING.set(false);
    if result.is_ok() {
        use crate::mobs::data::{AIR, LAND, VALID_EMPTY_SPAWN, WATER};
        for (index, (template, _)) in (block.rename.first..).zip(block.state_combinations()) {
            let waterlogged = template != dry;
            let path = if waterlogged { "WATER" } else { "OPEN" };
            let flags = LAND | WATER | AIR | if waterlogged { 0 } else { VALID_EMPTY_SPAWN };
            list[index as usize] = J::Arr(vec![J::Str(path.into()), J::Num(flags.into()), J::f64(0.0)]);
        }
    }
    ControlFlow::Break(result)
}
