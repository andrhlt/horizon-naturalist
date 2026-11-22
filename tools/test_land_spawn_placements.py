"""Check the shared land placement against the Ostrich Java substrate fixture."""
import json
from pathlib import Path


ROOT = Path(__file__).parents[1]


def main() -> None:
    fixture = json.loads((ROOT / "fixtures/ostrich-java-substrates.json").read_text())
    placement = fixture["natural_spawn_placement"]
    assert placement["brightness"] == [8, 9, 15]
    assert set(placement["outcomes"]) >= {"minecraft:dirt", "minecraft:grass_block", "minecraft:stone"}
    source = (ROOT / "src/current/land_placements.rs").read_text()
    for species in ("ostrich", "tortoise", "zebra"):
        assert f'"{species}"' in source
    assert 'spawnable_on: Some("minecraft:substrate_overworld".to_owned())' in source
    assert "SpawnLight::Bright" in source
    print("Land placement coverage: Ostrich substrate/light fixture and shared Ostrich/Tortoise/Zebra rule")


if __name__ == "__main__":
    main()
