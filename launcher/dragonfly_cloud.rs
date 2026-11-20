use super::{Game, Tag};
use horizon_client::mobs::Who;

impl Game<'_> {
    pub(in crate::game) fn spawn_naturalist_dragonfly_cloud(&mut self, owner: u32, at: [f64; 3]) {
        let string = |value: &str| Tag::String(value.into());
        let effect = Tag::compound([
            ("id", string("minecraft:instant_damage")),
            ("duration", Tag::Int(1)),
            ("amplifier", Tag::Int(1)),
        ]);
        let tag = Tag::compound([
            ("Pos", Tag::List(at.into_iter().map(|coord| Tag::Double(coord.to_bits())).collect())),
            ("Radius", Tag::Float(0.5f32.to_bits())),
            ("Duration", Tag::Int(200)),
            ("potion_contents", Tag::compound([("custom_effects", Tag::List(vec![effect]))])),
            ("custom_particle", Tag::compound([
                ("type", string("minecraft:dragon_breath")),
                ("power", Tag::Float(1.0f32.to_bits())),
            ])),
        ]);
        if let Some(id) = self.load_potion_cloud(&tag)
            && let Some(cloud) = self.breath_clouds.last_mut().filter(|cloud| cloud.id == id)
            && let Some(potion) = &mut cloud.potion
        {
            potion.owner = Some(Who::Mob(owner));
        }
    }
}
