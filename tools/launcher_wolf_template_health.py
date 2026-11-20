"""Verify Wolf-backed species health defaults and saved overrides."""
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
            for kind, tag in (("ant", "ant"), ("hedgehog", "hedge"), ("crab", "crab")):
                command(f'summon naturalist:{kind} 8 72 -6 {{Tags:[{tag}],NoAI:1b}}')
            command('summon naturalist:ant 8 72 -6 {Tags:[custom],NoAI:1b,attributes:[{id:"minecraft:max_health",base:13d}]}')
            def values():
                return {tag: command(f'attribute @e[tag={tag},limit=1] minecraft:max_health base get')
                        for tag in ("ant", "hedge", "crab", "custom")}
            initial = values()
            defaults = {"ant": "6.0", "hedge": "12.0", "crab": "8.0", "custom": "13.0"}
            assert all(initial[tag].endswith(value) for tag, value in defaults.items()), initial
            command('data merge entity @e[tag=ant,limit=1] {attributes:[{id:"minecraft:max_health",base:17d}]}')
            expected = {"ant": "17.0", "hedge": "12.0", "crab": "8.0", "custom": "13.0"}
            before = values()
            assert all(before[tag].endswith(value) for tag, value in expected.items()), before
            ctl('save')
            quit_and_wait()
            launch()
            after = values()
            assert all(after[tag].endswith(value) for tag, value in expected.items()), after
            print(json.dumps({"before": before, "after": after}, indent=2))
        finally:
            if socket.exists():
                quit_and_wait()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('version', type=Path)
    parser.add_argument('jar', type=Path)
    args = parser.parse_args()
    check(args.version, args.jar)
