// Keyframes translated from Naturalist 2.0.7 Java source (MIT).
use super::{Clip, Channel, Interpolation, Target};
pub(super) const ADULT_SWIM: Clip = Clip {
    duration: 1.0, loops: true, channels: &[
        Channel { bone: "body", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 5.0, 0.0]),
            (0.1667, [0.0, 2.5, 0.0]),
            (0.4167, [0.0, -4.33, 0.0]),
            (0.5417, [0.0, -4.83, 0.0]),
            (0.6667, [0.0, -2.5, 0.0]),
            (0.9167, [0.0, 4.33, 0.0]),
            (1.0, [0.0, 5.0, 0.0]),
        ] },
        Channel { bone: "tail", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.0833, [0.0, 11.25, 0.0]),
            (0.125, [0.0, 15.91, 0.0]),
            (0.1667, [0.0, 19.49, 0.0]),
            (0.2083, [0.0, 21.73, 0.0]),
            (0.25, [0.0, 22.5, 0.0]),
            (0.2917, [0.0, 21.73, 0.0]),
            (0.3333, [0.0, 19.49, 0.0]),
            (0.375, [0.0, 15.91, 0.0]),
            (0.4583, [0.0, 5.82, 0.0]),
            (0.5833, [0.0, -11.25, 0.0]),
            (0.625, [0.0, -15.91, 0.0]),
            (0.6667, [0.0, -19.49, 0.0]),
            (0.7083, [0.0, -21.73, 0.0]),
            (0.75, [0.0, -22.5, 0.0]),
            (0.7917, [0.0, -21.73, 0.0]),
            (0.8333, [0.0, -19.49, 0.0]),
            (0.875, [0.0, -15.91, 0.0]),
            (0.9583, [0.0, -5.82, 0.0]),
            (1.0, [0.0, 0.0, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_FLOP: Clip = Clip {
    duration: 0.5, loops: true, channels: &[
        Channel { bone: "body", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -90.0]),
            (0.0833, [0.0, -16.0, -90.0]),
            (0.25, [0.0, 9.0, -90.0]),
            (0.4167, [0.0, 18.0, -90.0]),
            (0.5, [0.0, 0.0, -90.0]),
        ] },
        Channel { bone: "body", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -1.8, 0.0]),
            (0.0833, [0.0, -1.3, 0.5]),
            (0.125, [0.0, 1.6409, 0.326]),
            (0.1667, [0.0, 2.5786, 0.1558]),
            (0.2083, [0.0, 3.0239, 0.0418]),
            (0.2917, [0.0, 3.2061, -0.06]),
            (0.3333, [0.0, 3.0127, -0.2201]),
            (0.375, [0.0, 2.4068, -0.4557]),
            (0.4167, [0.0, 0.1, -0.7]),
            (0.5, [0.0, -1.8, 0.0]),
        ] },
        Channel { bone: "right_fin", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 13.0, 0.0]),
            (0.0833, [0.0, 36.0, 0.0]),
            (0.1667, [0.0, 13.0, 0.0]),
            (0.25, [0.0, 36.0, 0.0]),
            (0.3333, [0.0, 13.0, 0.0]),
            (0.4167, [0.0, 36.0, 0.0]),
            (0.5, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "left_fin", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -13.0, 0.0]),
            (0.0833, [0.0, -36.0, 0.0]),
            (0.1667, [0.0, -13.0, 0.0]),
            (0.25, [0.0, -36.0, 0.0]),
            (0.3333, [0.0, -13.0, 0.0]),
            (0.4167, [0.0, -36.0, 0.0]),
            (0.5, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "tail", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 21.0, 0.0]),
            (0.0833, [0.0, 6.5, 0.0]),
            (0.1667, [0.0, -12.0, 0.0]),
            (0.4583, [0.0, 26.0, 0.0]),
            (0.5, [0.0, 21.0, 0.0]),
        ] },
        Channel { bone: "root", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -0.75, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_ATTACK: Clip = Clip {
    duration: 0.75, loops: false, channels: &[
        Channel { bone: "body", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.2083, [-10.0, 0.0, 0.0]),
            (0.2917, [7.5, 0.0, 0.0]),
            (0.4583, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "body", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.2083, [0.0, 0.2, 0.1]),
            (0.2917, [0.0, 0.0, -0.1]),
            (0.4583, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "body", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.2083, [1.0, 1.1, 1.0]),
            (0.2917, [1.2, 0.8, 1.0]),
            (0.4583, [1.0, 1.0, 1.0]),
        ] },
        Channel { bone: "jaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.2083, [90.0, 0.0, 0.0]),
            (0.2917, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "jaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.2083, [0.0, -0.2, 0.2]),
            (0.2917, [0.0, -0.15, 0.0]),
            (0.4583, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "jaw", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.25, [1.0, 1.0, 1.0]),
            (0.2917, [1.2, 0.9, 1.1]),
            (0.3333, [1.1734, 0.9133, 1.0867]),
            (0.4167, [1.0453, 0.9774, 1.0226]),
            (0.4583, [1.0, 1.0, 1.0]),
        ] },
        Channel { bone: "fin_bottom", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.25, [0.0, 0.0, 1.0]),
            (0.3333, [0.0, 0.0, 0.0]),
        ] },
    ],
};
