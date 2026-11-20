"""Verify that an upgraded native version contains exactly its own refreshed brewing aliases."""
import argparse
import hashlib
import json
from pathlib import Path


def aliases(version):
    return version / ".local/horizon-client/26.3/data/minecraft/recipe/brewing"


def digests(files):
    return {file.name: hashlib.sha256(file.read_bytes()).hexdigest() for file in files}


def verify(previous, upgraded, unchanged):
    prior = digests(aliases(previous).glob("naturalist_*.json"))
    assert prior == unchanged, "The previously installed version changed"
    target = aliases(upgraded)
    original = upgraded / "native-content/naturalist/data/naturalist/recipe/brewing"
    sources = sorted(original.glob("*.json"))
    expected = {f"naturalist_{file.name}" for file in sources}
    assert expected == {file.name for file in target.glob("naturalist_*.json")}
    assert all((target / f"naturalist_{file.name}").read_bytes() == file.read_bytes()
               for file in sources)
    assert not any(target.glob("naturalist_*.tmp.*"))
    upgraded_hashes = digests(target.glob("naturalist_*.json"))
    changed = sum(upgraded_hashes[name] != prior.get(name) for name in expected)
    assert changed, "No stale alias was exercised by this upgrade"
    return {"aliases": len(sources), "refreshed": changed, "previous_version_unchanged": True}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("previous_version", "upgraded_version", "previous_checksums"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    print(json.dumps(verify(args.previous_version, args.upgraded_version,
                            json.loads(args.previous_checksums.read_text())), indent=2))
