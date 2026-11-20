// Keyframes translated from Naturalist 2.0.7 Java source (MIT).
use super::{Clip, Channel, Interpolation, Target};
pub(super) const ADULT_IDLE: Clip = Clip {
    duration: 2.0, loops: true, channels: &[
        Channel { bone: "antennae", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.5, [-2.0, 0.0, 0.0]),
            (1.4583, [1.98, 0.0, 0.0]),
            (2.0, [0.0, 0.0, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_CRAWL: Clip = Clip {
    duration: 0.625, loops: true, channels: &[
        Channel { bone: "extended", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.0833, [0.0, 1.75, 0.0]),
            (0.1667, [0.0, 3.03, 0.0]),
            (0.2083, [0.0, 3.38, 0.0]),
            (0.25, [0.0, 3.5, 0.0]),
            (0.2917, [0.0, 3.38, 0.0]),
            (0.3333, [0.0, 3.03, 0.0]),
            (0.4167, [0.0, 1.75, 0.0]),
            (0.5, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "butt", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.0417, [-15.53, 0.0, 0.0]),
            (0.0833, [-30.0, 0.0, 0.0]),
            (0.125, [-42.43, 0.0, 0.0]),
            (0.1667, [-51.96, 0.0, 0.0]),
            (0.2083, [-57.96, 0.0, 0.0]),
            (0.25, [-60.0, 0.0, 0.0]),
            (0.2917, [-57.96, 0.0, 0.0]),
            (0.3333, [-51.96, 0.0, 0.0]),
            (0.375, [-42.43, 0.0, 0.0]),
            (0.4167, [-30.0, 0.0, 0.0]),
            (0.4583, [-15.53, 0.0, 0.0]),
            (0.5, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "chest", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.0417, [15.53, 0.0, 0.0]),
            (0.0833, [30.0, 0.0, 0.0]),
            (0.125, [42.43, 0.0, 0.0]),
            (0.1667, [51.96, 0.0, 0.0]),
            (0.2083, [57.96, 0.0, 0.0]),
            (0.25, [60.0, 0.0, 0.0]),
            (0.2917, [57.96, 0.0, 0.0]),
            (0.3333, [51.96, 0.0, 0.0]),
            (0.375, [42.43, 0.0, 0.0]),
            (0.4167, [30.0, 0.0, 0.0]),
            (0.4583, [15.53, 0.0, 0.0]),
            (0.5, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "neck", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.125, [0.0, 0.0, 1.41]),
            (0.2083, [0.0, 0.0, 1.93]),
            (0.2917, [0.0, 0.0, 1.93]),
            (0.4167, [0.0, 0.0, 1.0]),
            (0.5, [0.0, 0.0, 0.0]),
        ] },
    ],
};
