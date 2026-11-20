"""Observe adult leaders and calves from ordinary headless runtime spawning."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-rhino-groups-", ignore_cleanup_errors=True) as tmp:
        home = Path(tmp)
        socket = home / "groups.sock"
        fixture = home / "spawn_fixture"
        pack = fixture / "data/naturalist_fixture/neoforge/biome_modifier"
        pack.mkdir(parents=True)
        (fixture / "horizon-mod.toml").write_text(
            'id = "naturalist_fixture"\nname = "Natural group fixture"\n'
            'version = "1.0.0"\napi = "0.2"\nside = "both"\ncapabilities = ["registries"]\n'
        )
        # A higher weight makes the natural-spawn sample fast. The authored
        # groups, placement checks, spawn attempts and age logic still run.
        (pack / "rhinos.json").write_text(json.dumps({
            "type": "neoforge:add_spawns", "biomes": "minecraft:plains",
            "spawners": {"type": "naturalist:rhino", "weight": 10000, "minCount": 3, "maxCount": 3},
        }))

        def ctl(*args):
            result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                    capture_output=True, text=True, timeout=90)
            if result.returncode:
                raise RuntimeError((args, result.stdout, result.stderr))
            return json.loads(result.stdout.splitlines()[-1])

        def cmd(line):
            result = ctl("command", line)
            if not result["ok"]:
                raise RuntimeError((line, result))
            return result["chat"]

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--world", "Naturalist Herd Fixture", "--seed", 1729,
                "--creative", "--test-terrain", "--render-distance", 5,
                "--dev-mod", fixture, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:spawn_monsters false")
            cmd("tp @s 0.5 101 0.5")
            ctl("wait", 30)
            filled = cmd("fill -56 100 -56 56 100 56 minecraft:grass_block")
            assert any("12769" in row for row in filled), filled
            cmd("time set noon")
            cmd("gamerule minecraft:spawn_mobs true")
            herd = []
            for _ in range(24):
                ctl("wait", 80)
                herd = ctl("entities", 100, "naturalist:rhino")["entities"]
                if len(herd) >= 3:
                    break
            return sorted(herd, key=lambda mob: mob["id"])
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    args = parser.parse_args()
    reference = json.loads(args.fixture.read_text())
    assert reference["natural_first_adult"] and reference["natural_second_calf"] and reference["natural_third_calf"]
    observations = capture(args.version)
    print(json.dumps(observations, indent=2), flush=True)
    assert len(observations) >= 3 and not observations[0]["baby"], observations
    assert any(mob["baby"] for mob in observations[1:]), observations
