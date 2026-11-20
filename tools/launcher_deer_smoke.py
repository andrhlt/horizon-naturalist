"""Check deer loot, cooking, and Java Forest Dasher recipes on a launcher-built version."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import time

from forest_dasher import brew


def capture(version, fixture):
    with tempfile.TemporaryDirectory(prefix='naturalist-native-brew-', ignore_cleanup_errors=True) as tmp:
        home = Path(tmp)
        socket = home / 'brew.sock'

        def control(*words):
            cmd = [version / 'horizon-ctl', '--socket', socket, *map(str, words)]
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=75)
            if result.returncode:
                raise RuntimeError((words, result.stdout, result.stderr))
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            return control('command', line)['chat']

        try:
            control('launch', '--headless', '--game', version / 'horizon-client',
                    '--resources', version / '.local/horizon-client/26.3',
                    '--home', home, '--game-dir', home, '--world', 'Naturalist Launcher Brewing QA',
                    '--creative', '--test-terrain', '--render-distance', 4, '--wait')
            command('tp @s 8.5 102 5.5')
            control('wait', 35)
            command('gamerule minecraft:spawn_mobs false')
            command('fill 4 100 4 13 100 13 minecraft:stone')
            command('fill 4 101 4 13 106 13 minecraft:air')
            summon = command('summon naturalist:deer 8.5 101 9.5 {NoAI:1b}')
            assert any('Summoned' in row for row in summon), summon
            print('Registered and spawned deer from native bundle', flush=True)
            tagged = '@e[type=naturalist:deer,tag=loot_fixture,limit=1]'
            assert any('Summoned' in r for r in command(
                'summon naturalist:deer 10.5 101 9.5 {NoAI:1b,Tags:[loot_fixture]}'))
            awarded = command(f'loot give @s kill {tagged}')
            inventory = command('data get entity @s Inventory')
            assert any('naturalist:venison' in row for row in inventory), (awarded, inventory)
            print('Loot command awarded Naturalist venison', flush=True)
            command(f'kill {tagged}')
            control('wait', 32)
            drops = control('entities', 32, 'minecraft:item')['entities']
            assert any(drop['item']['item'] == 'naturalist:venison' for drop in drops), drops
            print('The deer dropped Naturalist venison on death', flush=True)
            command('setblock 6 101 7 minecraft:furnace')
            command('data merge block 6 101 7 {Items:[{Slot:0b,id:"naturalist:venison",count:1},'
                    '{Slot:1b,id:"minecraft:coal",count:1}],cooking_time_spent:195,cooking_total_time:200}')
            control('wait', 30)
            cooked = command('data get block 6 101 7 Items')
            assert any('naturalist:cooked_venison' in row for row in cooked), cooked
            print('A furnace cooked the launcher bundle’s venison', flush=True)

            command('tick freeze')
            reference = json.loads(fixture.read_text())
            output = brew(command, 10, 101, 8)
            assert output == reference['forest_dasher'], (output, reference.keys())
            print(json.dumps(output, sort_keys=True), flush=True)
            command('tick unfreeze')
            command('gamemode survival @s')
            feedback = command('item replace entity @s weapon.mainhand with minecraft:potion[minecraft:potion_contents={potion:"naturalist:strong_forest_dasher"}]')
            assert any('slot' in row.lower() or 'replaced' in row.lower() for row in feedback), feedback
            control('use-item', '--hold', 2.2)
            effects = control('effects')
            print(json.dumps(effects, sort_keys=True), flush=True)
            assert {effect['id']: effect['amplifier'] for effect in effects['effects']['active']} == {'speed': 2, 'weakness': 1}, effects
        finally:
            if socket.exists():
                control('quit')
                deadline = time.monotonic() + 10
                while socket.exists() and time.monotonic() < deadline:
                    time.sleep(0.1)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=lambda path: Path(path).resolve(strict=True),
                        help="Directory produced by the Horizon launcher’s native build")
    parser.add_argument("fixture", type=lambda path: Path(path).resolve(strict=True),
                        help="Java reference JSON containing the Forest Dasher recipe results")
    args = parser.parse_args()
    capture(args.version, args.fixture)


if __name__ == "__main__":
    main()
