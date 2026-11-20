// Keyframes translated from Naturalist 2.0.7 Java source (MIT).
use super::{Clip, Channel, Interpolation, Target};
pub(super) const ADULT_FLY: Clip = Clip {
    duration: 0.5, loops: true, channels: &[
        Channel { bone: "body", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.125, [-2.0, 0.0, 0.0]),
            (0.3333, [1.73, 0.0, 0.0]),
            (0.4583, [1.0, 0.0, 0.0]),
            (0.5, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "body", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -0.19, 0.0]),
            (0.25, [0.0, 0.19, 0.0]),
            (0.5, [0.0, -0.19, 0.0]),
        ] },
        Channel { bone: "rightWing", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [8.21, 0.0, 0.0]),
            (0.0417, [-3.47, 0.0, 54.52]),
            (0.0833, [-11.68, 0.0, 54.52]),
            (0.125, [-8.21, 0.0, 0.0]),
            (0.1667, [3.47, 0.0, -54.52]),
            (0.2083, [11.68, 0.0, -54.52]),
            (0.25, [8.21, 0.0, 0.0]),
            (0.2917, [-3.47, 0.0, 54.52]),
            (0.3333, [-11.68, 0.0, 54.52]),
            (0.375, [-8.21, 0.0, 0.0]),
            (0.4167, [3.47, 0.0, -54.52]),
            (0.4583, [11.68, 0.0, -54.52]),
            (0.5, [8.21, 0.0, 0.0]),
        ] },
        Channel { bone: "leftWing", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [8.21, 0.0, 0.0]),
            (0.0417, [-3.47, 0.0, -54.52]),
            (0.0833, [-11.68, 0.0, -54.52]),
            (0.125, [-8.21, 0.0, 0.0]),
            (0.1667, [3.47, 0.0, 54.52]),
            (0.2083, [11.68, 0.0, 54.52]),
            (0.25, [8.21, 0.0, 0.0]),
            (0.2917, [-3.47, 0.0, -54.52]),
            (0.3333, [-11.68, 0.0, -54.52]),
            (0.375, [-8.21, 0.0, 0.0]),
            (0.4167, [3.47, 0.0, 54.52]),
            (0.4583, [11.68, 0.0, 54.52]),
            (0.5, [8.21, 0.0, 0.0]),
        ] },
    ],
};
