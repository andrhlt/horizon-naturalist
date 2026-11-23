"""Check pinned Horizon sources from which current native copies derive."""
import argparse
from hashlib import sha256
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument("horizon_source", type=Path)
args = parser.parse_args()
expected = {
    "crates/horizon-client/src/game/world/persist.rs": "8345f3b650194ec95baefe42f9ca7672d0ea44ce8c48ed687d13751602209294",
    "crates/horizon-client/src/game/mobs/creatures/mod.rs": "50404d84b4bc64fb0def70f7390af73548a9eda692d80ab209467f8f661618fa",
}
for path, want in expected.items():
    got = sha256((args.horizon_source / path).read_bytes()).hexdigest()
    if got != want:
        raise SystemExit(f"{path}: expected {want}, got {got}")
print("Pinned Horizon sources match")
