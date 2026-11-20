"""Measure physical experience orbs from attributed and unattributed mob deaths."""
from argparse import ArgumentParser
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, species, repeats):
    with tempfile.TemporaryDirectory(prefix="nat-xp-", ignore_cleanup_errors=True) as temp:
        home = Path(temp)
        socket = home / "xp.sock"

        def ctl(*args):
            run = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                 check=True, capture_output=True, text=True, timeout=75)
            return json.loads(run.stdout.splitlines()[-1])

        def command(line):
            return ctl("command", line)["chat"]

        def measure(species, age, damage):
            command("kill @e[type=minecraft:experience_orb]")
            command(f"summon naturalist:{species} 13.5 150 8.5 {{Age:{age},NoAI:1b,Open:1b,Tags:[xp_mob]}}")
            hit = command(f"damage @e[tag=xp_mob,limit=1] 200 {damage}")
            assert any("applied" in line.lower() for line in hit), (species, hit)
            command("tick step 1")
            ctl("wait", 5)
            orbs = ctl("entities", 18, "minecraft:experience_orb")["entities"]
            return sum(row["value"] * row.get("count", 1) for row in orbs)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Naturalist XP Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("gamerule minecraft:mob_drops true")
            command("tp @s 3.5 150 8.5")
            ctl("wait", 25)
            command("fill 0 149 0 15 149 15 minecraft:stone")
            command("fill 0 150 0 15 155 15 minecraft:air")
            command("tp @s 3.5 150 8.5")
            command("tick freeze")
            result = {}
            for name in species:
                result[name] = {}
                for label, age in (("adult", 0), ("juvenile_tag", -24000)):
                    result[name][label] = [measure(name, age, "minecraft:player_attack by @s") for _ in range(repeats)]
            result["unattributed"] = measure("duck", 0, "minecraft:generic")
            command("gamerule minecraft:mob_drops false")
            result["no_mob_drops"] = measure("duck", 0, "minecraft:player_attack by @s")
            return result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    cli = ArgumentParser(description=__doc__)
    cli.add_argument("version", type=Path)
    cli.add_argument("reference", type=Path)
    cli.add_argument("--repeat", type=int, default=3)
    cli.add_argument("--species", nargs="+")
    args = cli.parse_args()
    if args.repeat < 1:
        cli.error("--repeat must be at least 1")
    expected = json.loads(args.reference.read_text())
    species = args.species or list(expected)
    actual = capture(args.version, species, args.repeat)
    print(json.dumps(actual, indent=2), flush=True)
    for name in species:
        for age, values in actual[name].items():
            assert set(values).issubset(expected[name][age]), (name, age, values, expected[name][age])
    assert actual["unattributed"] == actual["no_mob_drops"] == 0, actual
