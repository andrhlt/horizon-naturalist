#!/usr/bin/env python3
"""Connect a headless Horizon client to an already running test server.

The client, resources and original Naturalist jar are explicit inputs so this
checks the chosen candidate rather than whichever launcher version is selected.
The server should use the same source artifact and pinned Horizon revision.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

JAR_SHA512 = ('b1a220dc6bcb6101911226a57d75d62f2797461c3e529e8cf7110673b69632538'
              'c4a129b135f9572411cbc60336c096e056488fbe1528638196c247eef6492cd')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--version', type=Path, required=True, help='Exact built client directory (contains horizon-client and horizon-ctl)')
    parser.add_argument('--resources', type=Path, required=True, help='Prepared 26.3 resource directory')
    parser.add_argument('--jar', type=Path, required=True, help='Original pinned Naturalist jar')
    parser.add_argument('--server', required=True, help='Test server host:port, e.g. a loopback SSH tunnel')
    parser.add_argument('--name', default='NatCrossCheck')
    args = parser.parse_args()
    if not 1 <= len(args.name) <= 16 or not all(c.isascii() and (c.isalnum() or c == '_') for c in args.name):
        parser.error('player name must be 1–16 ASCII letters, digits, or underscores')
    version, resources, jar = (p.expanduser().resolve(strict=True) for p in (args.version, args.resources, args.jar))
    client, controller = version / 'horizon-client', version / 'horizon-ctl'
    for path in (client, controller):
        if not path.is_file():
            parser.error(f'candidate is missing {path}')
    if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA512:
        parser.error('Naturalist jar SHA-512 does not match the pinned source')

    with tempfile.TemporaryDirectory(prefix='naturalist-cross-') as work:
        root = Path(work)
        socket, home = root / 'client.sock', root / 'client'
        (home / 'jars').mkdir(parents=True)
        shutil.copy2(jar, home / 'jars' / f'{JAR_SHA512}.jar')
        env = {**os.environ, 'HORIZON_NO_AUDIO': '1'}

        def ctl(*command):
            p = subprocess.run([str(controller), '--socket', str(socket), *map(str, command)],
                               cwd=root, env=env, capture_output=True, text=True, timeout=100)
            if p.returncode:
                raise RuntimeError(f'{command}: {p.stdout}\n{p.stderr}')
            return json.loads(p.stdout.splitlines()[-1])

        try:
            ctl('launch', '--headless', '--game', client, '--resources', resources,
                '--home', home, '--game-dir', home, '--name', args.name, '--render-distance', 4)
            ctl('connect', args.server, args.name)
            ctl('wait-for', 'loaded', '--timeout', 80)
            state = ctl('state')
            if state.get('screen') != 'playing':
                raise RuntimeError(f'client did not enter the world: {state}')
            print(json.dumps({'connected': True, 'server': args.server, 'client': str(client),
                              'resources': str(resources), 'player': args.name, 'state': state}))
        finally:
            if socket.exists():
                ctl('quit')


if __name__ == '__main__':
    main()
