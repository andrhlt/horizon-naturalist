"""Check the native aquatic EntityPart limits against the pinned Java fixture."""
import json
from pathlib import Path


ROOT = Path(__file__).parents[1]
EXPECTED = {
    "naturalist:anglerfish": 8,
    "naturalist:bass": 6,
    "naturalist:blobfish": 8,
    "naturalist:catfish": 8,
    "naturalist:jellyfish": 8,
    "naturalist:piranha": 24,
    "naturalist:ray": 8,
}


def main() -> None:
    fixture = json.loads((ROOT / "fixtures/aquatic-spawn-java.json").read_text())
    assert fixture["cluster_limits"] == EXPECTED
    source = (ROOT / "src/current/lib.rs").read_text()
    for kind, limit in EXPECTED.items():
        if kind == "naturalist:bass":
            assert "then_some(6)" in (ROOT / "src/current/bass_hunt.rs").read_text()
        else:
            assert f'("{kind}", {limit})' in source
    print(f"Aquatic cluster limits: {len(EXPECTED)} species, Java-backed")


if __name__ == "__main__":
    main()
