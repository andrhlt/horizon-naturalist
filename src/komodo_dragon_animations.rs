// Keyframes translated from Naturalist 2.0.7 Java source (MIT).
use super::{Clip, Channel, Interpolation, Target};
pub(super) const ADULT_IDLE: Clip = Clip {
    duration: 2.0, loops: true, channels: &[
        Channel { bone: "tail", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -3.24, 0.0]),
            (0.3333, [0.0, -3.65, 0.0]),
            (1.0417, [0.0, 3.52, 0.0]),
            (1.375, [0.0, 3.41, 0.0]),
            (2.0, [0.0, -3.24, 0.0]),
        ] },
        Channel { bone: "tail2", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 3.53, 0.0]),
            (0.5, [0.0, -4.85, 0.0]),
            (0.75, [0.0, -5.93, 0.0]),
            (1.0, [0.0, -3.53, 0.0]),
            (1.5, [0.0, 4.85, 0.0]),
            (1.75, [0.0, 5.93, 0.0]),
            (2.0, [0.0, 3.53, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_WALK: Clip = Clip {
    duration: 1.2917, loops: true, channels: &[
        Channel { bone: "tail", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -0.84, -5.0]),
            (0.2083, [0.0, -7.21, -2.62]),
            (0.375, [0.0, -7.47, 1.29]),
            (0.5417, [0.0, -3.04, 4.4]),
            (0.7083, [0.0, 3.3, 4.75]),
            (0.875, [0.0, 7.56, 2.11]),
            (1.0417, [0.0, 7.09, -1.85]),
            (1.2083, [0.0, 2.16, -4.65]),
            (1.2917, [0.0, -1.07, -5.0]),
        ] },
        Channel { bone: "skull", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 1.68, -4.0]),
            (0.25, [0.0, 2.91, -1.37]),
            (0.5417, [0.0, -0.3, 3.52]),
            (0.7083, [0.0, -2.37, 3.8]),
            (0.9167, [0.0, -2.81, 0.92]),
            (1.2083, [0.0, 0.64, -3.72]),
            (1.2917, [0.0, 1.75, -4.0]),
        ] },
        Channel { bone: "tail2", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 7.96, 0.0]),
            (0.125, [0.0, 6.04, -2.87]),
            (0.2917, [0.0, 0.33, -4.95]),
            (0.5, [0.0, -6.63, -3.21]),
            (0.6667, [0.0, -7.81, 0.58]),
            (0.7917, [0.0, -5.39, 3.32]),
            (0.9583, [0.0, 0.6, 5.0]),
            (1.1667, [0.0, 7.11, 2.75]),
            (1.2917, [0.0, 7.93, -0.15]),
        ] },
        Channel { bone: "body", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -0.21, 4.0]),
            (0.2083, [0.0, -1.8, 2.1]),
            (0.5, [0.0, -1.12, -3.06]),
            (0.6667, [0.0, 0.44, -3.97]),
            (1.0, [0.0, 1.92, 0.69]),
            (1.2083, [0.0, 0.54, 3.72]),
            (1.2917, [0.0, -0.27, 4.0]),
        ] },
        Channel { bone: "body", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.91, 0.0]),
            (0.4583, [0.0, 1.24, 0.0]),
            (1.0833, [0.0, 0.75, 0.0]),
            (1.2917, [0.0, 0.91, 0.0]),
        ] },
        Channel { bone: "left_arm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.1667, [22.5, 0.0, 0.0]),
            (0.5, [-45.0, 0.0, 0.0]),
            (0.8333, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "left_arm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 1.0]),
            (0.0833, [0.0, 1.2, 0.7]),
            (0.1667, [0.0, 2.17, 0.44]),
            (0.5, [0.0, 1.0, -1.0]),
            (0.6667, [0.0, 0.5, -2.5]),
            (0.8333, [0.0, 0.0, -2.0]),
            (1.2917, [0.0, 0.0, 1.0]),
        ] },
        Channel { bone: "right_arm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-45.0, 0.0, 0.0]),
            (0.3333, [0.0, 0.0, 0.0]),
            (0.7917, [0.0, 0.0, 0.0]),
            (0.9583, [22.5, 0.0, 0.0]),
            (1.2917, [-45.0, 0.0, 0.0]),
        ] },
        Channel { bone: "right_arm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 1.0, -1.0]),
            (0.1667, [0.0, 0.5, -2.5]),
            (0.3333, [0.0, 0.0, -2.0]),
            (0.7917, [0.0, 0.0, 1.0]),
            (0.875, [0.0, 1.2, 0.7]),
            (0.9583, [0.0, 2.17, 0.44]),
            (1.2917, [0.0, 1.0, -1.0]),
        ] },
        Channel { bone: "left_leg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-45.0, 0.0, 0.0]),
            (0.3333, [0.0, 0.0, 0.0]),
            (0.7917, [0.0, 0.0, 0.0]),
            (1.0, [45.0, 0.0, 0.0]),
            (1.2917, [-45.0, 0.0, 0.0]),
        ] },
        Channel { bone: "left_leg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 1.0, 1.0]),
            (0.1667, [0.0, 0.5, -1.5]),
            (0.3333, [0.0, 0.0, -3.0]),
            (0.7917, [0.0, 0.0, 3.0]),
            (0.8333, [0.0, 0.67, 3.87]),
            (0.875, [0.0, 1.2, 3.7]),
            (1.0, [0.0, 2.17, 0.44]),
            (1.2917, [0.0, 1.0, 1.0]),
        ] },
        Channel { bone: "right_leg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.2083, [45.0, 0.0, 0.0]),
            (0.5, [-45.0, 0.0, 0.0]),
            (0.5417, [-45.0, 0.0, 0.0]),
            (0.875, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "right_leg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 3.0]),
            (0.0417, [0.0, 0.67, 3.87]),
            (0.0833, [0.0, 1.2, 3.7]),
            (0.2083, [0.0, 2.17, 0.44]),
            (0.5, [0.0, 1.0, 1.0]),
            (0.5417, [0.0, 1.0, 1.0]),
            (0.7083, [0.0, 0.5, -1.5]),
            (0.875, [0.0, 0.0, -3.0]),
            (1.2917, [0.0, 0.0, 3.0]),
        ] },
    ],
};
pub(super) const ADULT_BITE: Clip = Clip {
    duration: 0.3125, loops: false, channels: &[
        Channel { bone: "skull", target: Target::Rotation, interpolation: Interpolation::CatmullRom, keys: &[
            (0.125, [-22.5, 0.0, 0.0]),
            (0.1875, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "skull", target: Target::Position, interpolation: Interpolation::CatmullRom, keys: &[
            (0.125, [0.0, 0.0, 0.0]),
            (0.126, [0.0, 0.0, 0.0]),
            (0.1875, [0.0, -1.0, 0.0]),
            (0.3125, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "lower_jaw", target: Target::Rotation, interpolation: Interpolation::CatmullRom, keys: &[
            (0.125, [67.5, 0.0, 0.0]),
            (0.1875, [0.0, 0.0, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_BABY_TRANSFORM: Clip = Clip {
    duration: 0.0, loops: true, channels: &[
        Channel { bone: "skull", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [1.5, 1.5, 1.5]),
        ] },
    ],
};
pub(super) const ADULT_BASK: Clip = Clip {
    duration: 2.0, loops: true, channels: &[
        Channel { bone: "tail", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [7.5, -3.24, 0.0]),
            (0.3333, [7.5, -3.65, 0.0]),
            (1.0417, [7.5, 3.52, 0.0]),
            (1.375, [7.5, 3.41, 0.0]),
            (2.0, [7.5, -3.24, 0.0]),
        ] },
        Channel { bone: "tail2", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 3.53, 0.0]),
            (0.5, [0.0, -4.85, 0.0]),
            (0.75, [0.0, -5.93, 0.0]),
            (1.0, [0.0, -3.53, 0.0]),
            (1.5, [0.0, 4.85, 0.0]),
            (1.75, [0.0, 5.93, 0.0]),
            (2.0, [0.0, 3.53, 0.0]),
        ] },
        Channel { bone: "root", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -3.0, 0.0]),
        ] },
        Channel { bone: "neck", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -2.0, 0.0]),
        ] },
        Channel { bone: "left_arm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-90.0, -47.5, 0.0]),
        ] },
        Channel { bone: "left_arm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [1.0, 0.0, 0.0]),
        ] },
        Channel { bone: "right_arm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-90.0, 47.5, 0.0]),
        ] },
        Channel { bone: "right_arm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-1.0, 0.0, 0.0]),
        ] },
        Channel { bone: "left_leg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-69.059, -40.7895, -30.3612]),
        ] },
        Channel { bone: "left_leg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [1.0, 0.75, 0.0]),
        ] },
        Channel { bone: "right_leg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-69.059, 40.7895, 30.3612]),
        ] },
        Channel { bone: "right_leg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-1.0, 0.75, 0.0]),
        ] },
    ],
};
