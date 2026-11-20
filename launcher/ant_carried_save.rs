fn ant_carried_save(mut tag: Tag, raw: Option<&Tag>) -> Tag {
    if let Some(raw) = raw
        && (raw.get("Ant").is_some()
            || matches!(raw.get("id"), Some(Tag::String(id)) if id.to_string_lossy() == "naturalist:carried_food"))
    {
        put(&mut tag, "id", Tag::String("naturalist:carried_food".into()));
        put(&mut tag, "Age", Tag::Short(0));

        if let Tag::Compound(fields) = &mut tag {
            fields.remove(&"NaturalistUnresolvedSince".into());
            if raw.get("Ant").is_none() && raw.get("NoGravity").is_none() {
                fields.remove(&"NoGravity".into());
            }
        }
    }
    tag
}
