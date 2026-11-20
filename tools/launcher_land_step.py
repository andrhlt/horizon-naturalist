"""Sample audible steps of the port's land animals in motion."""
from argparse import ArgumentParser
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-land-step-", ignore_cleanup_errors=True) as temp:
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
                "--world", "Naturalist Land Footsteps", "--creative", "--test-terrain",
                "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 5.5 101 8.5")
            ctl("wait", 25)
            command("fill 0 100 0 15 100 15 minecraft:stone")
            command("fill 0 101 0 15 108 15 minecraft:air")
            command("tp @s 5.5 101 8.5")
            command("gamemode survival @s")
            command("effect give @s minecraft:resistance infinite 4 true")
            command("gamerule minecraft:advance_time false")
            command("time set midnight")
            output = {}
            for label, lure in (("capybara", "melon_slice"), ("turkey", "wheat_seeds"),
                                ("lizard", "spider_eye"), ("komodo_dragon", "porkchop"),
                                ("desert_scorpion", "air"), ("jungle_scorpion", "air"),
                                ("rhino", "air"), ("lizard_tail", "air")):
                command(f"item replace entity @s weapon.mainhand with minecraft:{lure}")
                command("tick freeze")
                command("kill @e[tag=audible_land_mob]")
                command("tp @s 5.5 101 8.5")
                if label == "rhino":
                    command("summon naturalist:rhino 14.5 101 9.5 {Age:-24000,NoAI:1b,Silent:1b,Tags:[rhino_calf]}")
                motion = "Motion:[-0.9d,0.0d,0.0d]," if label == "lizard_tail" else ""
                command(f"summon naturalist:{label} 13.5 101 8.5 {{{motion}Tags:[audible_land_mob]}}")
                baseline = heard()
                command("tick step 100")
                ctl("wait", 102)
                rows = command("data get entity @e[tag=audible_land_mob,limit=1] Pos")
                output[label] = {
                    "position": next(line.split("following entity data: ", 1)[1] for line in rows
                                     if "following entity data: " in line),
                    "steps": {name: count - baseline.get(name, 0) for name, count in heard().items()
                              if count > baseline.get(name, 0)},
                }
                if label == "rhino":
                    command("kill @e[tag=rhino_calf]")
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
    assert actual.keys() == expected.keys()
    for name, (step, _, _) in expected.items():
        heard = set(actual[name]["steps"])
        if name == "rhino":
            heard.discard("minecraft:entity.hoglin.step")  # Special charge cue.
        assert heard == {step}, (name, actual[name], step)
