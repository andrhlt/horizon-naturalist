"""Check pairing and removal of the packaged wildlife on two headless clients."""
import argparse
import json
from math import dist
from pathlib import Path
import subprocess
import tempfile
import time


def capture(version, reload=False, upgrade_from=None):
    target_version = version
    registry = version / "native-content/naturalist/data/naturalist/horizon/entity_type"
    roster = sorted(f"naturalist:{file.stem}" for file in registry.glob("*.json")
                    if file.stem not in ("duck_egg", "lizard_tail", "dirt_trail", "carried_food"))
    assert roster, registry
    if upgrade_from:
        old_registry = upgrade_from / "native-content/naturalist/data/naturalist/horizon/entity_type"
        old_roster = sorted(f"naturalist:{file.stem}" for file in old_registry.glob("*.json")
                            if file.stem not in ("duck_egg", "lizard_tail", "dirt_trail", "carried_food"))
        assert len(roster) == 47 and old_roster == roster, (old_roster, roster)
        version = upgrade_from
    with tempfile.TemporaryDirectory(prefix="nat-lan-roster-", ignore_cleanup_errors=True) as work:
        root = Path(work)

        def call(side, *args):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                check=True, capture_output=True, text=True, timeout=65)
            return json.loads(reply.stdout.splitlines()[-1])

        def launch(side, *options):
            home = root / side
            home.mkdir(exist_ok=True)
            call(side, "launch", "--headless", "--game", version / "horizon-client",
                 "--resources", version / ".local/horizon-client/26.3", "--home", home,
                 "--game-dir", home, "--render-distance", 4, *options)

        def command(text):
            return call("host", "command", text)["chat"]

        def present():
            return {entity["id"] for entity in call("guest", "entities", 30)["entities"]}

        def fixture_ids():
            positions = {}
            marker = " following entity data: "
            for species in roster:
                rows = command(f"data get entity @e[type={species},tag=lan_roster,limit=1] Pos")
                value = next((line.partition(marker)[2] for line in rows if marker in line), None)
                assert value is not None, (species, rows)
                positions[species] = tuple(float(axis.strip().rstrip("d")) for axis in value[1:-1].split(","))
            found = {}
            for entity in call("host", "entities", 42)["entities"]:
                species = entity["type"]
                if species in positions and dist(positions[species], entity["pos"]) < 0.01:
                    assert species not in found, (species, entity, found)
                    found[species] = entity["id"]
            assert found.keys() == positions.keys(), sorted(positions.keys() - found.keys())
            return {identifier: species for species, identifier in found.items()}

        def fixture_uuids():
            result = {}
            marker = " following entity data: "
            for species in roster:
                rows = command(f"data get entity @e[type={species},tag=lan_roster,limit=1] UUID")
                value = next((line.partition(marker)[2] for line in rows if marker in line), None)
                assert value is not None, (species, rows)
                result[species] = value
            assert len(set(result.values())) == len(roster), result
            return result

        try:
            host_options = ("--world", "Naturalist LAN Roster", "--creative", "--test-terrain", "--wait")
            launch("host", *host_options)
            command("gamerule minecraft:spawn_mobs false")
            command("gamerule minecraft:advance_time false")
            command("time set midnight")
            command("tp @s 5.5 102 5.5")
            call("host", "wait", 25)
            command("fill 0 100 0 15 100 15 minecraft:stone")
            command("fill 0 101 0 15 108 15 minecraft:air")
            for i, species in enumerate(roster):
                x, z = 3.5 + 2 * (i % 5), 3.5 + 2 * (i // 5)
                rows = command(f"summon {species} {x} 101 {z} "
                               "{NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[lan_roster]}")
                assert any("Summoned" in row for row in rows), (species, rows)

            before_uuids = fixture_uuids() if upgrade_from else None
            if reload or upgrade_from:
                command("save-all flush")
                call("host", "quit")
                deadline = time.monotonic() + 10.0
                while (root / "host.sock").exists() and time.monotonic() < deadline:
                    time.sleep(0.1)
                version = target_version
                launch("host", *host_options)
            after_uuids = fixture_uuids() if upgrade_from else None
            if upgrade_from:
                assert before_uuids == after_uuids, (before_uuids, after_uuids)
            expected = fixture_ids()
            port = call("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            call("guest", "connect", f"127.0.0.1:{port}", "NaturalistRoster")
            call("guest", "wait-for", "loaded", "--timeout", 55)
            command("tp NaturalistRoster 5.5 101 5.5")
            for _ in range(10):
                call("host", "wait", 10)
                if expected.keys() <= present():
                    break
            else:
                raise AssertionError(("missing on late join", sorted(expected[identifier] for identifier in expected.keys() - present())))

            removal = command("kill @e[tag=lan_roster]")
            assert f"Killed {len(roster)} entities" in removal, removal
            for _ in range(10):
                call("host", "wait", 10)
                if not (expected.keys() & present()):
                    break
            else:
                raise AssertionError(("still paired after removal", sorted(expected[identifier] for identifier in expected.keys() & present())))

            result = {"reloaded_world": reload or bool(upgrade_from), "late_join": roster, "removed_from_guest": len(roster)}
            if upgrade_from:
                result.update({"upgrade_from": str(upgrade_from), "upgrade_to": str(target_version),
                               "uuids_before": before_uuids, "uuids_after": after_uuids})
            return result
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    call(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("--reload", action="store_true")
    parser.add_argument("--upgrade-from", type=Path)
    args = parser.parse_args()
    print(json.dumps(capture(args.version, args.reload, args.upgrade_from), indent=2), flush=True)
