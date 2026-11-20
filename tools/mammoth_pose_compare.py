"""Build the current Rust model and animation evaluator and compare the calf's idle poses."""

import argparse
import json
import re
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory


def top_level(source, declaration):
    return re.search(rf"^{re.escape(declaration)}.*?^}}", source, re.M | re.S).group()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("rustc", type=Path)
    parser.add_argument("client_library", type=Path, help="horizon_client_lib rlib from the tagged launcher build")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    src = root / "src"
    animations = (src / "deer.rs").read_text()
    declarations = animations[
        animations.index("#[derive(Clone, Copy)]\npub(super) enum Target"):
        animations.index("struct DeerPose")
    ]
    renderer = (src / "elephant_render.rs").read_text()
    evaluator = "\n".join([
        "mod probe {",
        "use horizon_client::mobs::model_layers::ModelPart;",
        declarations,
        top_level(animations, "pub(super) fn bone"),
        top_level(renderer, "fn pachyderm_clips"),
        f"#[path = {json.dumps(str(src / 'elephant_animations.rs'))}] mod elephant_animations;",
        f"#[path = {json.dumps(str(src / 'mammoth_models.rs'))}] mod mammoth_models;",
        r'''
    pub fn sample(baby_clip: bool) {
        let mut clock = AnimationClock::default();
        let mut model = mammoth_models::baby();
        let clip = pachyderm_clips(baby_clip)[0];
        for tick in 1..=120 {
            clock.advance_with_rate(true, tick, 0.5);
            model.reset_all_poses();
            clock.draw(clip, &mut model, tick as f32 + 0.5, 0.5, 1.0, 1.0);
            if tick % 10 == 0 {
                let body_z = bone(&mut model, "body").unwrap().z_rot;
                let trunk_x = bone(&mut model, "trunk").unwrap().x_rot;
                let ear_z = bone(&mut model, "leftEar").unwrap().z_rot;
                println!("{{\"tick\":{tick},\"baby_clip\":{baby_clip},\"body_z\":{body_z:.9},\"trunk_x\":{trunk_x:.9},\"ear_z\":{ear_z:.9}}}");
            }
        }
    }
}
fn main() {
    probe::sample(true);
    probe::sample(false);
}
''',
    ])
    with TemporaryDirectory() as tmp:
        source, runner = (Path(tmp) / name for name in ("probe.rs", "probe"))
        source.write_text(evaluator)
        subprocess.run([
            args.rustc, "--edition=2024", "-A", "dead_code,unused_variables,non_snake_case", "--extern",
            f"horizon_client={args.client_library}", "-L",
            f"dependency={args.client_library.parent}", source, "-o", runner,
        ], check=True)
        actual = subprocess.check_output([runner], text=True)
    expected = (root / "fixtures/mammoth-pose-java.ndjson").read_text()
    deltas = []
    clip_deltas = []
    corrected = {}
    for line, control in zip(expected.splitlines(), actual.splitlines(), strict=True):
        java, rust = json.loads(line), json.loads(control)
        assert (java["tick"], java["baby_clip"]) == (rust["tick"], rust["baby_clip"])
        axes = ("body_z", "trunk_x", "ear_z")
        deltas.extend(abs(java[axis] - rust[axis]) for axis in axes)
        angles = [rust[axis] for axis in axes]
        if rust["baby_clip"]:
            corrected[rust["tick"]] = angles
        else:
            clip_deltas.extend(abs(a - b) for a, b in zip(corrected[rust["tick"]], angles, strict=True))
    assert len(deltas) == 72 and max(deltas) < 2e-7
    assert max(clip_deltas) > 0.05
    print(f"Calf and adult clips on the calf model: 24 poses, 72 angles, maximum Java delta {max(deltas):.10g} radians")
    print(f"Former adult clip changed a sampled calf angle by up to {max(clip_deltas):.10g} radians")


if __name__ == "__main__":
    main()
