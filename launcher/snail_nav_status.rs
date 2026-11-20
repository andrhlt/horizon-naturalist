fn snail_nav_status(done: bool, nav: &Navigation) -> bool {
    done && nav
        .__horizon_mixins_naturalist_snail_state()
        .crawl_target
        .is_none()
}
