use super::Navigation;

const ROBIN: &str = "naturalist:entity.bird.ambient_robin";

#[derive(Clone, Debug)]
pub struct Voice {
    call: &'static str,
    note: Option<[f64; 3]>,
}

impl Default for Voice {
    fn default() -> Self {
        Self { call: ROBIN, note: None }
    }
}

pub fn set_call(nav: &mut Navigation, variant: &str) {
    nav.__horizon_mixins_naturalist_bird_voice_mut().call = match variant.split_once(':').map_or(variant, |(_, path)| path) {
        "blue_jay" | "stellers_jay" => "naturalist:entity.bird.ambient_bluejay",
        "northern_cardinal" => "naturalist:entity.bird.ambient_cardinal",
        "carolina_chickadee" | "tufted_titmouse" => "naturalist:entity.bird.ambient_finch",
        "red_winged_blackbird" => "naturalist:entity.bird.ambient_canary",
        "white_throated_sparrow" => "naturalist:entity.bird.ambient_sparrow",
        _ => ROBIN,
    };
}

pub fn call(nav: &Navigation) -> &'static str {
    nav.__horizon_mixins_naturalist_bird_voice().call
}

pub fn request_note(nav: &mut Navigation, at: [f64; 3]) {
    nav.__horizon_mixins_naturalist_bird_voice_mut().note = Some(at);
}

pub fn take_note(nav: &mut Navigation) -> Option<[f64; 3]> {
    nav.__horizon_mixins_naturalist_bird_voice_mut().note.take()
}
