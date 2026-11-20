fn elephant_gear_meta(
    mob: &Mob,
    meta: &mut Vec<(u8, i32, Value)>,
    defaults: &mut Vec<(u8, i32, Value)>,
) {
    use horizon_client::mobs::nav::naturalist_elephants as elephants;
    if !elephants::includes(&mob.kind.id) {
        return;
    }
    let flags = i8::from(mob.extra.saddled)
        | (i8::from(mob.extra.chested) << 1)
        | (i8::from(mob.extra.tame) << 2);
    if flags != 0 {
        meta.push((elephants::GEAR_META, serializer::BYTE, Value::Byte(flags)));
    }
    defaults.push((elephants::GEAR_META, serializer::BYTE, Value::Byte(0)));
    if let Some(Some(stack)) = mob.extra.items.get(elephants::BANNER_SLOT) {
        let tags = horizon_client::save::encode_items(&[Some(stack.clone())]);
        let text = horizon_client::snbt::to_snbt(&tags);
        meta.push((elephants::BANNER_META, serializer::STRING, Value::String(text)));
    }
    defaults.push((elephants::BANNER_META, serializer::STRING, Value::String(String::new())));
}
