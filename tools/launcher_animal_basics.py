"""Compare registered Horizon bass or bird with its Java attribute and variant fixture."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version, species, reference):
    with tempfile.TemporaryDirectory(prefix="nat-entity-", ignore_cleanup_errors=True) as work:
        home = Path(work)
        socket = home / "qa.sock"

        def control(*words):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, words)],
                                    check=True, capture_output=True, text=True, timeout=75)
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            return control("command", line)["chat"]

        try:
            control("launch", "--headless", "--game", version / "horizon-client",
                    "--resources", version / ".local/horizon-client/26.3", "--home", home,
                    "--game-dir", home, "--world", "Naturalist Entity Fixture", "--creative",
                    "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 101 6.5")
            control("wait", 20)
            command("fill 1 100 1 15 100 15 minecraft:stone")
            command("fill 1 101 1 15 106 15 minecraft:air")
            command("tick freeze")
            output = {"attributes": {}, "variants": {}}
            for index, variant in enumerate(reference["variants"]):
                rows = command(f'summon naturalist:{species} {5 + 3 * index} 101 8 '
                               f'{{Variant:"naturalist:{variant}",NoAI:1b,Tags:[nat_fixture,{variant}]}}')
                assert any("Summoned" in row for row in rows), rows
                selector = f"@e[tag={variant},tag=nat_fixture,limit=1]"
                rows = command(f"data get entity {selector} Variant")
                output["variants"][variant] = next((row.split("following entity data: ", 1)[1]
                                                   for row in rows if "following entity data: " in row), None)
                if index == 0:
                    for attribute in reference["attributes"]:
                        rows = command(f"attribute {selector} minecraft:{attribute} base get")
                        output["attributes"][attribute] = next((float(m[1]) for row in rows
                                                               if (m := re.search(r" is ([0-9.eE+-]+)", row))), None)
                    if species == "bass":
                        command(f"loot give @s kill {selector}")
                        output["raw_bass_loot"] = any('id: "naturalist:bass"' in line
                                                     for line in command("data get entity @s Inventory"))
            if species == "bird":
                rows = command('summon naturalist:bird 5 101 7 {Age:-24000,NoAI:1b,Tags:[bird_baby]}')
                assert any("Summoned" in row for row in rows), rows
                rows = command("data get entity @e[tag=bird_baby,limit=1] Age")
                output["bird_baby_age"] = next((row.split("following entity data: ", 1)[1] for row in rows
                                              if "following entity data: " in row), None)
                output["bird_baby_age_query"] = rows
        finally:
            if socket.exists():
                control("quit")

    print(json.dumps(output, indent=2), flush=True)
    assert output["variants"] == reference["variants"], output
    for name, value in reference["attributes"].items():
        actual = output["attributes"][name]
        assert actual is not None and abs(actual - value) < 1e-6, (actual, name, value)
    if species == "bass":
        assert output["raw_bass_loot"], output
    else:
        assert output["bird_baby_age"] is None, output
        assert any("Age" in row and ("no element" in row.lower() or "no data" in row.lower())
                   for row in output["bird_baby_age_query"]), output


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("species", choices=("bass", "bird"))
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    capture(args.version, args.species, json.loads(args.fixture.read_text()))
