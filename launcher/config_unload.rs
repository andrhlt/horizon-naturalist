fn config_unload(config: &NativeConfigFile) {
    if config.mod_id == "naturalist" && config.kind == super::native_configs::NativeConfigKind::Server {
        horizon_client::mobs::naturalist_config::set_snail_crushing(false);
        horizon_client::mobs::naturalist_config::set_removed(Default::default());
    }
}
