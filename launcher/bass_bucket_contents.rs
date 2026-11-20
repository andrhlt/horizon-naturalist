fn bass_bucket_contents(item: &str) -> std::ops::ControlFlow<Option<(horizon_world::level::BucketContent, &str, Option<&str>)>> {
    use std::ops::ControlFlow;
    let mob = match item {
        "naturalist:bass_bucket" => "naturalist:bass",
        "naturalist:catfish_bucket" => "naturalist:catfish",
        "naturalist:piranha_bucket" => "naturalist:piranha",
        "naturalist:blobfish_bucket" => "naturalist:blobfish",
        "naturalist:jellyfish_bucket" => "naturalist:jellyfish",
        "naturalist:starfish_bucket" => "naturalist:starfish",
        "naturalist:giant_isopod_bucket" => "naturalist:giant_isopod",
        "naturalist:anglerfish_bucket" => "naturalist:anglerfish",
        "naturalist:ray_bucket" => "naturalist:ray",
        _ => return ControlFlow::Continue(()),
    };
    ControlFlow::Break(Some((horizon_world::level::BucketContent::MobWater, "minecraft:item.bucket.empty_fish", Some(mob))))
}
