import json
from pathlib import Path
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
with tempfile.TemporaryDirectory(prefix="nat-egg-", ignore_cleanup_errors=True) as work:
    home = Path(work)
    socket = home / "egg.sock"

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                check=True, capture_output=True, text=True, timeout=35)
        return json.loads(result.stdout.splitlines()[-1])

    def command(text):
        return ctl("command", text)["chat"]

    try:
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Naturalist Duck Egg Discovery", "--creative", "--test-terrain",
            "--render-distance", 4, "--wait")
        command("gamerule minecraft:spawn_mobs false")
        command("tp @s 8.5 101 5.5")
        ctl("wait", 20)
        command("fill 1 100 1 15 100 15 minecraft:stone")
        command("fill 1 101 1 15 107 15 minecraft:air")
        command("tp @s 8.5 101 5.5")
        ctl("look", 0, 65)
        ctl("wait", 10)
        command("item replace entity @s weapon.mainhand with naturalist:duck_egg")
        before = ctl("status")["heard"]
        ctl("mouse", "right", "click")
        ctl("wait", 12)
        after = ctl("status")["heard"]
        print("throw", {key: val - before.get(key, 0) for key, val in after.items()
                        if val > before.get(key, 0)}, flush=True)
        prefix = "Duck egg particles "
        rows = command("naturalist_fixture_duck_egg_particles")
        velocities = next(json.loads(row.split(prefix, 1)[1]) for row in rows if prefix in row)
        assert len(velocities) == 8, velocities
        assert all(-0.04 <= value < 0.04 for row in velocities for value in row), velocities
        assert any(value != 0 for row in velocities for value in row), velocities
        print(json.dumps({"particle_velocities": velocities}), flush=True)
        command("tick freeze")
        for attempt in range(144):
            reply = command('summon naturalist:duck_egg 8.5 103 8.5 {Motion:[0.0d,-0.8d,0.0d]}')
            if not any("Summoned" in line for line in reply):
                raise AssertionError((attempt, reply))
        command("tick step 8")
        ctl("wait", 10)
        babies = ctl("entities", 20, "naturalist:duck")["entities"]
        print(json.dumps({"hatchlings": len(babies), "all_babies": all(row["baby"] for row in babies)},
                         indent=2), flush=True)
        assert babies and all(row["baby"] for row in babies), babies
    finally:
        if socket.exists():
            ctl("quit")
