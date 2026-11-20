fn hedgehog_item_bar(stack: &ItemStack, bar: &mut Option<(i32, u32)>) {
    use horizon_world::nbt::Tag;
    if stack.item.as_ref() != "naturalist:hedgehog" {
        return;
    }
    let health = match stack.custom_data().and_then(|data| data.get(&"Health".into())) {
        Some(Tag::Float(bits)) => f32::from_bits(*bits),
        Some(Tag::Double(bits)) => f64::from_bits(*bits) as f32,
        Some(Tag::Int(health)) => *health as f32,
        _ => 18.0,
    };
    *bar = (health < 18.0).then(|| {
        let fraction = (health / 18.0).clamp(0.0, 1.0);
        let hue = fraction / 3.0 * 6.0;
        let red = (2.0 - hue).clamp(0.0, 1.0);
        let green = hue.clamp(0.0, 1.0);
        let rgb = (((red * 255.0) as u32) << 16) | (((green * 255.0) as u32) << 8);
        ((13.0 * fraction).round() as i32, rgb)
    });
}
