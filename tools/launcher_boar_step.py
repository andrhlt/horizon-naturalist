"""Sample a boar's audible steps while it follows a player holding food."""
from argparse import ArgumentParser
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-boar-step-", ignore_cleanup_errors=True) as temp:
        home = Path(temp)
        socket = home / "step.sock"

        def ctl(*args):
            run = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                 check=True, capture_output=True, text=True, timeout=75)
            return json.loads(run.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        def heard():
            return {key: count for key, count in ctl("status")["heard"].items() if key.endswith(".step")}

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Naturalist Boar Footsteps", "--creative", "--test-terrain",
                "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 5.5 101 8.5")
            ctl("wait", 25)
            command("fill 0 100 0 15 100 15 minecraft:stone")
            command("fill 0 101 0 15 108 15 minecraft:air")
            command("tp @s 5.5 101 8.5")
            command("gamemode survival @s")
            command("item replace entity @s weapon.mainhand with minecraft:carrot")
            output = {}
            for label, age in (("adult", 0), ("juvenile", -24000)):
                command("tick freeze")
                command("kill @e[tag=audible_boar]")
                command(f"summon naturalist:boar 13.5 101 8.5 {{Age:{age},Tags:[audible_boar]}}")
                baseline = heard()
                command("tick step 100")
                ctl("wait", 102)
                rows = command("data get entity @e[tag=audible_boar,limit=1] Pos")
                output[label] = {
                    "position": next(line.split("following entity data: ", 1)[1] for line in rows
                                     if "following entity data: " in line),
                    "steps": {name: count - baseline.get(name, 0) for name, count in heard().items()
                              if count > baseline.get(name, 0)},
                }
            return output
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    cli = ArgumentParser(description=__doc__)
    cli.add_argument("version", type=Path)
    cli.add_argument("reference", type=Path)
    args = cli.parse_args()
    actual = capture(args.version)
    expected = json.loads(args.reference.read_text())
    print(json.dumps(actual, indent=2), flush=True)
    for stage, (step, _, _) in zip(("adult", "juvenile"), expected, strict=True):
        assert set(actual[stage]["steps"]) == {step}, (stage, actual[stage], step)
