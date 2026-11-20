fn config_read(config: &NativeConfigFile) {
    use super::native_configs::NativeConfigKind;
    use horizon_client::mobs::naturalist_config;
    use std::collections::BTreeSet;

    if config.mod_id != "naturalist" {
        return;
    }
    if config.kind == NativeConfigKind::Client {
        match toml::from_str::<toml::Table>(&config.text) {
            Ok(settings) => naturalist_config::set_glow_goop_tooltip(
                settings.get("glow_goop_tooltip").and_then(toml::Value::as_bool).unwrap_or(true),
            ),
            Err(error) => log::warn!("Invalid Naturalist client config: {error}"),
        }
        return;
    }
    if config.kind != NativeConfigKind::Server {
        return;
    }
    // Preview 0.7.1-preview.1 emitted this duplicate in its default settings.
    let duplicate = "\t\tturkey_removed = false\n\t\ttortoise_removed = false\n";
    let corrected;
    let text = if config.text.contains(duplicate) && config.text.matches("tortoise_removed = ").count() == 2 {
        corrected = config.text.replace(duplicate, "\t\tturkey_removed = false\n");
        corrected.as_str()
    } else {
        config.text.as_str()
    };
    let settings = match toml::from_str::<toml::Table>(text) {
        Ok(settings) => settings,
        Err(error) => {
            log::warn!("Invalid Naturalist server config: {error}");
            return;
        }
    };
    let mut removed = BTreeSet::new();
    if let Some(mobs) = settings.get("disable_mobs").and_then(toml::Value::as_table) {
        let all_bugs = mobs.get("remove_all_bugs").and_then(toml::Value::as_bool) == Some(true);
        if all_bugs {
            removed.extend(["ant", "butterfly", "desert_scorpion", "dragonfly", "firefly", "jungle_scorpion", "snail"]
                .into_iter().map(str::to_owned));
        }
        for group in ["aquatic_mobs", "land_mobs", "bugs"] {
            if let Some(group) = mobs.get(group).and_then(toml::Value::as_table) {
                for (key, value) in group {
                    if value.as_bool() == Some(true) && let Some(mob) = key.strip_suffix("_removed") {
                        removed.insert(mob.to_owned());
                    }
                }
            }
        }
    }
    let crushing = settings.get("behavior")
        .and_then(toml::Value::as_table)
        .and_then(|behavior| behavior.get("snail_crushing"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    naturalist_config::set_snail_crushing(crushing);
    naturalist_config::set_removed(removed);
}
