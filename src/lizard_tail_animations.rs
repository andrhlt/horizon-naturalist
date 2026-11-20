// Keyframes translated from Naturalist 2.0.7 Java source (MIT).
use super::{Clip, Channel, Interpolation, Target};
pub(super) const ADULT_FLOP: Clip = Clip {
    duration: 1.0, loops: true, channels: &[
        Channel { bone: "root", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [16.0, 0.0, 0.0]),
            (0.0417, [13.86, 0.0, 0.0]),
            (0.0833, [8.0, 0.0, 0.0]),
            (0.1667, [-8.0, 0.0, 0.0]),
            (0.2083, [-13.86, 0.0, 0.0]),
            (0.25, [-16.0, 0.0, 0.0]),
            (0.2917, [-13.86, 0.0, 0.0]),
            (0.3333, [-8.0, 0.0, 0.0]),
            (0.4167, [8.0, 0.0, 0.0]),
            (0.4583, [13.86, 0.0, 0.0]),
            (0.5, [16.0, 0.0, 0.0]),
            (0.5417, [13.86, 0.0, 0.0]),
            (0.5833, [8.0, 0.0, 0.0]),
            (0.6667, [-8.0, 0.0, 0.0]),
            (0.7083, [-13.86, 0.0, 0.0]),
            (0.75, [-16.0, 0.0, 0.0]),
            (0.7917, [-13.86, 0.0, 0.0]),
            (0.8333, [-8.0, 0.0, 0.0]),
            (0.9167, [8.0, 0.0, 0.0]),
            (0.9583, [13.86, 0.0, 0.0]),
            (1.0, [16.0, 0.0, 0.0]),
        ] },
        Channel { bone: "root", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 1.0, 0.0]),
        ] },
    ],
};
