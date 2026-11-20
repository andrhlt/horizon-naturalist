fn glow_goop_marker(main_hand: Option<&ItemStack>) -> std::ops::ControlFlow<Option<&'static str>> {
    if main_hand.is_some_and(|stack| stack.count > 0 && stack.item.as_ref() == "naturalist:glow_goop") {
        std::ops::ControlFlow::Break(Some("naturalist:glow_goop"))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
