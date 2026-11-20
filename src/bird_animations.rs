// Keyframes translated from Naturalist 2.0.7 Java source (MIT).
use super::{Clip, Channel, Interpolation, Target};
pub(super) const ADULT_IDLE: Clip = Clip {
    duration: 4.0, loops: true, channels: &[
        Channel { bone: "body", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [25.0, 0.0, 0.0]),
        ] },
        Channel { bone: "body", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 1.0]),
        ] },
        Channel { bone: "tail", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [34.0, 0.0, 0.0]),
            (0.5833, [32.44, 0.0, 0.0]),
            (1.7917, [26.21, 0.0, 0.0]),
            (2.375, [26.67, 0.0, 0.0]),
            (3.625, [33.33, 0.0, 0.0]),
            (4.0, [34.0, 0.0, 0.0]),
        ] },
        Channel { bone: "wings", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [5.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftWing", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [22.5, 2.35, 0.0]),
            (1.0417, [22.5, -3.38, 0.0]),
            (1.5833, [22.5, -3.84, 0.0]),
            (2.0, [22.5, -2.35, 0.0]),
            (3.0417, [22.5, 3.38, 0.0]),
            (3.5833, [22.5, 3.84, 0.0]),
            (4.0, [22.5, 2.35, 0.0]),
        ] },
        Channel { bone: "right_leg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 7.5, 0.0]),
        ] },
        Channel { bone: "left_leg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -12.5, 0.0]),
        ] },
        Channel { bone: "headRot", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-22.5, 0.0, 0.0]),
        ] },
        Channel { bone: "headRot", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -0.575, 0.0]),
        ] },
        Channel { bone: "rightWing", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [22.5, -2.35, 0.0]),
            (1.0417, [22.5, 3.38, 0.0]),
            (1.5833, [22.5, 3.84, 0.0]),
            (2.0, [22.5, 2.35, 0.0]),
            (3.0417, [22.5, -3.38, 0.0]),
            (3.5833, [22.5, -3.84, 0.0]),
            (4.0, [22.5, -2.35, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_WALK: Clip = Clip {
    duration: 0.4167, loops: true, channels: &[
        Channel { bone: "body", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [25.0, 0.0, 0.0]),
        ] },
        Channel { bone: "body", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 1.0]),
        ] },
        Channel { bone: "tail", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [30.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftWing", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [22.5, 2.35, 0.0]),
            (0.4167, [22.5, -0.1, 0.0]),
        ] },
        Channel { bone: "right_leg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 7.5, 0.0]),
        ] },
        Channel { bone: "left_leg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -12.5, 0.0]),
        ] },
        Channel { bone: "bodyRot", target: Target::Position, interpolation: Interpolation::CatmullRom, keys: &[
            (0.125, [0.0, 0.0, 0.0]),
            (0.25, [0.0, 3.0, 0.0]),
            (0.375, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "headRot", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-22.5, 0.0, 0.0]),
        ] },
        Channel { bone: "headRot", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -0.575, 0.0]),
        ] },
        Channel { bone: "rightWingShoulderRot", target: Target::Rotation, interpolation: Interpolation::CatmullRom, keys: &[
            (0.1667, [0.0, 0.0, 0.0]),
            (0.1677, [0.0, 0.0, 0.0]),
            (0.3333, [0.0, 0.0, 22.5]),
            (0.4167, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftWingShoulderRot", target: Target::Rotation, interpolation: Interpolation::CatmullRom, keys: &[
            (0.1667, [0.0, 0.0, 0.0]),
            (0.1677, [0.0, 0.0, 0.0]),
            (0.3333, [0.0, 0.0, -22.5]),
            (0.4167, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightWing", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [22.5, -2.35, 0.0]),
            (0.4167, [22.5, 0.1, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_FLY: Clip = Clip {
    duration: 1.0, loops: true, channels: &[
        Channel { bone: "bodyRot", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [90.0, 0.0, 0.0]),
        ] },
        Channel { bone: "bodyRot", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -3.0, -3.0]),
        ] },
        Channel { bone: "tail", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -0.9]),
        ] },
        Channel { bone: "leftWing", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 45.0]),
            (0.0417, [0.0, 0.0, 38.97]),
            (0.0833, [0.0, 0.0, 22.5]),
            (0.1667, [0.0, 0.0, -22.5]),
            (0.2083, [0.0, 0.0, -38.97]),
            (0.25, [0.0, 0.0, -45.0]),
            (0.2917, [0.0, 0.0, -38.97]),
            (0.3333, [0.0, 0.0, -22.5]),
            (0.4167, [0.0, 0.0, 22.5]),
            (0.4583, [0.0, 0.0, 38.97]),
            (0.5, [0.0, 0.0, 45.0]),
            (0.5417, [0.0, 0.0, 38.97]),
            (0.5833, [0.0, 0.0, 22.5]),
            (0.6667, [0.0, 0.0, -22.5]),
            (0.7083, [0.0, 0.0, -38.97]),
            (0.75, [0.0, 0.0, -45.0]),
            (0.7917, [0.0, 0.0, -38.97]),
            (0.8333, [0.0, 0.0, -22.5]),
            (0.9167, [0.0, 0.0, 22.5]),
            (0.9583, [0.0, 0.0, 38.97]),
            (1.0, [0.0, 0.0, 45.0]),
        ] },
        Channel { bone: "headRot", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-90.0, 0.0, 0.0]),
        ] },
        Channel { bone: "headRot", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 1.5, -1.5]),
        ] },
        Channel { bone: "legs", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-45.0, 0.0, 0.0]),
        ] },
        Channel { bone: "legs", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.5, -0.5]),
        ] },
        Channel { bone: "comb", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [22.5, 0.0, 0.0]),
        ] },
        Channel { bone: "leftWingRot", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-178.0, -88.0, 90.0]),
        ] },
        Channel { bone: "leftWingRot", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-0.5, -0.7627, -0.2025]),
        ] },
        Channel { bone: "rightWingRot", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-178.0, 88.0, -90.0]),
        ] },
        Channel { bone: "rightWingRot", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.5, -0.7627, -0.2025]),
        ] },
        Channel { bone: "rightWing", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -45.0]),
            (0.0417, [0.0, 0.0, -38.97]),
            (0.0833, [0.0, 0.0, -22.5]),
            (0.1667, [0.0, 0.0, 22.5]),
            (0.2083, [0.0, 0.0, 38.97]),
            (0.25, [0.0, 0.0, 45.0]),
            (0.2917, [0.0, 0.0, 38.97]),
            (0.3333, [0.0, 0.0, 22.5]),
            (0.4167, [0.0, 0.0, -22.5]),
            (0.4583, [0.0, 0.0, -38.97]),
            (0.5, [0.0, 0.0, -45.0]),
            (0.5417, [0.0, 0.0, -38.97]),
            (0.5833, [0.0, 0.0, -22.5]),
            (0.6667, [0.0, 0.0, 22.5]),
            (0.7083, [0.0, 0.0, 38.97]),
            (0.75, [0.0, 0.0, 45.0]),
            (0.7917, [0.0, 0.0, 38.97]),
            (0.8333, [0.0, 0.0, 22.5]),
            (0.9167, [0.0, 0.0, -22.5]),
            (0.9583, [0.0, 0.0, -38.97]),
            (1.0, [0.0, 0.0, -45.0]),
        ] },
    ],
};
pub(super) const BABY_IDLE: Clip = Clip {
    duration: 4.0, loops: true, channels: &[
        Channel { bone: "root", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [1.0, 1.0, 1.0]),
            (0.5417, [0.985, 1.0301, 0.985]),
            (1.0417, [0.98, 1.0399, 0.98]),
            (1.5833, [0.9878, 1.0244, 0.9878]),
            (2.6667, [1.0173, 0.9654, 1.0173]),
            (3.1667, [1.0193, 0.9614, 1.0193]),
            (4.0, [1.0, 1.0, 1.0]),
        ] },
        Channel { bone: "mouthOpen", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "beakOpen", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftWing", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 3.0]),
            (1.0, [0.0, 0.0, -3.0]),
            (2.0, [0.0, 0.0, 3.0]),
            (3.0, [0.0, 0.0, -3.0]),
            (4.0, [0.0, 0.0, 3.0]),
        ] },
        Channel { bone: "rightWing", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -3.0]),
            (1.0, [0.0, 0.0, 3.0]),
            (2.0, [0.0, 0.0, -3.0]),
            (3.0, [0.0, 0.0, 3.0]),
            (4.0, [0.0, 0.0, -3.0]),
        ] },
    ],
};
pub(super) const BABY_WALK: Clip = Clip {
    duration: 0.5833, loops: true, channels: &[
        Channel { bone: "root", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-1.82, 0.27, -1.36]),
            (0.0833, [-0.25, -0.11, 1.05]),
            (0.25, [-2.61, -0.45, 2.57]),
            (0.375, [-0.25, 0.11, -1.05]),
            (0.5417, [-2.61, 0.45, -2.57]),
            (0.5833, [-1.82, 0.27, -1.36]),
        ] },
        Channel { bone: "mouthOpen", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "beakOpen", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftWing", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 2.5]),
            (0.0417, [0.0, 0.0, 2.1]),
            (0.2083, [0.0, 0.0, -4.5]),
            (0.25, [0.0, 0.0, -5.5]),
            (0.2917, [0.0, 0.0, -5.0]),
            (0.4583, [0.0, 0.0, 2.5]),
            (0.5, [0.0, 0.0, 3.5]),
            (0.5833, [0.0, 0.0, 2.5]),
        ] },
        Channel { bone: "rightWing", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -2.5]),
            (0.0417, [0.0, 0.0, -2.1]),
            (0.2083, [0.0, 0.0, 4.5]),
            (0.25, [0.0, 0.0, 5.5]),
            (0.2917, [0.0, 0.0, 5.0]),
            (0.4583, [0.0, 0.0, -2.5]),
            (0.5, [0.0, 0.0, -3.5]),
            (0.5833, [0.0, 0.0, -2.5]),
        ] },
        Channel { bone: "neck", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [1.9, 0.0, 2.0]),
            (0.0833, [2.6, 0.0, -1.14]),
            (0.1667, [4.9, 0.0, -2.49]),
            (0.25, [3.9, 0.0, -3.83]),
            (0.2917, [3.9, 0.0, -2.0]),
            (0.375, [1.9, 0.0, 1.14]),
            (0.4583, [3.9, 0.0, 2.48]),
            (0.5417, [2.9, 0.0, 3.83]),
            (0.5833, [1.9, 0.0, 2.0]),
        ] },
    ],
};
