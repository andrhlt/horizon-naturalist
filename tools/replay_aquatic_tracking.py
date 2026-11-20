"""Run headless Horizon entity tracking and compare it with the Java fixture."""
import json
from pathlib import Path
import subprocess
import sys

root = Path(__file__).resolve().parents[1]
version = Path(sys.argv[1]).resolve()
fixture = root / "fixtures" / "aquatic-tracking-java-extended.json"
tracking = subprocess.run(
    [sys.executable, str(root / "tools" / "launcher_aquatic_tracking_range.py"), str(version)],
    check=True, capture_output=True, text=True, timeout=600,
)
observed = json.loads(tracking.stdout)
expected = json.loads(fixture.read_text())["observed"]
assert observed == expected, {"expected": expected, "observed": observed}

packets = subprocess.run(
    [sys.executable, str(root / "tools" / "launcher_beached_packets.py"), str(version)],
    check=True, capture_output=True, text=True, timeout=600,
)
motion = {}
for line in packets.stdout.splitlines():
    species, encoded = line.split(" ", 1)
    rows = json.loads(encoded)
    first = next(row for row in rows if any(packet[1] > .4 for packet in row["received"]))
    guest = next(packet for packet in first["received"] if packet[1] > .4)
    recorded = first["motion"][0].split("[", 1)[1].split("]", 1)[0]
    host = [float(value.removesuffix("d")) for value in recorded.split(", ")]
    assert first["tick"] == 2 and all(abs(a - b) <= 1 / 32766 for a, b in zip(host, guest, strict=True)), (species, first)
    motion[species] = {"tick": first["tick"], "velocity": guest}
assert set(motion) == {"whale", "great_white_shark"}, motion
samples = sum(len(group) for groups in observed.values() for group in groups)
print(json.dumps({"matches": samples, "pairings": observed, "flop_motion_packets": motion}, indent=2))
