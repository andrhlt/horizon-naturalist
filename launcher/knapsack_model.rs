fn knapsack_model(property: &str, stack: &StackView, value: &mut bool) {
    if property == "naturalist:knapsack_filled" {
        *value = stack.patch.get("minecraft:custom_data")
            .and_then(Option::as_ref)
            .and_then(|data| data.get("id"))
            .is_some();
    }
}
