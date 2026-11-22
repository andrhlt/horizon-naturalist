"""Validate the source-side Naturalist spawning contract.

This is intentionally independent of the Horizon SDK.  It catches the
regressions that are otherwise easy to miss when the native launcher is not
available: every Java profile/variant owner must have a Horizon entity, every
ordered biome modifier must be present exactly once, and every species with a
recorded predicate fixture must remain represented by a spawn row.
"""
import argparse
import json
from pathlib import Path


def metadata_species(root: Path) -> set[str]:
    data = json.loads((root / "migration/entity-metadata.json").read_text())
    return {
        key.removeprefix("naturalist:")
        for key in data
        if key != "naturalist:dirt_trail"
    }


def variant_species(root: Path) -> set[str]:
    data = json.loads((root / "fixtures/variant-order-java.json").read_text())
    return {key.removeprefix("naturalist:") for key in data}


def entity_species(root: Path) -> set[str]:
    return {
        path.stem
        for path in (root / "data/naturalist/horizon/entity_type").glob("*.json")
        if path.stem != "dirt_trail"
    }


def modifier_species(root: Path) -> list[str]:
    result = []
    for path in sorted((root / "data/naturalist/neoforge/biome_modifier").glob("*.json")):
        value = json.loads(path.read_text())
        if value.get("type") != "neoforge:add_spawns":
            continue
        species = value.get("spawners", {}).get("type")
        if species and species.startswith("naturalist:"):
            result.append(species.removeprefix("naturalist:"))
    return result


def fixture_species(root: Path) -> set[str]:
    result = set()
    for path in (root / "fixtures").glob("*-java.json"):
        try:
            value = json.loads(path.read_text())
        except json.JSONDecodeError:
            continue
        text = json.dumps(value)
        for species in entity_species(root):
            if f"naturalist:{species}" in text:
                result.add(species)
    return result


def verify(root: Path) -> dict:
    metadata = metadata_species(root)
    variants = variant_species(root)
    entities = entity_species(root)
    modifiers = modifier_species(root)
    fixture = fixture_species(root)
    errors = []
    if missing := sorted((metadata | variants) - entities):
        errors.append(f"missing entity definitions: {missing}")
    # Bird and Snake intentionally have one modifier per variant/biome pool.
    duplicates = sorted(
        name for name in set(modifiers)
        if modifiers.count(name) > 1 and name not in {"bird", "snake"}
    )
    if duplicates:
        errors.append(f"duplicate spawn modifiers: {duplicates}")
    non_natural_spawn = {"ant", "carried_food", "caterpillar", "duck_egg", "lizard_tail"}
    if missing := sorted((fixture - non_natural_spawn) - set(modifiers)):
        errors.append(f"fixture species without spawn modifier: {missing}")
    return {
        "metadata_species": len(metadata),
        "variant_species": len(variants),
        "entity_species": len(entities),
        "spawn_modifiers": len(modifiers),
        "fixture_species_with_rows": len(fixture & set(modifiers)),
        "errors": errors,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path, nargs="?", default=Path(__file__).parents[1])
    args = parser.parse_args()
    result = verify(args.root)
    print(json.dumps(result, indent=2, sort_keys=True))
    return int(bool(result["errors"]))


if __name__ == "__main__":
    raise SystemExit(main())
