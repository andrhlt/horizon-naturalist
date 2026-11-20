"""Compare actual loaded Naturalist mob profiles against the pinned Java registry."""
import argparse
import json
import math
from pathlib import Path
import re
import subprocess
import tempfile


def capture(version, reference, verify_scale=False):
    registry = version / "native-content/naturalist/data/naturalist/horizon/entity_type"
    kinds = sorted(f"naturalist:{file.stem}" for file in registry.glob("*.json")
                   if file.stem not in ("duck_egg", "dirt_trail"))
    assert kinds and all(kind in reference for kind in kinds), kinds
    with tempfile.TemporaryDirectory(prefix="nat-mob-profiles-", ignore_cleanup_errors=True) as work:
        home = Path(work)
        socket = home / "profile.sock"

        def call(*args):
            proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                  capture_output=True, text=True, check=True, timeout=65)
            return json.loads(proc.stdout.splitlines()[-1])

        def command(line):
            return call("command", line)["chat"]

        try:
            call("launch", "--headless", "--game", version / "horizon-client",
                 "--resources", version / ".local/horizon-client/26.3", "--home", home,
                 "--game-dir", home, "--world", "Naturalist Mob Profiles",
                 "--creative", "--test-terrain", "--render-distance", 4, "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("tp @s 8.5 101 8.5")
            call("wait", 25)
            command("tick freeze")
            output = {}
            for i, kind in enumerate(kinds):
                x, z = 4 + i % 7, 4 + i // 7
                rows = command(f"summon {kind} {x} 102 {z} "
                               "{Age:0,NoAI:1b,NoGravity:1b,PersistenceRequired:1b}")
                assert any("Summoned" in row for row in rows), (kind, rows)
                rows = command(f"naturalist_fixture_mob_profile {kind}")
                marker = "Naturalist mob profile "
                profile = next(json.loads(row.split(marker, 1)[1])
                               for row in rows if marker + "{" in row)
                target = f"@e[type={kind},limit=1]"
                for attribute in ("attack_knockback", "scale", "step_height"):
                    responses = command(f"attribute {target} minecraft:{attribute} base get")
                    profile["attributes"][attribute] = next(
                        (float(match[1]) for row in responses
                         if (match := re.search(r" is ([-\d.eE+]+)", row))), None)
                output[kind] = profile
            if verify_scale:
                for kind in ("naturalist:deer", "naturalist:boar", "naturalist:starfish"):
                    target = f"@e[type={kind},limit=1]"
                    command(f"attribute {target} minecraft:scale base set 2")
                    command(f"attribute {target} minecraft:scale modifier add naturalist:profile_scale 0.5 add_multiplied_total")
                    command(f"data merge entity {target} {{Silent:1b}}")
                    rows = command(f"naturalist_fixture_mob_profile {kind}")
                    scaled = next(json.loads(row.split(marker, 1)[1])
                                  for row in rows if marker + "{" in row)
                    expected = [dimension * 3 for dimension in reference[kind]["dimensions"]]
                    assert all(math.isclose(a, b, abs_tol=2e-6)
                               for a, b in zip(scaled["dimensions"], expected)), (kind, expected, scaled)
                    saved = command(f"data get entity {target} attributes")
                    assert any("naturalist:profile_scale" in row for row in saved), (kind, saved)
            return output
        finally:
            if socket.exists():
                call("quit")


def differences(expected, actual, parts=()):
    if isinstance(expected, dict) and isinstance(actual, dict):
        assert expected.keys() == actual.keys(), (parts, expected.keys(), actual.keys())
        for key, value in expected.items():
            yield from differences(value, actual[key], (*parts, key))
    elif isinstance(expected, list) and isinstance(actual, list):
        assert len(expected) == len(actual), (parts, expected, actual)
        for index, (left, right) in enumerate(zip(expected, actual)):
            yield from differences(left, right, (*parts, str(index)))
    elif isinstance(expected, (int, float)) and isinstance(actual, (int, float)):
        if not math.isclose(expected, actual, abs_tol=2e-6):
            yield {"field": ".".join(parts), "java": expected, "horizon": actual}
    elif expected != actual:
        yield {"field": ".".join(parts), "java": expected, "horizon": actual}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    parser.add_argument("--scale", action="store_true")
    parser.add_argument("--tortoise-command-profile", type=Path,
                        help="Pinned Java Tortoise profile after /summon with NBT")
    args = parser.parse_args()
    reference = json.loads(args.fixture.read_text())
    if args.tortoise_command_profile:
        command_profile = json.loads(args.tortoise_command_profile.read_text())
        reference["naturalist:tortoise"]["attributes"].update(command_profile["nbt"])
    actual = capture(args.version, reference, args.scale)
    changes = list(differences({kind: reference[kind] for kind in actual}, actual))
    print(json.dumps({"entities": len(actual), "discrepancies": changes, "profiles": actual}, indent=2))
    assert not changes, f"{len(changes)} deltas across {len(actual)} entities"
