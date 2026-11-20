import argparse
import datetime
import hashlib
import json
from pathlib import Path
from zipfile import ZipFile, ZIP_DEFLATED


def package(base_index, existing_module, output):
    index = json.loads(base_index.read_text())
    main = index["projects"][0]["versions"][0]
    _, horizon, mixins = main["api"].split("/")
    project = existing_module.name
    version = "0.1.1"
    title = "Naturalist Bird region RNG fixture"
    output.mkdir(parents=True, exist_ok=True)
    bundle = output / f"{project}-{version}.hnmod"
    with ZipFile(bundle, "w", ZIP_DEFLATED) as zip_file:
        metadata = {"schema": 1, "id": project, "version": version, "game": main["game"],
                    "horizon": horizon, "mixins": mixins}
        zip_file.writestr("horizon-native.json", json.dumps(metadata))
        for file in sorted(existing_module.iterdir()):
            if file.is_file():
                zip_file.write(file, f"mixins/{project}/{file.name}")
    sha256 = hashlib.sha256(bundle.read_bytes()).hexdigest()
    index["projects"].append({
        "id": project, "kind": "native-mod", "name": title,
        "summary": "Headless fixture for the chunk generation random stream",
        "authors": ["Naturalist port"], "owners": ["Naturalist port"], "license": "MIT",
        "source": index["projects"][0]["source"], "categories": [],
        "versions": [{"version": version, "published": str(datetime.date.today()),
                      "api": main["api"], "game": main["game"],
                      "file": {"name": bundle.name, "url": bundle.as_uri(),
                               "size": bundle.stat().st_size, "sha256": sha256}}],
    })
    (output / "local-index.json").write_text(json.dumps(index, indent=2) + "\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    for name in ("base_index", "existing_module", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    package(args.base_index, args.existing_module, args.output)
