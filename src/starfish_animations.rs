// Keyframes translated from Naturalist 2.0.7 Java source (MIT).
use super::{Clip, Channel, Interpolation, Target};
pub(super) const ADULT_IDLE: Clip = Clip {
    duration: 3.0, loops: true, channels: &[
        Channel { bone: "leftArm", target: Target::Rotation, interpolation: Interpolation::CatmullRom, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (1.5, [0.0, 0.0, 12.5]),
            (3.0, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightArm", target: Target::Rotation, interpolation: Interpolation::CatmullRom, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (1.5, [0.0, 0.0, -12.5]),
            (3.0, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftLeg", target: Target::Rotation, interpolation: Interpolation::CatmullRom, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (1.5, [-4.8413, -1.508, 2.6349]),
            (3.0, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightLeg", target: Target::Rotation, interpolation: Interpolation::CatmullRom, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (1.5, [-4.8413, 1.508, -2.6349]),
            (3.0, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "skull", target: Target::Rotation, interpolation: Interpolation::CatmullRom, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (1.5, [7.5, 0.0, 0.0]),
            (3.0, [0.0, 0.0, 0.0]),
        ] },
    ],
};
