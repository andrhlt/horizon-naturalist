"""Play Bird spawn-egg calls at noon and midnight without ambient roll timing."""
import argparse
import json
from pathlib import Path
import socket
import subprocess
import tempfile


def capture(version, fixture):
    with tempfile.TemporaryDirectory(prefix="nat-bird-voice-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        path = directory / "voice.sock"

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", path, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def raw(command):
            with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as channel:
                channel.settimeout(8)
                channel.connect(str(path))
                channel.sendall((command + "\n").encode())
                with channel.makefile() as stream:
                    return json.loads(stream.readline())

        def cmd(line):
            return ctl("command", line)["chat"]

        def heard():
            return {event: count for event, count in ctl("status")["heard"].items()
                    if event.startswith("naturalist:entity.bird.ambient_")}

        def live_particles_plus_one():
            return raw("particle end_rod 7.5 102.5 8.5 0 0 0 1")["live"]

        def step(ticks):
            cmd(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        def discharge(variant):
            cmd("setblock 8 100 8 minecraft:air")
            cmd("setblock 8 101 8 minecraft:air")
            cmd("setblock 8 100 8 minecraft:dispenser[facing=east]")
            stack = ('{Items:[{Slot:0b,id:"naturalist:bird_spawn_egg",count:1,'
                     'components:{"minecraft:entity_data":{id:"naturalist:bird",'
                     f'Variant:"naturalist:{variant}",NoAI:1b,Tags:[voice_fixture]'
                     '}}}]}')
            setup = cmd("data merge block 8 100 8 " + stack)
            assert any("Modified block data" in row for row in setup), setup
            sound_before = heard()
            particles_before = live_particles_plus_one()
            cmd("setblock 8 101 8 minecraft:redstone_block")
            step(4)
            sound_after = heard()
            particles_after = live_particles_plus_one()
            birds = ctl("entities", 10, "naturalist:bird")["entities"]
            assert len(birds) == 1, (variant, birds)
            cmd("kill @e[tag=voice_fixture]")
            step(30)
            calls = [key for key, count in sound_after.items() if count > sound_before.get(key, 0)]
            return (calls[0] if len(calls) == 1 else None), particles_after - particles_before - 1

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", directory, "--game-dir", directory,
                "--world", "Bird Voices Fixture", "--creative", "--test-terrain",
                "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 7.5 103 6.5")
            ctl("wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 105 15 minecraft:air")
            cmd("tick freeze")
            cmd("time set noon")
            step(2)
            noon = {variant: discharge(variant) for variant in fixture["day"]}
            cmd("time set midnight")
            step(2)
            midnight = {variant: discharge(variant) for variant in ("blue_jay", "northern_cardinal")}
            print(json.dumps({"noon": noon, "midnight": midnight}, indent=2), flush=True)
            assert {variant: call for variant, (call, _) in noon.items()} == fixture["day"], noon
            assert all(extra >= 1 for _, extra in noon.values()), noon
            assert all(call is None and extra == 0 for call, extra in midnight.values()), midnight
        finally:
            if path.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    capture(args.version, json.loads(args.fixture.read_text()))
