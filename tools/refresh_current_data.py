"""Regenerate or check the root project’s generated content from the pinned jar."""
import argparse
import hashlib
import json
import tomllib
from pathlib import Path
from tempfile import TemporaryDirectory
from zipfile import ZIP_DEFLATED, ZipFile, ZipInfo

import package_launcher as legacy
from prepare import JAR_SHA, dump, pack_deer


def generate(jar: Path):
    if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
        raise ValueError('expected the exact Naturalist 2.0.7 NeoForge for Minecraft 26.3 jar')
    with ZipFile(jar) as original:
        original_files = {name: original.read(name) for name in original.namelist() if not name.endswith('/')}
        original_paths = set(original_files)
    with TemporaryDirectory(prefix='naturalist-content-') as temp:
        stage = Path(temp)
        pack_deer(jar, stage)
        for name in ('catfish', 'blobfish', 'ray', 'jellyfish', 'starfish', 'alligator',
                     'hippo', 'shark', 'whale', 'crab', 'giraffe', 'giant_isopod',
                     'piranha', 'duck', 'bass', 'anglerfish', 'bird', 'insects', 'clam',
                     'wings', 'vulture', 'snail', 'decor', 'glow_goop', 'plush_bear',
                     'land', 'knapsack', 'elephants', 'ostrich', 'ant', 'hedgehog',
                     'cooked_egg', 'spawn_tables'):
            getattr(legacy, f'pack_{name}')(jar, stage, dump)
        # The legacy native content hook supplied the cow template; until that
        # behavior is ported, use the stock-supported placeholder from .45.
        deer = stage / 'data/naturalist/horizon/entity_type/deer.json'
        entity = json.loads(deer.read_text())
        entity['template'] = 'minecraft:wandering_trader'
        deer.write_text(json.dumps(entity, indent=2) + '\n')
        omitted = {}
        # The placeholder's attribute map lacks attack_damage and flying_speed.
        # They must be restored when the native entity registrations are ported.
        metadata = json.loads((Path(__file__).resolve().parents[1] / 'migration/entity-metadata.json').read_text())
        for path in (stage / 'data/naturalist/horizon/entity_type').glob('*.json'):
            entity = json.loads(path.read_text())
            profile = metadata.get('naturalist:' + path.stem, {})
            # Biome modifier composition routes from the content template,
            # before the native metadata hook corrects MobType categories.
            if profile.get('category') == 'creature' and entity.get('template') == 'minecraft:zombie':
                entity['template'] = 'minecraft:wandering_trader'
            if entity.get('template') == 'minecraft:wandering_trader':
                attributes = entity.get('attributes', {})
                for attribute in ('minecraft:attack_damage', 'minecraft:flying_speed'):
                    if attribute in attributes:
                        omitted.setdefault(path.stem, {})[attribute] = attributes.pop(attribute)
                path.write_text(json.dumps(entity, indent=2) + '\n')
        # Only our generated Horizon registry definitions are carried forward.
        # The original jar remains the source of every ordinary asset/data file.
        root = stage / 'data/naturalist/horizon'
        paths = [p for p in root.rglob('*') if p.is_file()]
        # The legacy generators produce actual NeoForge modifier overlays and
        # convention tags as well as copies of original data. Only generated
        # additions/changes are carried; exact jar files remain external.
        for subtree in ('data/naturalist/neoforge/biome_modifier', 'data/c/tags/worldgen/biome'):
            paths.extend(p for p in (stage / subtree).rglob('*') if p.is_file())
        paths = sorted(p for p in paths if original_files.get(p.relative_to(stage).as_posix()) != p.read_bytes())
        names = [p.relative_to(stage).as_posix() for p in paths]
        counts = {kind: sum(n.startswith(f'data/naturalist/horizon/{kind}/') for n in names)
                  for kind in ('entity_type', 'item', 'block')}
        result = {name: path.read_bytes() for name, path in zip(names, paths)}
        result['README-omitted-attributes.txt'] = (json.dumps(omitted, sort_keys=True, indent=2) + '\n').encode()
        return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('jar', type=Path)
    parser.add_argument('--write', action='store_true', help='replace generated project data; default checks it')
    args = parser.parse_args()
    expected = generate(args.jar)
    root = Path(__file__).resolve().parents[1]
    existing = {p.relative_to(root).as_posix() for p in (root / 'data').rglob('*') if p.is_file()}
    existing.add('README-omitted-attributes.txt')
    changed = sorted(name for name, value in expected.items()
                     if not (root / name).is_file() or (root / name).read_bytes() != value)
    extra = sorted(existing - set(expected))
    if args.write:
        for name in extra:
            (root / name).unlink(missing_ok=True)
        for name, value in expected.items():
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(value)
        print(f'Wrote {len(expected)} generated files')
    elif changed or extra:
        raise SystemExit(f'generated content differs: changed={changed}, extra={extra}')
    else:
        print(f'All {len(expected)} generated files match')


if __name__ == '__main__':
    main()
