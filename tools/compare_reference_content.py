"""Compare the Java recipes, loot tables and item models with the installed Horizon package."""
import argparse
from io import BytesIO
import json
from pathlib import Path
from zipfile import ZipFile


DIRECTORIES = (
    "data/naturalist/recipe/",
    "data/naturalist/loot_table/entities/",
    "data/naturalist/loot_table/blocks/",
    "assets/naturalist/items/",
)


def compare(jar, package):
    with ZipFile(jar) as java, ZipFile(package) as native:
        with ZipFile(BytesIO(native.read("mixins/naturalist/data/content.zip"))) as horizon:
            names = set(horizon.namelist())
            result = {}
            for prefix in DIRECTORIES:
                expected = {name for name in java.namelist()
                            if name.startswith(prefix) and not name.endswith("/")}
                changed = [name for name in sorted(expected & names)
                           if java.read(name) != horizon.read(name)]
                result[prefix] = {
                    "identical": len(expected & names) - len(changed),
                    "missing": sorted(expected - names),
                    "changed": changed,
                }
            return result


if __name__ == "__main__":
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("reference", type=Path)
    p.add_argument("launcher_package", type=Path)
    args = p.parse_args()
    result = compare(args.reference, args.launcher_package)
    print(json.dumps(result, indent=2))
    assert all(not row["missing"] and not row["changed"] for row in result.values()), result
