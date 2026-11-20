"""Exercise real Rat breeding and inspect the live child's owner and selected coat."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())

with tempfile.TemporaryDirectory(prefix="nat-rat-breed-", ignore_cleanup_errors=True) as scratch:
    home = Path(scratch)
    socket = home / "control.sock"

    def ctl(*args):
        proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                              check=True, capture_output=True, text=True, timeout=75)
        return json.loads(proc.stdout.splitlines()[-1])

    def command(line):
        return ctl("command", line)["chat"]

    def step(ticks):
        command(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    def data(target, field):
        reply = " following entity data: "
        return next((line.split(reply, 1)[1] for line in command(f"data get entity {target} {field}")
                     if reply in line), None)

    try:
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Naturalist Rat Breeding", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        command("gamerule minecraft:spawn_mobs false")
        command("gamerule minecraft:advance_time false")
        command("tp @s 7.5 92 3.5")
        ctl("wait", 24)
        command("tick freeze")
        command("fill 2 91 2 15 91 14 minecraft:stone")
        command("fill 2 92 2 15 98 14 minecraft:air")
        command("time set 1000")
        owner = data("@s", "UUID")
        assert owner
        for x, variant in ((8.5, "white"), (9.5, "brown")):
            command(f'summon naturalist:rat {x} 92 5.5 '
                    f'{{Owner:{owner},Variant:"naturalist:{variant}",Tags:[rat_parent],InLove:600}}')
        child = "@e[type=naturalist:rat,tag=!rat_parent,limit=1]"
        step(90)
        age = data(child, "Age")
        actual_owner = data(child, "Owner")
        coat = (data(child, "Variant") or "").strip('"')
        print({"child_age": age, "actual_owner": actual_owner, "expected_owner": owner, "coat": coat}, flush=True)
        assert age is not None and int(age) < 0, age
        assert coat in {"naturalist:white", "naturalist:brown"}, coat
        assert actual_owner == owner and reference["offspring"][:2] == [True, True], actual_owner
    finally:
        if socket.exists():
            ctl("quit")
