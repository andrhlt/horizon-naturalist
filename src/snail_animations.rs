// Keyframes translated from Naturalist 2.0.7 Java source (MIT).
use super::{Clip, Channel, Interpolation, Target};
pub(super) const ADULT_IDLE: Clip = Clip {
    duration: 2.0, loops: true, channels: &[
        Channel { bone: "shell", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -1.0]),
            (0.6667, [-0.87, 0.0, 0.5]),
            (1.3333, [0.87, 0.0, 0.5]),
            (2.0, [0.0, 0.0, -1.0]),
        ] },
        Channel { bone: "leftEye", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -1.0]),
            (0.5, [-2.0, 0.0, 0.0]),
            (1.1667, [1.0, 0.0, 0.87]),
            (1.625, [1.85, 0.0, -0.38]),
            (2.0, [0.0, 0.0, -1.0]),
        ] },
        Channel { bone: "rightEye", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 1.62]),
            (0.3333, [-1.73, 0.0, 1.83]),
            (0.7917, [-1.22, 0.0, -0.57]),
            (1.25, [1.41, 0.0, -1.98]),
            (1.625, [1.85, 0.0, -0.47]),
            (2.0, [0.0, 0.0, 1.62]),
        ] },
    ],
};
pub(super) const ADULT_CRAWL: Clip = Clip {
    duration: 2.0, loops: true, channels: &[
        Channel { bone: "front", target: Target::Rotation, interpolation: Interpolation::CatmullRom, keys: &[
            (0.0, [0.0, -1.0, 0.0]),
            (1.0833, [0.0, 0.97, 0.0]),
            (2.0, [0.0, -1.0, 0.0]),
        ] },
        Channel { bone: "tail", target: Target::Rotation, interpolation: Interpolation::CatmullRom, keys: &[
            (0.0, [0.0, -3.54, 0.0]),
            (0.6667, [0.0, 4.83, 0.0]),
            (1.0, [0.0, 3.54, 0.0]),
            (1.6667, [0.0, -4.83, 0.0]),
            (2.0, [0.0, -3.54, 0.0]),
        ] },
        Channel { bone: "shell", target: Target::Rotation, interpolation: Interpolation::CatmullRom, keys: &[
            (0.0, [-1.0, 0.0, 0.0]),
            (0.625, [-2.9, 0.0, 0.0]),
            (1.4167, [0.96, 0.0, 0.0]),
            (1.875, [-0.07, 0.0, 0.0]),
            (2.0, [-1.0, 0.0, 0.0]),
        ] },
        Channel { bone: "shell", target: Target::Position, interpolation: Interpolation::CatmullRom, keys: &[
            (0.0, [0.0, 0.14, 0.0]),
            (0.9167, [0.0, -0.17, 0.0]),
            (1.75, [0.0, 0.2, 0.0]),
            (2.0, [0.0, 0.14, 0.0]),
        ] },
        Channel { bone: "body", target: Target::Scale, interpolation: Interpolation::CatmullRom, keys: &[
            (0.0, [1.0, 1.0, 1.0]),
            (0.2083, [1.0, 1.0, 1.0696]),
            (0.4167, [1.0, 1.0, 1.0978]),
            (0.7917, [1.0, 1.0, 1.0696]),
            (1.125, [1.0, 1.0, 0.9535]),
            (1.2917, [1.0, 1.0, 0.9146]),
            (1.5, [1.0, 1.0, 0.9]),
            (1.7083, [1.0, 1.0, 0.9146]),
            (1.875, [1.0, 1.0, 0.9535]),
            (2.0, [1.0, 1.0, 1.0]),
        ] },
        Channel { bone: "rightEye", target: Target::Rotation, interpolation: Interpolation::CatmullRom, keys: &[
            (0.0, [13.66, 0.0, 0.0]),
            (0.5833, [-3.41, 0.0, 0.0]),
            (1.0417, [-2.99, 0.0, 0.0]),
            (1.5833, [13.41, 0.0, 0.0]),
            (1.8333, [14.97, 0.0, 0.0]),
            (2.0, [13.66, 0.0, 0.0]),
        ] },
        Channel { bone: "leftEye", target: Target::Rotation, interpolation: Interpolation::CatmullRom, keys: &[
            (0.0, [15.0, 0.0, 0.0]),
            (0.2083, [13.54, 0.0, 0.0]),
            (0.7917, [-3.54, 0.0, 0.0]),
            (1.1667, [-4.09, 0.0, 0.0]),
            (1.3333, [-0.9, 0.0, 0.0]),
            (1.7917, [13.54, 0.0, 0.0]),
            (2.0, [15.0, 0.0, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_HIDE_START: Clip = Clip {
    duration: 1.5, loops: false, channels: &[
        Channel { bone: "shell", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.0833, [-3.0625, -0.3287, -1.9716]),
            (0.125, [-3.0, 0.0, 0.0]),
            (0.25, [19.0, 0.0, 0.0]),
            (0.375, [-10.0, 0.0, 0.0]),
            (0.5, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "shell", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.125, [0.0, 0.2, 0.2]),
            (0.25, [0.0, 0.0, -1.0]),
            (0.375, [0.0, -1.5, 0.0]),
            (0.5, [0.0, -2.0, 0.0]),
        ] },
        Channel { bone: "body", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.125, [0.0, 0.0, 0.0]),
            (0.2083, [1.0, 0.0, 0.0]),
        ] },
        Channel { bone: "body", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0417, [0.0, 0.0, 0.0]),
            (0.125, [0.0, 0.0, -0.4]),
            (0.2083, [0.0, 0.0, 0.5]),
            (0.7917, [0.0, 0.0, 2.5]),
        ] },
        Channel { bone: "body", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0417, [1.0, 1.0, 1.0]),
            (0.125, [1.0, 1.0, 1.11]),
            (0.2083, [1.0, 1.0, 0.54]),
            (0.375, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "eyes", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.125, [0.0, 0.0, 0.0]),
            (0.2083, [88.0, 0.0, 0.0]),
        ] },
        Channel { bone: "eyes", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.125, [0.0, 0.0, 0.0]),
            (0.2083, [0.0, -2.0, 0.0]),
            (0.7917, [0.0, 0.0, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_HIDE_END: Clip = Clip {
    duration: 0.7917, loops: false, channels: &[
        Channel { bone: "shell", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.2917, [0.0, 0.0, 0.0]),
            (0.4167, [-10.0, 0.0, 0.0]),
            (0.5417, [19.0, 0.0, 0.0]),
            (0.6667, [-3.0, 0.0, 0.0]),
            (0.7083, [-3.0625, -0.3287, -1.9716]),
            (0.7917, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "shell", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.2917, [0.0, -2.0, 0.0]),
            (0.4167, [0.0, -1.5, 0.0]),
            (0.5417, [0.0, 0.0, -1.0]),
            (0.6667, [0.0, 0.2, 0.2]),
            (0.7917, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "body", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.5833, [1.0, 0.0, 0.0]),
            (0.6667, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "body", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 2.5]),
            (0.5833, [0.0, 0.0, 0.5]),
            (0.6667, [0.0, 0.0, -0.4]),
            (0.75, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "body", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.4167, [0.0, 0.0, 0.0]),
            (0.5833, [1.0, 1.0, 0.54]),
            (0.6667, [1.0, 1.0, 1.11]),
            (0.75, [1.0, 1.0, 1.0]),
        ] },
        Channel { bone: "eyes", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.5833, [88.0, 0.0, 0.0]),
            (0.6667, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "eyes", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.5833, [0.0, -2.0, 0.0]),
            (0.6667, [0.0, 0.0, 0.0]),
        ] },
    ],
};
