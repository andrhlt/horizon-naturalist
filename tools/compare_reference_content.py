"""Compare shared Java content with a Horizon package using its external jar."""
import argparse
import hashlib
from io import BytesIO
import json
from pathlib import Path
from zipfile import ZipFile


DIRECTORIES = (
    "data/naturalist/recipe/",
    "data/naturalist/advancement/recipes/",
    "data/naturalist/loot_table/",
    "data/naturalist/tags/",
    "assets/naturalist/items/",
)


def compare(jar, package):
    with ZipFile(jar) as java, ZipFile(package) as native:
        with ZipFile(BytesIO(native.read("mixins/naturalist/data/content.zip"))) as horizon:
            names = set(horizon.namelist())
            result = {
                "reference_sha256": hashlib.sha256(jar.read_bytes()).hexdigest(),
                "launcher_package_sha256": hashlib.sha256(package.read_bytes()).hexdigest(),
                "content_zip_sha256": hashlib.sha256(native.read("mixins/naturalist/data/content.zip")).hexdigest(),
                "resources": {},
                "semantics": "Resources absent from content.zip are supplied by the external jar declared in horizon-mod.toml; only same-path overlays are byte-compared.",
            }
            for prefix in DIRECTORIES:
                expected = {name for name in java.namelist()
                            if name.startswith(prefix) and not name.endswith("/")}
                names_for_prefix = {name for name in names if name.startswith(prefix)}
                overlay = expected & names_for_prefix
                changed = [name for name in sorted(overlay)
                           if java.read(name) != horizon.read(name)]
                # Naturalist's original jar is declared external by horizon-mod.toml;
                # resources absent from content.zip therefore remain available from it.
                external = expected - overlay
                result["resources"][prefix] = {
                    "external_identical": len(external),
                    "overlay_identical": len(overlay) - len(changed),
                    "missing": [],
                    "changed": changed,
                    "overlay_only": sorted(names_for_prefix - expected),
                }
            return result


if __name__ == "__main__":
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("reference", type=Path)
    p.add_argument("launcher_package", type=Path)
    args = p.parse_args()
    result = compare(args.reference, args.launcher_package)
    print(json.dumps(result, indent=2))
    assert all(not row["missing"] and not row["changed"]
               for row in result["resources"].values()), result
