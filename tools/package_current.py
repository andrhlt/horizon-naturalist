"""Build a development-only Naturalist content stage with an external original jar.

Without --native-project this registers content only. An optional development
native project adds its implemented features; neither form is a release or a
full Naturalist parity claim.
"""
import argparse
import hashlib
import json
import tomllib
from pathlib import Path
from tempfile import TemporaryDirectory
from zipfile import ZIP_DEFLATED, ZipFile, ZipInfo

import package_launcher as legacy
from prepare import JAR_SHA, dump, pack_deer


def build(jar: Path, output: Path, native_project: Path | None = None):
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
        manifest = f'''id = "naturalist"
name = "Naturalist content development stage"
version = "0.8.0-dev.1"
description = "Development content stage; Naturalist gameplay and rendering are incomplete."
api = "0.2"
side = "both"
capabilities = ["registries"]

[[jar]]
id = "naturalist"
version = "2.0.7"
sha512 = "{JAR_SHA}"
'''
        native_files = {}
        if native_project is not None:
            native_text = (native_project / 'horizon-mod.toml').read_text()
            native = tomllib.loads(native_text)
            if (native.get('id') != 'naturalist' or native.get('version') != '0.8.0-dev.1'
                    or native.get('native', {}).get('horizon') != '0a875da91dbb0fdcb7ba8c42c11341aa229af51a'):
                raise ValueError('native project identity, version or Horizon pin differs from the development stage')
            manifest += '\n' + native_text[native_text.index('[native]'):]
            native_files['Cargo.toml'] = (native_project / 'Cargo.toml').read_bytes()
            for path in sorted((native_project / 'src').rglob('*.rs')):
                native_files[path.relative_to(native_project).as_posix()] = path.read_bytes()
        output.parent.mkdir(parents=True, exist_ok=True)
        payloads = {
            'horizon-mod.toml': manifest.encode(),
            'README.txt': b'Development-only stage. Naturalist gameplay and rendering are incomplete.\nSee README-omitted-attributes.txt for content placeholder overrides.\n',
            'README-omitted-attributes.txt': (json.dumps(omitted, sort_keys=True, indent=2) + '\n').encode(),
        }
        payloads.update({name: path.read_bytes() for path, name in zip(paths, names)})
        payloads.update(native_files)
        # Stable timestamps, ordering and permissions make builds reproducible.
        with ZipFile(output, 'w', ZIP_DEFLATED, compresslevel=9) as z:
            for name, data in sorted(payloads.items()):
                info = ZipInfo(name, (1980, 1, 1, 0, 0, 0))
                info.compress_type = ZIP_DEFLATED
                info.external_attr = 0o100644 << 16
                z.writestr(info, data, compresslevel=9)
        return counts


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('jar', type=Path, help='Naturalist 2.0.7 NeoForge for Minecraft 26.3')
    p.add_argument('output', type=Path, help='development .hmod output path')
    p.add_argument('--native-project', type=Path, help='optionally combine the experimental native project')
    a = p.parse_args()
    if a.output.suffix != '.hmod':
        p.error('output must end in .hmod')
    print(json.dumps(build(a.jar, a.output, a.native_project), indent=2))


if __name__ == '__main__':
    main()
