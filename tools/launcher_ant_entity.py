"""Verify ant-hill block-entity creation and payload persistence headlessly."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import time

from prepare import JAR_SHA


def check(version: Path, jar: Path):
    if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
        raise ValueError('expected the pinned Naturalist 2.0.7 NeoForge 26.3 jar')
    with tempfile.TemporaryDirectory(prefix='naturalist-ant-entity-', ignore_cleanup_errors=True) as tmp:
        home = Path(tmp)
        (home / 'jars').mkdir()
        shutil.copyfile(jar, home / 'jars' / f'{JAR_SHA}.jar')
        socket = home / 'control.sock'

        def ctl(*args):
            result = subprocess.run([version / 'horizon-ctl', '--socket', socket, *map(str, args)],
                                    capture_output=True, text=True, check=True, timeout=90)
            return json.loads(result.stdout.splitlines()[-1])

        def command(text):
            response = ctl('command', text)
            if not response.get('ok'):
                raise AssertionError((text, response))
            return ' '.join(response['chat'])

        def launch():
            ctl('launch', '--headless', '--game', version / 'horizon-client', '--resources',
                version / '.local/horizon-client/26.3', '--home', home, '--game-dir', home,
                '--world', 'Ant Entity Test', '--creative', '--test-terrain', '--render-distance', '4', '--wait')

        def quit_and_wait():
            ctl('quit')
            deadline = time.monotonic() + 10
            while socket.exists() and time.monotonic() < deadline:
                time.sleep(.1)
            if socket.exists():
                raise RuntimeError('game control socket did not close')

        try:
            launch()
            command('setblock 8 72 -6 naturalist:ant_hill')
            initial = command('data get block 8 72 -6')
            assert 'Storage: []' in initial and 'naturalist:ant_hill' in initial, initial
            command('data merge block 8 72 -6 {Owner:[I;1,2,3,4],Storage:[{id:"minecraft:wheat",count:3}]}')
            ctl('save')
            quit_and_wait()
            launch()
            saved = command('data get block 8 72 -6')
            assert 'Owner: [I; 1, 2, 3, 4]' in saved and 'minecraft:wheat' in saved and 'count: 3' in saved, saved
            print(json.dumps({'initial': initial, 'reopened': saved}, indent=2))
        finally:
            if socket.exists():
                quit_and_wait()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('version', type=Path)
    parser.add_argument('jar', type=Path)
    args = parser.parse_args()
    check(args.version, args.jar)
