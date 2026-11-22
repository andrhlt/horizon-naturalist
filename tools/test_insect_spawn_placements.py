"""Check fixture-backed insect placement declarations and light modes."""
import json
from pathlib import Path


ROOT = Path(__file__).parents[1]


def main() -> None:
    fixture = json.loads((ROOT / "fixtures/insect-spawn-java.json").read_text())
    assert fixture["open_mud"]["butterfly"] == 64
    assert fixture["open_mud"]["dragonfly"] == 64
    assert fixture["dark_grass"]["firefly"] == 64
    source = (ROOT / "src/current/land_placements.rs").read_text()
    assert 'naturalist:butterflies_spawnable_on' in source
    assert 'naturalist:dragonflies_spawnable_on' in source
    assert 'naturalist:fireflies_spawnable_on' in source
    assert "SpawnLight::Bright" in source and "SpawnLight::Dark" in source
    print("Insect placement coverage: mud/grass substrates and bright/dark light modes")


if __name__ == "__main__":
    main()
