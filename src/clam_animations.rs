// Keyframes translated from Naturalist 2.0.7 Java source (MIT).
use super::{Clip, Channel, Interpolation, Target};
pub(super) const ADULT_OPEN: Clip = Clip {
    duration: 1.25, loops: false, channels: &[
        Channel { bone: "top", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.125, [-10.45, 0.0, 0.0]),
            (0.375, [-38.42, 0.0, 0.0]),
            (0.5, [-51.53, 0.0, 0.0]),
            (0.625, [-63.24, 0.0, 0.0]),
            (0.75, [-72.98, 0.0, 0.0]),
            (0.875, [-79.93, 0.0, 0.0]),
            (1.0, [-82.8, 0.0, 0.0]),
        ] },
        Channel { bone: "hinge", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.25, [-0.85, 0.0, 0.0]),
            (0.375, [-2.3, 0.0, 0.0]),
            (0.5, [-5.26, 0.0, 0.0]),
            (0.75, [-18.87, 0.0, 0.0]),
            (0.875, [-20.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightItem", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.3125, [0.0, 0.0, 0.0]),
            (0.5625, [1.05, 1.05, 1.05]),
            (0.6875, [1.0, 1.0, 1.0]),
        ] },
    ],
};
pub(super) const ADULT_CLOSE: Clip = Clip {
    duration: 0.5, loops: false, channels: &[
        Channel { bone: "top", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-80.0, 0.0, 0.0]),
            (0.5, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "hinge", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-17.5, 0.0, 0.0]),
            (0.4, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightItem", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.1, [1.0, 1.0, 1.0]),
            (0.3, [0.0, 0.0, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_IDLE_OPEN: Clip = Clip {
    duration: 2.0, loops: true, channels: &[
        Channel { bone: "top", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-79.6, 0.0, 0.0]),
            (1.125, [-81.33, 0.0, 0.0]),
            (2.0, [-79.6, 0.0, 0.0]),
        ] },
        Channel { bone: "hinge", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-17.5, 0.0, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_IDLE_CLOSED: Clip = Clip {
    duration: 2.0, loops: true, channels: &[
        Channel { bone: "rightItem", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
        ] },
    ],
};
