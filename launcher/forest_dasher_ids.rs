fn forest_dasher_ids(plan: &Plan<'_>, name: &str, doc: &mut J) {
    let mut ids = Vec::new();
    if plan.entity_types.iter().any(|entity| entity.id == "naturalist:deer") {
        ids.extend(["naturalist:forest_dasher",
            "naturalist:long_forest_dasher", "naturalist:strong_forest_dasher"]);
    }
    if plan.entity_types.iter().any(|entity| entity.id == "naturalist:desert_scorpion") {
        ids.push("naturalist:antivenom");
    }
    if ids.is_empty() {
        return;
    }
    if name == "command-registries.json" {
        if let Some(J::Arr(list)) = doc.at("by_registry/minecraft:potion/ids") {
            for id in ids {
                let id = J::Str(id.to_owned());
                if !list.contains(&id) {
                    list.push(id);
                }
            }
        }
    } else if name == "registries.json"
        && let Some(J::Obj(entries)) = doc.get_mut("minecraft:potion").and_then(|r| r.get_mut("entries"))
    {
        let mut id = entries.iter()
            .filter_map(|(_, entry)| {
                let J::Obj(fields) = entry else { return None; };
                fields.iter().find(|(key, _)| key == "protocol_id")?.1.as_u32()
            })
            .max().map_or(0, |max| max + 1);
        for potion in ids {
            if entries.iter().any(|(name, _)| name == potion) {
                continue;
            }
            entries.push((potion.to_owned(), J::Obj(vec![("protocol_id".into(), J::Num(id.into()))])));
            id += 1;
        }
    }
}
