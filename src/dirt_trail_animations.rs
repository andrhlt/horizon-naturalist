// Keyframes translated from Naturalist 2.0.7 Java source (MIT).
use super::{Clip, Channel, Interpolation, Target};
pub(super) const ADULT_SPAWN: Clip = Clip {
    duration: 0.625, loops: false, channels: &[
        Channel { bone: "mound", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -4.0]),
            (0.0417, [3.46, 0.0, -2.0]),
            (0.0625, [4.0, 0.0, 0.0]),
            (0.1042, [2.0, 0.0, 3.46]),
            (0.125, [0.0, 0.0, 4.0]),
            (0.1667, [-3.46, 0.0, 2.0]),
            (0.1875, [-4.0, 0.0, 0.0]),
            (0.2083, [-3.46, 0.0, -2.0]),
            (0.2292, [-1.67, 0.0, -2.89]),
            (0.2708, [1.0, 0.0, -1.73]),
            (0.3125, [0.67, 0.0, 0.0]),
            (0.3333, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "mound", target: Target::Position, interpolation: Interpolation::CatmullRom, keys: &[
            (0.0, [0.0, -1.0, 0.0]),
            (0.2083, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "mound", target: Target::Scale, interpolation: Interpolation::CatmullRom, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.125, [1.0, 1.0, 1.0]),
        ] },
    ],
};
