"""Compare Elephant and Mammoth survival interactions against pinned Java."""
import argparse
import json
import math
from pathlib import Path
import subprocess
import tempfile
from launcher_managed import launch_managed, wait_managed


def capture(version, expected, live_melee=False):
    with tempfile.TemporaryDirectory(prefix="nat-elephant-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        sock = directory / "ctl.sock"

        def call(*args):
            proc = subprocess.run([version / "horizon-ctl", "--socket", sock, *map(str, args)],
                                  capture_output=True, text=True, timeout=75)
            if proc.returncode:
                status = subprocess.run([version / "horizon-ctl", "--socket", sock, "status"],
                                        capture_output=True, text=True, timeout=10)
                raise RuntimeError(f"horizon-ctl {args!r} failed ({proc.returncode}): "
                                   f"stdout={proc.stdout!r}; stderr={proc.stderr!r}; "
                                   f"status={status.stdout!r}/{status.stderr!r}")
            return json.loads(proc.stdout.splitlines()[-1])

        def command(line):
            return call("command", line)["chat"]

        def field(entity, key):
            prefix = "following entity data: "
            return next((row.split(prefix, 1)[1] for row in command(f"data get entity {entity} {key}")
                         if prefix in row), None)

        def scalar(entity, key):
            raw = field(entity, key)
            return float(raw.rstrip("fdbls")) if raw else 0

        def item(id="air", count=1):
            command(f"item replace entity @s weapon.mainhand with minecraft:{id} {count}")
            call("wait", 3)

        def use(target):
            kind = target.split("type=", 1)[1].split(",", 1)[0]
            entities = call("entities", 8, kind)["entities"]
            entity = min(entities, key=lambda entry: entry["distance"]) if entities else None
            assert entity and entity.get("type") == kind, (target, entities)
            aimed = None
            for offset in (0.8, 1.2, 1.6, 0.4):
                call("look-at", entity["pos"][0], entity["pos"][1] + offset, entity["pos"][2])
                aimed = call("looking-at")
                if aimed.get("kind") == "entity" and aimed["entity"].get("id") == entity["id"]:
                    break
            assert aimed and aimed.get("kind") == "entity" and aimed["entity"].get("id") == entity["id"], aimed
            call("mouse", "right", "click")
            call("wait", 8)

        def combat(species):
            attacker = f"@e[type=naturalist:{species},tag=parity_attack,limit=1]"
            prey = "@e[type=minecraft:zombie,tag=parity_prey,limit=1]"
            command("tick freeze")
            command(f"summon naturalist:{species} 8.5 100 8.5 {{Tags:[parity_attack]}}")
            command("summon minecraft:zombie 9.5 100 8.5 "
                    "{NoAI:1b,Silent:1b,Tags:[parity_prey],equipment:{}}")
            call("wait", 3)
            if live_melee:
                command("time set midnight")
                damage = command(f"damage {attacker} 1 minecraft:mob_attack by {prey}")
                assert any("Applied 1.0 damage" in row for row in damage), damage
                output_health = scalar(prey, "Health")
                for _ in range(60):
                    command("tick step 1")
                    call("wait", 3)
                    output_health = scalar(prey, "Health")
                    if 0.0 < output_health < 17.0:
                        break
                result = {"live_revenge_target_health": output_health}
            else:
                prefix = "Elephant attack "
                lines = command(f"naturalist_fixture_elephant_attack naturalist:{species}")
                result = next((json.loads(line.split(prefix, 1)[1]) for line in lines if prefix in line), None)
                assert result, (species, lines)
            command(f"kill {attacker}")
            command(f"kill {prey}")
            command("tick unfreeze")
            return result

        play = None
        try:
            play = launch_managed(version, sock, directory, "Naturalist Elephant Comparison",
                                  creative=True, test_terrain=True)
            command("gamerule minecraft:spawn_mobs false")
            call("wait", 20)
            command("fill 1 99 1 15 99 15 minecraft:stone")
            command("fill 1 100 1 15 109 15 minecraft:air")
            command("tp @s 8.5 100 6.0")
            command("gamemode survival @s")
            owner = field("@s", "UUID")
            output = {}
            for species in expected:
                command("tp @s 8.5 100 6.5")
                target = f"@e[type=naturalist:{species},tag=parity_mount,limit=1]"
                rows = command(f"summon naturalist:{species} 8.5 100 9.5 "
                               "{Age:-24000,NoAI:1b,Tags:[parity_mount]}")
                assert any("Summoned" in row for row in rows), rows
                call("wait", 10)
                taming = []
                spent = []
                for _ in range(5):
                    item("cake")
                    use(target)
                    spent.append(field("@s", "SelectedItem.id") is None)
                    taming.append(field(target, "Owner") == owner)
                result = {"baby_taming": taming, "cake_spent_once": all(spent),
                          "taming_count_saved": scalar(target, "TamingFood")}
                command(f"data merge entity {target} {{Age:0}}")
                call("wait", 5)
                command(f"data merge entity {target} {{Health:52.0f}}")
                print(species, "loaded health before healing", scalar(target, "Health"), flush=True)

                item("melon_slice", 2)
                use(target)
                result["healed_to"] = scalar(target, "Health")
                result["melon_remaining"] = scalar("@s", "SelectedItem.count")
                item("chest")
                use(target)
                result["chested"] = field(target, "ChestedHorse") == "1b"
                item("saddle")
                use(target)
                result["saddled"] = field(target, "equipment") is not None and "minecraft:saddle" in field(target, "equipment")

                item()
                use(target)
                result["owner_mounted"] = call("status").get("riding") is not None
                result["seat_height"] = scalar("@s", "Pos[1]") - scalar(target, "Pos[1]")

                call("key", "shift", "down")
                call("wait", 4)
                call("key", "shift", "up")
                result["owner_dismounted"] = call("status").get("riding") is None
                command("tp @s 8.5 100 6.5")
                item("shears")
                print(species, "before shears", field(target, "Items"), field(target, "Pos"),
                      field("@s", "Pos"), flush=True)
                use(target)
                result["shear_chest_first"] = field(target, "ChestedHorse") == "0b" and "minecraft:saddle" in (field(target, "equipment") or "")
                command("tp @s 8.5 100 6.5")
                call("wait", 8)
                use(target)
                print(species, "after shears", field(target, "Items"), field(target, "equipment"),
                      field("@s", "SelectedItem"), flush=True)
                result["shear_saddle_second"] = "minecraft:saddle" not in (field(target, "equipment") or "")
                result["shear_damage"] = scalar("@s", 'SelectedItem.components."minecraft:damage"')
                command(f"kill {target}")
                result.update(combat(species))
                if not live_melee:
                    command("time set noon")
                    substrate = {}
                    for block in expected[species]["spawn_substrate"]:
                        command(f"setblock 12 129 12 minecraft:{block}")
                        call("wait", 4)
                        rows = command(f"naturalist_fixture_elephant_spawn naturalist:{species}")
                        marker = "Elephant spawn "
                        reply = next((row.split(marker, 1)[1] for row in rows if marker in row), None)
                        assert reply in ("true", "false"), (species, block, rows)
                        substrate[block] = reply == "true"
                    result["spawn_substrate"] = substrate

                output[species] = result
            return output
        finally:
            if sock.exists():
                call("quit")
            wait_managed(play)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    parser.add_argument("--live-melee", action="store_true")
    args = parser.parse_args()
    expected = json.loads(args.fixture.read_text())
    actual = capture(args.version, expected, args.live_melee)
    print(json.dumps(actual, indent=2), flush=True)
    for species, result in actual.items():
        java = expected[species]
        for key, value in result.items():
            if key == "seat_height":
                assert math.isclose(value, java["seat"][1], abs_tol=0.03), (species, key, value)
            elif key == "live_revenge_target_health":
                assert math.isclose(value, java["live_revenge"]["target_health"], abs_tol=0.001), (species, key, value)
            elif key == "owner_dismounted":
                assert value, (species, key, value)
            elif key == "target_health":
                assert math.isclose(value, java[key], abs_tol=0.001), (species, key, value, java[key])
            elif key == "target_motion":
                assert all(math.isclose(v, j, abs_tol=0.025) for v, j in zip(value, java[key])), (species, key, value, java[key])
            elif key == "cake_spent_once":
                assert value, (species, key, value)
            else:
                assert value == java[key], (species, key, value, java[key])
