"""Replay the Java duck fixture in a launcher-built Horizon client."""
import argparse
from pathlib import Path
import json
import re
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="naturalist-duck-", ignore_cleanup_errors=True) as temp:
        home = Path(temp)
        socket = home / "duck.sock"

        def control(*words):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, words)],
                                    check=True, capture_output=True, text=True, timeout=75)
            return json.loads(result.stdout.splitlines()[-1])

        def command(line):
            result = control("command", line)
            if not result["ok"]:
                raise RuntimeError((line, result))
            return result["chat"]

        try:
            control("launch", "--headless", "--game", version / "horizon-client",
                    "--resources", version / ".local/horizon-client/26.3", "--home", home,
                    "--game-dir", home, "--world", "Naturalist Duck Fixture", "--creative",
                    "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 101 5.5")
            control("wait", 25)
            command("fill 1 100 1 15 100 15 minecraft:stone")
            command("fill 1 101 1 15 106 15 minecraft:air")
            command("tick freeze")

            adult = "@e[type=naturalist:duck,tag=duck_adult,limit=1]"
            baby = "@e[type=naturalist:duck,tag=duck_baby,limit=1]"
            for command_line in (
                "summon naturalist:duck 5 101 8 {EggLayTime:2,NoAI:1b,Tags:[duck_adult]}",
                "summon naturalist:duck 9 101 8 {Age:-24000,EggLayTime:2,NoAI:1b,Tags:[duck_baby]}",
            ):
                rows = command(command_line)
                assert any("Summoned" in row for row in rows), rows

            def value(entity, key):
                query = f"data get entity {entity} {key}"
                return next((row.split("following entity data: ", 1)[1]
                             for row in command(query) if "following entity data: " in row), None)

            output = {"attributes": {}, "before": {}, "after": {}}
            for attribute in ("max_health", "movement_speed", "step_height"):
                rows = command(f"attribute {adult} minecraft:{attribute} base get")
                output["attributes"][attribute] = next((float(match[1]) for row in rows
                                                           if (match := re.search(r" is ([0-9.eE+-]+)", row))), None)
            for who, selector in (("adult", adult), ("baby", baby)):
                output["before"][who] = {key: value(selector, key)
                                          for key in ("Age", "Health", "EggLayTime", "Variant")}
            command("tick step 2")
            control("wait", 2)
            for who, selector in (("adult", adult), ("baby", baby)):
                output["after"][who] = {key: value(selector, key) for key in ("Age", "EggLayTime")}
            drops = control("entities", 32, "minecraft:item")["entities"]
            output["adult_laid_duck_egg"] = any(drop["item"]["item"] == "naturalist:duck_egg" for drop in drops)
            command(f"loot give @s kill {adult}")
            output["duck_loot"] = any("naturalist:duck" in row for row in command("data get entity @s Inventory"))
            return output
        finally:
            if socket.exists():
                control("quit")


def compare(output, reference):
    assert output["adult_laid_duck_egg"] == reference["adult_laid_duck_egg"], output
    assert output["duck_loot"], output
    for attribute in reference["attributes"]:
        assert abs(output["attributes"][attribute] - reference["attributes"][attribute]) < 1e-6
    for who in ("adult", "baby"):
        for key in reference["before"][who]:
            observed = output["before"][who][key]
            target = reference["before"][who][key]
            if key == "Health":
                assert float(observed.rstrip("f")) == float(target.rstrip("f")), (who, key, observed, target)
            else:
                assert observed == target, (who, key, observed, target)
        assert output["after"][who]["Age"] == reference["after"][who]["Age"], output
    assert output["after"]["baby"]["EggLayTime"] == reference["after"]["baby"]["EggLayTime"]
    assert 6000 <= int(output["after"]["adult"]["EggLayTime"]) <= 11999


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    observed = capture(args.version)
    print(json.dumps(observed, indent=2), flush=True)
    compare(observed, json.loads(args.fixture.read_text()))
