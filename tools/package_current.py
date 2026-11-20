"""Pack the root development project with its external Naturalist jar reference."""
import argparse
import hashlib
import json
from pathlib import Path
import tomllib
from zipfile import ZIP_DEFLATED, ZipFile, ZipInfo

from prepare import JAR_SHA, ROOT


def build(jar: Path, output: Path):
    if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
        raise ValueError('expected Naturalist 2.0.7 NeoForge for Minecraft 26.3')
    manifest = (ROOT / 'horizon-mod.toml').read_bytes()
    config = tomllib.loads(manifest.decode())
    if (config.get('id') != 'naturalist' or config.get('version') != '0.8.0-dev.1'
            or config.get('native', {}).get('horizon') != '0a875da91dbb0fdcb7ba8c42c11341aa229af51a'
            or config.get('jar') != [{'id': 'naturalist', 'version': '2.0.7', 'sha512': JAR_SHA}]):
        raise ValueError('root manifest does not match the pinned development project')
    paths = sorted(p for p in (ROOT / 'data').rglob('*') if p.is_file())
    with ZipFile(jar) as original:
        originals = set(original.namelist())
        for path in paths:
            name = path.relative_to(ROOT).as_posix()
            if name in originals and path.read_bytes() == original.read(name):
                raise ValueError(f'original jar resource must stay external: {name}')
    payloads = {
        'horizon-mod.toml': manifest,
        'Cargo.toml': (ROOT / 'Cargo.toml').read_bytes(),
        'README.txt': b'Development-only stage. Naturalist gameplay and rendering are incomplete.\nSee README-omitted-attributes.txt for content placeholder overrides.\n',
        'README-omitted-attributes.txt': (ROOT / 'README-omitted-attributes.txt').read_bytes(),
    }
    for path in paths:
        payloads[path.relative_to(ROOT).as_posix()] = path.read_bytes()
    for path in sorted((ROOT / 'src/current').rglob('*.rs')):
        payloads[path.relative_to(ROOT).as_posix()] = path.read_bytes()
    output.parent.mkdir(parents=True, exist_ok=True)
    with ZipFile(output, 'w', ZIP_DEFLATED, compresslevel=9) as z:
        for name, data in sorted(payloads.items()):
            info = ZipInfo(name, (1980, 1, 1, 0, 0, 0))
            info.compress_type = ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            z.writestr(info, data, compresslevel=9)
    return {kind: sum(name.startswith(f'data/naturalist/horizon/{kind}/') for name in payloads)
            for kind in ('entity_type', 'item', 'block')}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('jar', type=Path, help='Naturalist 2.0.7 NeoForge for Minecraft 26.3')
    parser.add_argument('output', type=Path, help='development .hmod output path')
    args = parser.parse_args()
    if args.output.suffix != '.hmod':
        parser.error('output must end in .hmod')
    print(json.dumps(build(args.jar, args.output), indent=2))


if __name__ == '__main__':
    main()
