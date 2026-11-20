"""Compare 144 samples from the real Vulture pose builder with the pinned Java model."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-vulture-pose-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "vulture-pose.sock"

        def ctl(*args):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                check=True, capture_output=True, text=True, timeout=75,
            )
            return json.loads(reply.stdout.splitlines()[-1])

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Vulture Pose Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            result = ctl("command", "naturalist_fixture_vulture_pose")
            assert any("wrote 144 frames" in row for row in result["chat"]), result
            trace = socket.with_suffix(".log")
            if not trace.exists():
                trace = next(directory.glob("*.log"))
            prefix = "naturalist_vulture_pose "
            return [json.loads(row.split(prefix,1)[1]) for row in trace.read_text().splitlines() if prefix in row]
        finally:
            if socket.exists():
                ctl("quit")


def compare(actual, reference):
    assert len(actual) == len(reference) == 144, (len(actual), len(reference))
    errors = []
    worst = 0.0
    previous_worst = 0.0
    previous_mismatched_frames = 0
    for observed, expected in zip(actual, reference):
        tick = expected["tick"], expected["partial"]
        if observed["tick"] != expected["tick"] or abs(observed["partial"] - expected["partial"]) > 1e-6:
            errors.append((tick, "frame", observed["tick"], observed["partial"]))
        assert observed["perched"] == expected["perched"], tick
        if observed["time_ms"] != expected["time_ms"]:
            errors.append((tick, "animation_time", observed["time_ms"], expected["time_ms"]))
        for got, wanted in zip(observed["weights"], expected["weights"]):
            if abs(got - wanted) > 1e-6:
                errors.append((tick, "animation_weight", got, wanted))
        assert observed["bones"].keys() == expected["bones"].keys(), (tick, observed["bones"], expected["bones"])
        previous_frame_worst = 0.0
        for bone, got in observed["bones"].items():
            wanted = expected["bones"][bone]
            error = max(abs(a - b) for a, b in zip(got, wanted))
            worst = max(worst, error)
            previous_error = max(abs(a - b) for a, b in zip(observed["previous"][bone], wanted))
            previous_frame_worst = max(previous_frame_worst, previous_error)
            if error > 2e-5:
                errors.append((tick, bone, got, wanted))
        previous_worst = max(previous_worst, previous_frame_worst)
        previous_mismatched_frames += previous_frame_worst > 2e-5
    print(json.dumps({"frames": len(actual), "bone_pose_max_error": worst,
                      "previous_bone_pose_max_error": previous_worst,
                      "previous_mismatched_frames": previous_mismatched_frames,
                      "mismatches": len(errors), "first_errors": errors[:12]}, indent=2))
    assert not errors, f"{len(errors)} animation comparisons differed from the pinned Java client"


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    parser.add_argument("--actual", type=Path)
    args = parser.parse_args()
    expected = [json.loads(line) for line in args.reference.read_text().splitlines()]
    observed = capture(args.version)
    if args.actual:
        args.actual.write_text("\n".join(map(json.dumps, observed)) + "\n")
    compare(observed, expected)
