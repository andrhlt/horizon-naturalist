fn snail_shell_default(item: &str, components: &mut serde_json::Map<String, serde_json::Value>) {
    if item == "naturalist:hedgehog" {
        components.insert(
            "minecraft:enchantable".into(),
            serde_json::json!({ "value": 1 }),
        );
    }
    if item == "naturalist:snail_shell" {
        components.insert("minecraft:custom_data".into(), serde_json::json!({"Color": 12}));
    }
}
