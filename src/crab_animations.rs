// Keyframes translated from Naturalist 2.0.7 Java source (MIT).
use super::{Clip, Channel, Interpolation, Target};
pub(super) const ADULT_IDLE: Clip = Clip {
    duration: 4.0, loops: true, channels: &[
        Channel { bone: "leftClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -2.35]),
            (0.5833, [0.0, 0.0, 3.73]),
            (0.9167, [0.0, 0.0, 3.11]),
            (1.5417, [0.0, 0.0, -3.52]),
            (1.875, [0.0, 0.0, -3.41]),
            (2.5833, [0.0, 0.0, 3.73]),
            (2.9167, [0.0, 0.0, 3.11]),
            (3.5417, [0.0, 0.0, -3.52]),
            (3.875, [0.0, 0.0, -3.41]),
            (4.0, [0.0, 0.0, -2.35]),
        ] },
        Channel { bone: "leftClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.2, 0.0]),
            (0.5, [0.0, 0.15, 0.0]),
            (1.25, [0.0, -0.25, 0.0]),
            (2.3333, [0.0, 0.23, 0.0]),
            (3.3333, [0.0, -0.23, 0.0]),
            (4.0, [0.0, 0.2, 0.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.4583, [0.0, 0.0, -3.97]),
            (0.75, [0.0, 0.0, -2.83]),
            (1.3333, [0.0, 0.0, 3.46]),
            (1.6667, [0.0, 0.0, 3.46]),
            (2.375, [0.0, 0.0, -3.7]),
            (2.7083, [0.0, 0.0, -3.17]),
            (3.375, [0.0, 0.0, 3.7]),
            (3.7083, [0.0, 0.0, 3.17]),
            (4.0, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0417, [0.0, -0.25, 0.0]),
            (1.0417, [0.0, 0.25, 0.0]),
            (1.9583, [0.0, -0.25, 0.0]),
            (3.0417, [0.0, 0.25, 0.0]),
            (4.0, [0.0, -0.25, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_WALK: Clip = Clip {
    duration: 0.5, loops: true, channels: &[
        Channel { bone: "crabLeftArm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 21.67]),
            (0.0417, [0.0, 0.0, 17.5]),
            (0.1667, [0.0, 0.0, 0.0]),
            (0.4167, [0.0, 0.0, 30.0]),
            (0.5, [0.0, 0.0, 21.67]),
        ] },
        Channel { bone: "crabLeftArm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 1.17, 0.0]),
            (0.0417, [0.0, 1.25, 0.0]),
            (0.1667, [0.0, 0.0, 0.0]),
            (0.4167, [0.0, 1.0, 0.0]),
            (0.5, [0.0, 1.17, 0.0]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 5.0]),
            (0.2083, [0.0, 0.0, 30.0]),
            (0.3333, [0.0, 0.0, 17.5]),
            (0.4583, [0.0, 0.0, 0.0]),
            (0.5, [0.0, 0.0, 5.0]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.16, 0.0]),
            (0.2083, [0.0, 1.0, 0.0]),
            (0.3333, [0.0, 1.25, 0.0]),
            (0.4583, [0.0, 0.0, 0.0]),
            (0.5, [0.0, 0.16, 0.0]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -5.0]),
            (0.0417, [0.0, 0.0, 0.0]),
            (0.1667, [0.0, 0.0, -17.5]),
            (0.2917, [0.0, 0.0, -30.0]),
            (0.5, [0.0, 0.0, -5.0]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.17, 0.0]),
            (0.0417, [0.0, 0.0, 0.0]),
            (0.1667, [0.0, 1.25, 0.0]),
            (0.2917, [0.0, 1.0, 0.0]),
            (0.5, [0.0, 0.17, 0.0]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -25.83]),
            (0.0417, [0.0, 0.0, -30.0]),
            (0.2917, [0.0, 0.0, 0.0]),
            (0.4167, [0.0, 0.0, -17.5]),
            (0.5, [0.0, 0.0, -25.83]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 1.08, 0.0]),
            (0.2917, [0.0, 0.0, 0.0]),
            (0.4167, [0.0, 1.25, 0.0]),
            (0.5, [0.0, 1.08, 0.0]),
        ] },
        Channel { bone: "crabBody", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.1667, [0.0, 0.0, -0.87]),
            (0.4167, [0.0, 0.0, 0.87]),
            (0.5, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "crabBody", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.2, 0.0]),
            (0.25, [0.0, -0.2, 0.0]),
            (0.5, [0.0, 0.2, 0.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.95]),
            (0.25, [0.0, 0.0, -0.95]),
            (0.5, [0.0, 0.0, 0.95]),
        ] },
        Channel { bone: "leftClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.06, 0.0]),
            (0.1667, [0.0, 0.13, 0.0]),
            (0.375, [0.0, -0.19, 0.0]),
            (0.5, [0.0, 0.06, 0.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.59]),
            (0.125, [0.0, 0.0, 0.81]),
            (0.375, [0.0, 0.0, -0.81]),
            (0.5, [0.0, 0.0, 0.59]),
        ] },
        Channel { bone: "rightClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -0.16, 0.0]),
            (0.25, [0.0, 0.16, 0.0]),
            (0.5, [0.0, -0.16, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_HIDE_LOOP: Clip = Clip {
    duration: 0.0, loops: true, channels: &[
        Channel { bone: "crabBody", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -1.0, -1.5]),
        ] },
        Channel { bone: "crabHead", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 3.0]),
        ] },
        Channel { bone: "crabHead", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftEye", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [90.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftEye", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-0.5, 0.25, 2.5]),
        ] },
        Channel { bone: "rightEye", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [90.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightEye", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.5, 0.25, 2.5]),
        ] },
        Channel { bone: "leftClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -45.0, 0.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-3.0, 1.0, 4.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.6]),
        ] },
        Channel { bone: "rightClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 45.0, 0.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [3.0, 1.0, 4.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.6]),
        ] },
        Channel { bone: "legs", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 2.0, 0.0]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-0.1, 0.1, 0.3]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-0.1, 0.1, 0.3]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-0.1, 0.1, 0.3]),
        ] },
        Channel { bone: "crabLeftArm", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-0.1, 0.1, 0.3]),
        ] },
    ],
};
pub(super) const ADULT_SIT_LOOP: Clip = Clip {
    duration: 0.0, loops: true, channels: &[
        Channel { bone: "crabBody", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-90.0, 0.0, 0.0]),
        ] },
        Channel { bone: "crabBody", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 4.0, -1.5]),
        ] },
        Channel { bone: "crabHead", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [90.0, 0.0, 0.0]),
        ] },
        Channel { bone: "crabHead", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -1.0, 0.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [157.5, 0.0, 0.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 1.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [157.5, 0.0, 0.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 1.0]),
        ] },
        Channel { bone: "rightLegs", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -22.5]),
        ] },
        Channel { bone: "rightLegs", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 5.0, -3.0]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [86.7243, 3.5962, -51.1592]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.3266, 0.1353, 0.5]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [91.5423, -24.6992, -20.079]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.3827, 0.9239, 0.0]),
        ] },
        Channel { bone: "leftLegs", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 22.5]),
        ] },
        Channel { bone: "leftLegs", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 5.0, -3.0]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [91.5423, 24.6992, 20.079]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-0.3827, 0.9239, 0.0]),
        ] },
        Channel { bone: "crabLeftArm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [86.7243, -3.5962, 51.1592]),
        ] },
        Channel { bone: "crabLeftArm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-0.3266, 0.1353, 0.5]),
        ] },
    ],
};
pub(super) const ADULT_HIDE: Clip = Clip {
    duration: 1.875, loops: false, channels: &[
        Channel { bone: "crabLeftArm", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.125, [1.0, 1.0, 1.0]),
            (0.2083, [-0.1, 0.1, 0.3]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.125, [1.0, 1.0, 1.0]),
            (0.2083, [-0.1, 0.1, 0.3]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.1667, [1.0, 1.0, 1.0]),
            (0.25, [-0.1, 0.1, 0.3]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.1667, [1.0, 1.0, 1.0]),
            (0.25, [-0.1, 0.1, 0.3]),
        ] },
        Channel { bone: "leftEye", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.125, [0.0, 0.0, 0.0]),
            (0.2083, [90.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftEye", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.1667, [0.0, 0.0, 0.0]),
            (0.25, [-0.5, 0.25, 2.5]),
        ] },
        Channel { bone: "rightEye", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.1667, [0.0, 0.0, 0.0]),
            (0.25, [90.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightEye", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.2083, [0.0, 0.0, 0.0]),
            (0.2917, [0.5, 0.25, 2.5]),
        ] },
        Channel { bone: "crabBody", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0833, [0.0, 0.0, 0.0]),
            (0.1667, [5.0, 0.0, 0.0]),
            (0.2917, [-17.25, -2.99, -9.55]),
            (0.4583, [-17.11, 3.73, 11.94]),
            (0.5833, [0.0, 0.0, 0.0]),
            (0.6667, [7.5, 0.0, 0.0]),
            (0.75, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "crabBody", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0833, [0.0, 0.0, 0.0]),
            (0.2917, [0.0, 3.0, 0.0]),
            (0.4583, [0.0, 4.0, -0.75]),
            (0.5833, [0.0, -1.0, -1.5]),
        ] },
        Channel { bone: "leftClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0833, [0.0, 0.0, 0.0]),
            (0.1667, [45.0, 0.0, 0.0]),
            (0.25, [0.0, -45.0, 0.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.2083, [0.0, 0.0, 0.0]),
            (0.25, [-3.0, 1.0, 4.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.1667, [1.0, 1.0, 1.0]),
            (0.25, [0.0, 0.0, 0.6]),
        ] },
        Channel { bone: "rightClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.125, [0.0, 0.0, 0.0]),
            (0.2083, [45.0, 0.0, 0.0]),
            (0.2917, [0.0, 45.0, 0.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.25, [0.0, 0.0, 0.0]),
            (0.2917, [3.0, 1.0, 4.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.2083, [1.0, 1.0, 1.0]),
            (0.2917, [0.0, 0.0, 0.6]),
        ] },
        Channel { bone: "crabHead", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.2083, [0.0, 0.0, 0.0]),
            (0.2917, [0.0, 0.0, 3.0]),
        ] },
        Channel { bone: "crabHead", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [1.0, 1.0, 1.0]),
            (0.125, [1.0, 1.0, 1.5]),
            (0.1667, [1.0, 1.0, 1.5]),
            (0.2917, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "legs", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0833, [0.0, 0.0, 0.0]),
            (0.25, [0.0, 2.0, 0.0]),
        ] },
        Channel { bone: "legs", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.1667, [1.0, 1.0, 1.0]),
            (0.25, [0.0, 0.0, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_UNHIDE: Clip = Clip {
    duration: 0.5, loops: false, channels: &[
        Channel { bone: "crabLeftArm", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.125, [-0.1, 0.1, 0.3]),
            (0.25, [1.0, 1.0, 1.0]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.125, [-0.1, 0.1, 0.3]),
            (0.25, [1.0, 1.0, 1.0]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0833, [-0.1, 0.1, 0.3]),
            (0.2083, [1.0, 1.0, 1.0]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0833, [-0.1, 0.1, 0.3]),
            (0.2083, [1.0, 1.0, 1.0]),
        ] },
        Channel { bone: "leftEye", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.125, [90.0, 0.0, 0.0]),
            (0.25, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftEye", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0833, [-0.5, 0.25, 2.5]),
            (0.2083, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightEye", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0833, [90.0, 0.0, 0.0]),
            (0.2083, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightEye", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.5, 0.25, 2.5]),
            (0.125, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "crabBody", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.125, [-10.0, 0.0, 0.0]),
            (0.3333, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "crabBody", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -1.0, -1.5]),
            (0.3333, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0833, [0.0, -45.0, 0.0]),
            (0.2083, [15.0, 0.0, 0.0]),
            (0.3333, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0833, [-3.0, 1.0, 4.0]),
            (0.125, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0833, [0.0, 0.0, 0.6]),
            (0.2083, [1.0, 1.0, 1.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 45.0, 0.0]),
            (0.125, [15.0, 0.0, 0.0]),
            (0.25, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [3.0, 1.0, 4.0]),
            (0.0833, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.6]),
            (0.125, [1.0, 1.0, 1.0]),
        ] },
        Channel { bone: "crabHead", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 3.0]),
            (0.125, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "crabHead", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.2083, [1.0, 1.0, 1.5]),
            (0.25, [1.0, 1.0, 1.5]),
            (0.4583, [1.0, 1.0, 1.0]),
        ] },
        Channel { bone: "legs", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 2.0, 0.0]),
            (0.0833, [0.0, 0.0, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_SWING_RIGHT: Clip = Clip {
    duration: 0.625, loops: false, channels: &[
        Channel { bone: "rightClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.0417, [5.0, 0.0, 0.0]),
            (0.0833, [-32.604, 45.3299, -7.8697]),
            (0.125, [-74.195, -4.2218, 9.1227]),
            (0.2083, [0.0, 5.0, 0.0]),
            (0.25, [10.0, 0.0, 0.0]),
            (0.3333, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.0833, [0.0, 0.0, 1.0]),
            (0.2083, [0.0, 0.0, -1.0]),
            (0.3333, [0.0, 0.0, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_DANCE: Clip = Clip {
    duration: 1.0, loops: true, channels: &[
        Channel { bone: "root", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 1.25, 0.0]),
        ] },
        Channel { bone: "crabLeftArm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 42.5]),
            (0.5, [0.0, 0.0, 0.0]),
            (1.0, [0.0, 0.0, 42.5]),
        ] },
        Channel { bone: "crabLeftArm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [1.5, 0.75, 0.0]),
            (0.5, [-1.25, 0.0, 0.0]),
            (1.0, [1.5, 0.75, 0.0]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.5, [0.0, 0.0, -42.5]),
            (1.0, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [1.25, 0.0, 0.0]),
            (0.5, [-1.5, 0.75, 0.0]),
            (1.0, [1.25, 0.0, 0.0]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 42.5]),
            (0.5, [0.0, 0.0, 0.0]),
            (1.0, [0.0, 0.0, 42.5]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [1.5, 0.75, 0.0]),
            (0.5, [-1.25, 0.0, 0.0]),
            (1.0, [1.5, 0.75, 0.0]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.5, [0.0, 0.0, -42.5]),
            (1.0, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [1.25, 0.0, 0.0]),
            (0.5, [-1.5, 0.75, 0.0]),
            (1.0, [1.25, 0.0, 0.0]),
        ] },
        Channel { bone: "crabBody", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 5.0, -10.0]),
            (0.0833, [0.0, 4.33, -8.66]),
            (0.2083, [0.0, 1.29, -2.59]),
            (0.375, [0.0, -3.54, 7.07]),
            (0.4583, [0.0, -4.83, 9.66]),
            (0.5417, [0.0, -4.83, 9.66]),
            (0.625, [0.0, -3.54, 7.07]),
            (0.875, [0.0, 3.54, -7.07]),
            (0.9583, [0.0, 4.83, -9.66]),
            (1.0, [0.0, 5.0, -10.0]),
        ] },
        Channel { bone: "crabBody", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [2.0, -0.69, 0.0]),
            (0.0833, [1.73, -0.54, 0.0]),
            (0.25, [0.0, -0.31, 0.0]),
            (0.375, [-1.41, -0.56, 0.0]),
            (0.4583, [-1.93, -0.7, 0.0]),
            (0.5417, [-1.93, -0.63, 0.0]),
            (0.625, [-1.41, -0.44, 0.0]),
            (0.7917, [0.52, -0.37, 0.0]),
            (0.9167, [1.73, -0.65, 0.0]),
            (1.0, [2.0, -0.69, 0.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [1.55, 0.0, 0.0]),
            (0.0417, [3.72, 0.0, 0.0]),
            (0.0833, [4.89, 0.0, 0.0]),
            (0.125, [4.76, 0.0, 0.0]),
            (0.2917, [-3.72, 0.0, 0.0]),
            (0.3333, [-4.89, 0.0, 0.0]),
            (0.375, [-4.76, 0.0, 0.0]),
            (0.5417, [3.72, 0.0, 0.0]),
            (0.5833, [4.89, 0.0, 0.0]),
            (0.625, [4.76, 0.0, 0.0]),
            (0.7917, [-3.72, 0.0, 0.0]),
            (0.8333, [-4.89, 0.0, 0.0]),
            (0.875, [-4.76, 0.0, 0.0]),
            (1.0, [1.55, 0.0, 0.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.78, 0.0]),
            (0.0833, [0.0, 0.41, 0.0]),
            (0.1667, [0.0, 0.63, 0.0]),
            (0.2917, [0.0, 1.47, 0.0]),
            (0.375, [0.0, 1.56, 0.0]),
            (0.5417, [0.0, 0.53, 0.0]),
            (0.625, [0.0, 0.44, 0.0]),
            (0.7917, [0.0, 1.47, 0.0]),
            (0.875, [0.0, 1.56, 0.0]),
            (1.0, [0.0, 0.78, 0.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [5.0, 0.0, 0.0]),
            (0.0417, [4.33, 0.0, 0.0]),
            (0.2083, [-4.33, 0.0, 0.0]),
            (0.25, [-5.0, 0.0, 0.0]),
            (0.2917, [-4.33, 0.0, 0.0]),
            (0.4583, [4.33, 0.0, 0.0]),
            (0.5, [5.0, 0.0, 0.0]),
            (0.5417, [4.33, 0.0, 0.0]),
            (0.7083, [-4.33, 0.0, 0.0]),
            (0.75, [-5.0, 0.0, 0.0]),
            (0.7917, [-4.33, 0.0, 0.0]),
            (0.9583, [4.33, 0.0, 0.0]),
            (1.0, [5.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.24, 0.0]),
            (0.1667, [0.0, 1.59, 0.0]),
            (0.2083, [0.0, 1.78, 0.0]),
            (0.2917, [0.0, 1.54, 0.0]),
            (0.4167, [0.0, 0.41, 0.0]),
            (0.4583, [0.0, 0.22, 0.0]),
            (0.5, [0.0, 0.24, 0.0]),
            (0.6667, [0.0, 1.59, 0.0]),
            (0.7083, [0.0, 1.78, 0.0]),
            (0.7917, [0.0, 1.54, 0.0]),
            (0.9167, [0.0, 0.41, 0.0]),
            (0.9583, [0.0, 0.22, 0.0]),
            (1.0, [0.0, 0.24, 0.0]),
        ] },
        Channel { bone: "shell", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 1.55, 5.88]),
            (0.0833, [0.0, 3.72, 9.14]),
            (0.1667, [0.0, 4.89, 9.95]),
            (0.25, [0.0, 4.76, 8.09]),
            (0.375, [0.0, 2.27, 1.56]),
            (0.5417, [0.0, -2.72, -7.77]),
            (0.625, [0.0, -4.46, -9.88]),
            (0.7083, [0.0, -4.99, -9.34]),
            (0.8333, [0.0, -3.35, -4.07]),
            (1.0, [0.0, 1.55, 5.88]),
        ] },
        Channel { bone: "leftLegs", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.1, 0.0]),
            (0.5, [-1.0, -1.0, 0.0]),
            (1.0, [0.0, 0.1, 0.0]),
        ] },
        Channel { bone: "rightLegs", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [1.0, -1.0, 0.0]),
            (0.5, [0.0, 0.1, 0.0]),
            (1.0, [1.0, -1.0, 0.0]),
        ] },
    ],
};
pub(super) const ADULT_HIDE_PEEK: Clip = Clip {
    duration: 1.1667, loops: false, channels: &[
        Channel { bone: "crabBody", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -1.0, -1.5]),
        ] },
        Channel { bone: "crabHead", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 3.0]),
        ] },
        Channel { bone: "crabHead", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.2083, [1.0, 1.0, 1.0]),
            (0.9583, [1.0, 1.0, 1.0]),
            (1.1667, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftEye", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [90.0, 0.0, 0.0]),
            (0.25, [0.0, 0.0, 0.0]),
            (0.875, [0.0, 0.0, 0.0]),
            (1.1667, [90.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftEye", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-0.5, -1.75, 2.5]),
            (0.25, [0.0, -1.75, -1.25]),
            (0.875, [0.0, -1.75, -1.25]),
            (1.1667, [-0.5, -1.75, 2.5]),
        ] },
        Channel { bone: "rightEye", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0833, [90.0, 0.0, 0.0]),
            (0.3333, [0.0, 0.0, 0.0]),
            (0.9167, [0.0, 0.0, 0.0]),
            (1.1667, [90.0, 0.0, 0.0]),
        ] },
        Channel { bone: "rightEye", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0833, [0.5, -1.75, 2.5]),
            (0.3333, [0.0, -1.75, -1.25]),
            (0.9167, [0.0, -1.75, -1.25]),
            (1.1667, [0.5, -1.75, 2.5]),
        ] },
        Channel { bone: "leftClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -45.0, 0.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-3.0, 1.0, 4.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.6]),
        ] },
        Channel { bone: "rightClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 45.0, 0.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [3.0, 1.0, 4.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.6]),
        ] },
        Channel { bone: "legs", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 2.0, 0.0]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-0.1, 0.1, 0.3]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-0.1, 0.1, 0.3]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-0.1, 0.1, 0.3]),
        ] },
        Channel { bone: "crabLeftArm", target: Target::Scale, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-0.1, 0.1, 0.3]),
        ] },
    ],
};
pub(super) const BABY_IDLE: Clip = Clip {
    duration: 4.0, loops: true, channels: &[
        Channel { bone: "crabBody", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.68, 0.0, 0.0]),
            (1.2917, [-1.99, 0.0, 0.0]),
            (2.9583, [1.83, 0.0, 0.0]),
            (4.0, [0.68, 0.0, 0.0]),
        ] },
        Channel { bone: "crabBody", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.1667, [0.0, 0.0, 0.0]),
            (2.4583, [0.0, -0.18, 0.0]),
            (4.0, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -3.46]),
            (0.5833, [-5.55, 0.0, -0.52]),
            (1.0833, [-6.94, 0.0, 2.44]),
            (1.5833, [-4.26, 0.0, 3.97]),
            (2.3333, [3.5, 0.0, 2.0]),
            (2.8333, [6.76, 0.0, -1.04]),
            (3.2917, [6.28, 0.0, -3.33]),
            (4.0, [0.0, 0.0, -3.46]),
        ] },
        Channel { bone: "leftClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.125, [0.0, 0.2, 0.0]),
            (2.125, [0.0, -0.2, 0.0]),
            (4.0, [0.0, 0.2, 0.0]),
        ] },
        Channel { bone: "rightClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [4.5, 0.0, 3.46]),
            (1.0833, [-5.9, 0.0, -2.44]),
            (1.5833, [-6.83, 0.0, -3.97]),
            (2.125, [-3.37, 0.0, -3.01]),
            (3.0, [5.36, 0.0, 2.0]),
            (3.5, [6.97, 0.0, 3.86]),
            (4.0, [4.5, 0.0, 3.46]),
        ] },
        Channel { bone: "rightClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.15, 0.0]),
            (1.125, [0.0, 0.1, 0.0]),
            (2.5833, [0.0, -0.2, 0.0]),
            (4.0, [0.0, 0.15, 0.0]),
        ] },
        Channel { bone: "crabLeftArm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 1.93]),
            (1.2083, [0.0, 0.0, -2.8]),
            (1.8333, [0.0, 0.0, -2.46]),
            (3.25, [0.0, 0.0, 2.86]),
            (4.0, [0.0, 0.0, 1.93]),
        ] },
        Channel { bone: "crabLeftArm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0417, [0.0, 0.06, 0.0]),
            (2.125, [0.0, -0.05, 0.0]),
            (4.0, [0.0, 0.06, 0.0]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -1.93]),
            (1.2083, [0.0, 0.0, 2.8]),
            (1.8333, [0.0, 0.0, 2.46]),
            (3.25, [0.0, 0.0, -2.86]),
            (4.0, [0.0, 0.0, -1.93]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0417, [0.0, 0.06, 0.0]),
            (2.125, [0.0, -0.05, 0.0]),
            (4.0, [0.0, 0.06, 0.0]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 1.93]),
            (1.2083, [0.0, 0.0, -2.8]),
            (1.8333, [0.0, 0.0, -2.46]),
            (3.25, [0.0, 0.0, 2.86]),
            (4.0, [0.0, 0.0, 1.93]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0417, [0.0, 0.06, 0.0]),
            (2.125, [0.0, -0.05, 0.0]),
            (4.0, [0.0, 0.06, 0.0]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -1.93]),
            (1.2083, [0.0, 0.0, 2.8]),
            (1.8333, [0.0, 0.0, 2.46]),
            (3.25, [0.0, 0.0, -2.86]),
            (4.0, [0.0, 0.0, -1.93]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0417, [0.0, 0.06, 0.0]),
            (2.125, [0.0, -0.05, 0.0]),
            (4.0, [0.0, 0.06, 0.0]),
        ] },
    ],
};
pub(super) const BABY_WALK: Clip = Clip {
    duration: 1.0, loops: true, channels: &[
        Channel { bone: "crabLeftArm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 15.86]),
            (0.125, [0.0, 0.0, 1.8]),
            (0.25, [0.0, 0.0, -13.31]),
            (0.375, [0.0, 0.0, -20.62]),
            (0.5, [0.0, 0.0, -15.86]),
            (0.625, [0.0, 0.0, -1.8]),
            (0.75, [0.0, 0.0, 13.31]),
            (0.875, [0.0, 0.0, 20.62]),
            (1.0, [0.0, 0.0, 15.86]),
        ] },
        Channel { bone: "crabLeftArm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.89, 0.0]),
            (0.25, [0.0, 0.79, 0.0]),
            (0.5, [0.0, 0.36, 0.0]),
            (0.75, [0.0, 0.34, 0.0]),
            (1.0, [0.0, 0.89, 0.0]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -15.86]),
            (0.125, [0.0, 0.0, -1.8]),
            (0.25, [0.0, 0.0, 13.31]),
            (0.375, [0.0, 0.0, 20.62]),
            (0.5, [0.0, 0.0, 15.86]),
            (0.625, [0.0, 0.0, 1.8]),
            (0.75, [0.0, 0.0, -13.31]),
            (0.875, [0.0, 0.0, -20.62]),
            (1.0, [0.0, 0.0, -15.86]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.36, 0.0]),
            (0.25, [0.0, 0.34, 0.0]),
            (0.5, [0.0, 0.89, 0.0]),
            (0.75, [0.0, 0.79, 0.0]),
            (1.0, [0.0, 0.36, 0.0]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 4.5]),
            (0.125, [0.0, 0.0, -0.48]),
            (0.375, [0.0, 0.0, -24.52]),
            (0.5, [0.0, 0.0, -29.5]),
            (0.625, [0.0, 0.0, -24.52]),
            (0.875, [0.0, 0.0, -0.48]),
            (1.0, [0.0, 0.0, 4.5]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.25, 0.0]),
            (0.25, [0.0, 1.04, 0.0]),
            (0.5, [0.0, 0.84, 0.0]),
            (0.75, [0.0, 0.21, 0.0]),
            (1.0, [0.0, 0.25, 0.0]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -29.5]),
            (0.125, [0.0, 0.0, -24.52]),
            (0.375, [0.0, 0.0, -0.48]),
            (0.5, [0.0, 0.0, 4.5]),
            (0.625, [0.0, 0.0, -0.48]),
            (0.875, [0.0, 0.0, -24.52]),
            (1.0, [0.0, 0.0, -29.5]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.84, 0.0]),
            (0.25, [0.0, 0.21, 0.0]),
            (0.5, [0.0, 0.25, 0.0]),
            (0.75, [0.0, 1.04, 0.0]),
            (1.0, [0.0, 0.84, 0.0]),
        ] },
        Channel { bone: "crabBody", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.0]),
            (0.375, [0.0, 0.0, -0.71]),
            (0.8333, [0.0, 0.0, 0.87]),
            (1.0, [0.0, 0.0, 0.0]),
        ] },
        Channel { bone: "crabBody", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.15, 0.0]),
            (0.2917, [0.0, -0.04, 0.0]),
            (0.5417, [0.0, 0.14, 0.0]),
            (0.7917, [0.0, -0.04, 0.0]),
            (1.0, [0.0, 0.15, 0.0]),
        ] },
        Channel { bone: "shell", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 0.49]),
            (0.5417, [0.0, 0.0, -0.45]),
            (1.0, [0.0, 0.0, 0.49]),
        ] },
    ],
};
pub(super) const BABY_SIT_IDLE: Clip = Clip {
    duration: 0.0, loops: true, channels: &[
        Channel { bone: "crabBody", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [-18.0, 0.0, 0.0]),
        ] },
        Channel { bone: "crabBody", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -0.2, 0.0]),
        ] },
        Channel { bone: "leftClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [36.9495, 12.0866, 8.9504]),
        ] },
        Channel { bone: "leftClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, -0.2, 0.6]),
        ] },
        Channel { bone: "rightClaw", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [36.9495, -12.0866, -8.9504]),
        ] },
        Channel { bone: "rightClaw", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.3, -0.2, 0.6]),
        ] },
        Channel { bone: "leftEye", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [15.0, 0.0, 0.0]),
        ] },
        Channel { bone: "crabLeftArm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 21.0]),
        ] },
        Channel { bone: "crabLeftArm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.6, 0.0]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -21.0]),
        ] },
        Channel { bone: "crabRightArm", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.6, 0.0]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, 22.0]),
        ] },
        Channel { bone: "crabLeftLeg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.4, -0.3]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Rotation, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.0, -22.0]),
        ] },
        Channel { bone: "crabRightLeg", target: Target::Position, interpolation: Interpolation::Linear, keys: &[
            (0.0, [0.0, 0.4, -0.3]),
        ] },
    ],
};
