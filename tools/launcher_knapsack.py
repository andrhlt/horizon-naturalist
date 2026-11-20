"""Exercise the Java Knapsack's real main-hand and offhand capture/release in Horizon."""
import argparse
import json
import math
from pathlib import Path
import re
import socket as ipc
import subprocess
import tempfile
import time


def capture(version, reference, baseline=False, lan=False, metadata=False, restart=False):
    with tempfile.TemporaryDirectory(prefix="nat-knapsack-", ignore_cleanup_errors=True) as folder:
        home = Path(folder)
        host_socket = home / "host.sock"
        guest_socket = home / "guest.sock"
        socket = guest_socket if lan else host_socket

        def ctl(*args, via=None):
            proc = subprocess.run([version / "horizon-ctl", "--socket", via or socket, *map(str, args)],
                                  capture_output=True, text=True, timeout=75)
            assert proc.returncode == 0, (args, proc.stdout, proc.stderr)
            return json.loads(proc.stdout.splitlines()[-1])

        def cmd(line):
            line = line.replace("@s", "KnapsackGuest") if lan else line
            return ctl("command", line, via=host_socket)["chat"]

        def data(target, path):
            prefix = "following entity data: "
            rows = cmd(f"data get entity {target} {path}")
            return next((row.split(prefix, 1)[1] for row in rows if prefix in row), None)

        def target(tag):
            return f"@e[tag={tag},limit=1]"

        def exists(tag):
            return data(target(tag), "UUID") is not None

        def teleport(x, y, z):
            cmd(f"tp @s {x} {y} {z}")
            ctl("wait", 5)

        def step(ticks):
            cmd(f"tick step {ticks}")
            ctl("wait", ticks + 2, via=host_socket)

        def launch(profile, profile_socket, *args):
            profile.mkdir(exist_ok=True)
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", profile, "--game-dir", profile,
                "--render-distance", 4, *args, via=profile_socket)

        def click_mob(x, y, z, kind):
            ctl("look-at", x, y, z)
            hit = ctl("looking-at")
            assert hit["kind"] == "entity" and hit["entity"]["type"] == kind, hit
            ctl("mouse", "right", "click")
            ctl("wait", 3)

        def click_release():
            teleport(11.5, 102, 8.5)
            ctl("look-at", "block", 9, 102, 8, "east")
            hit = ctl("looking-at")
            assert hit["kind"] == "block" and hit["block"]["face"] == "east", hit
            ctl("mouse", "right", "click")
            ctl("wait", 3)

        def nums(snbt):
            return [float(num) for num in re.findall(r"-?[0-9]+(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?", snbt)]

        def inspect(offhand=False):
            suffix = "_offhand" if offhand else ""
            rows = ctl("command", f"naturalist_fixture_knapsack{suffix}")["chat"]
            marker = "Knapsack fixture "
            return next(json.loads(row.split(marker, 1)[1]) for row in rows if marker in row)

        def expect_sound(previous, current, event, at):
            if "sound_events" not in current:
                return
            events = current["sound_events"][len(previous["sound_events"]):]
            assert len(events) == 1, events
            sound = events[0]
            assert sound["event"] == f"naturalist:item.knapsack.{event}" and sound["category"] == 6, sound
            assert math.isclose(sound["volume"], 0.6, abs_tol=1e-6) and sound["pitch"] == 1.0, sound
            assert sound["at"] == at, (sound, at)
            print("Actual native sound dispatch", sound, flush=True)

        sack = "naturalist:knapsack"
        custom = 'SelectedItem.components."minecraft:custom_data"'
        try:
            launch(home / "host", host_socket, "--world", "Naturalist Knapsack Probe",
                   "--creative", "--test-terrain", "--wait")
            if baseline:
                rows = cmd(f"give @s {sack}")
                assert any("Unknown item" in row for row in rows), rows
                print(rows)
                return
            if lan:
                port = ctl("lan", "open", 0, "commands", via=host_socket)["port"]
                launch(home / "guest", guest_socket)
                ctl("connect", f"127.0.0.1:{port}", "KnapsackGuest")
                ctl("wait-for", "loaded", "--timeout", 55)
            cmd("gamemode creative @s")
            cmd("gamerule minecraft:fall_damage false")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:advance_time false")
            cmd("time set midnight")
            teleport(8.5, 102, 6.5)
            ctl("wait", 25)
            cmd("fill 2 101 2 15 101 15 minecraft:stone")
            cmd("fill 2 102 2 15 109 15 minecraft:air")
            cmd("setblock 9 102 8 minecraft:stone")
            teleport(8.5, 102, 6.5)
            cmd("gamemode survival @s")
            cmd("tick freeze")
            cmd("clear @s")

            cmd('summon naturalist:deer 8.5 102 8.5 {NoAI:1b,Age:-1234,Health:5f,'
                'CustomName:{text:"Twig",color:"red"},Rotation:[47f,12f],Tags:[knapsack_deer]}')
            name = target("knapsack_deer")
            uuid = data(name, "UUID")
            custom_name = data(name, "CustomName")
            assert uuid and custom_name and "red" in custom_name, (uuid, custom_name)
            cmd(f"item replace entity @s weapon.mainhand with {sack}")
            assert data("@s", "SelectedItem.id") == f'"{sack}"'
            assert data("@s", f"{custom}.id") is None
            if metadata:
                empty = inspect()
                assert not empty["filled_model"] and empty["tooltip"] == [], empty
            click_mob(8.5, 102.45, 8.5, "naturalist:deer")
            captured = [data("@s", f"{custom}.id"), int(data("@s", f"{custom}.Age") or 0),
                        float((data("@s", f"{custom}.Health") or "0").rstrip("f"))]
            expected = reference["capture"]
            assert captured == [f'"{expected[3]}"', expected[4], expected[5]], captured
            assert data("@s", f"{custom}.UUID") == uuid
            if metadata:
                filled = inspect()
                labels = [f"{name}|{color & 0xFFFFFF:06x}" for name, color in filled["tooltip"]]
                assert filled["filled_model"] and labels == reference["named_tooltip"], filled
                expect_sound(empty, filled, "pickup", [8.5, 102.0, 6.5])
                print("Filled model and gray custom label", filled, flush=True)
            step(1)
            assert not exists("knapsack_deer")
            print("Captured named baby Deer with exact age, health and UUID", flush=True)

            if restart:
                cmd("save-all flush")
                closed = host_socket
                ctl("quit", via=closed)
                deadline = time.monotonic() + 15
                while time.monotonic() < deadline:
                    with ipc.socket(ipc.AF_UNIX) as probe:
                        try:
                            probe.connect(str(closed))
                        except (FileNotFoundError, ConnectionRefusedError):
                            break
                    time.sleep(0.05)
                host_socket = home / "reopen.sock"
                socket = host_socket
                launch(home / "host", host_socket, "--world", "Naturalist Knapsack Probe", "--wait")
                cmd("tick freeze")
                assert data("@s", f"{custom}.UUID") == uuid
                assert int(data("@s", f"{custom}.Age")) == expected[4]
                assert not exists("knapsack_deer")
                print("Saved, reopened, and retained the exact captured item without its old live mob", flush=True)

            cmd("summon minecraft:cow 8.5 102 8.5 {NoAI:1b,Age:-1234,Tags:[knapsack_test_cow]}")
            click_mob(8.5, 102.55, 8.5, "minecraft:cow")
            assert exists("knapsack_test_cow") and data("@s", f"{custom}.UUID") == uuid
            click_release()
            expect = reference["release"]
            assert data(name, "UUID") == uuid
            assert nums(data(name, "Pos")) == expect[2:5]
            assert int(data(name, "Age")) == expect[5]
            assert float(data(name, "Health").rstrip("f")) == expect[6]
            released_name = data(name, "CustomName")
            assert expect[7] in released_name and released_name == custom_name, (custom_name, released_name)
            assert nums(data(name, "Rotation")) == expect[8:10]
            assert (data(name, "PersistenceRequired") == "1b") == expect[10]
            assert data("@s", "SelectedItem.id") == f'"{sack}"' and data("@s", f"{custom}.id") is None
            if metadata:
                released = inspect()
                assert not released["filled_model"] and released["tooltip"] == [], released
                expect_sound(filled, released, "place", [10.5, 102.5, 8.5])
                print("Empty model restored", released, flush=True)
            print("Clicked east face released the marked Deer with all measured Java state", flush=True)
            cmd(f"kill {name}")
            cmd("kill @e[tag=knapsack_test_cow]")
            step(25)

            teleport(8.5, 102, 6.5)
            cmd("summon naturalist:deer 8.5 102 8.5 {NoAI:1b,Tags:[knapsack_adult]}")
            click_mob(8.5, 102.75, 8.5, "naturalist:deer")
            assert exists("knapsack_adult") and data("@s", f"{custom}.id") is None
            cmd("kill @e[tag=knapsack_adult]")
            step(25)
            cmd("summon minecraft:zombie 8.5 102 8.5 {NoAI:1b,IsBaby:1b,Tags:[knapsack_zombie]}")
            zombie = target("knapsack_zombie")
            zombie_uuid = data(zombie, "UUID")
            cmd("item replace entity @s weapon.mainhand with minecraft:air")
            cmd(f"item replace entity @s weapon.offhand with {sack}")
            off = 'equipment.offhand.components."minecraft:custom_data"'
            click_mob(8.5, 102.57, 8.5, "minecraft:zombie")
            assert data("@s", f"{off}.id") == '"minecraft:zombie"', {
                "off_path": data("@s", off),
                "whole_inventory": cmd("data get entity @s Inventory"),
                "equipment": cmd("data get entity @s equipment"),
                "client": ctl("inventory").get("offhand"),
                "zombie": cmd(f"data get entity {zombie}"),
            }
            assert data("@s", f"{off}.UUID") == zombie_uuid
            if metadata and "sound_events" in empty:
                unnamed = inspect(offhand=True)
                labels = [f"{name}|{color & 0xFFFFFF:06x}" for name, color in unnamed["tooltip"]]
                assert unnamed["filled_model"] and labels == reference["unnamed_tooltip"], unnamed
                print("Filled model and exact unnamed Zombie gray caption", flush=True)
            step(1)
            assert not exists("knapsack_zombie")
            click_release()
            assert data(zombie, "UUID") == zombie_uuid and data(zombie, "IsBaby") == "1b"
            assert data("@s", f"{off}.id") is None
            print("Offhand round-tripped baby Zombie and adult Deer was rejected", flush=True)
            cmd("kill @e[tag=knapsack_zombie]")
            step(25)

            cmd(f'item replace entity @s weapon.mainhand with {sack}[minecraft:custom_data={{id:"naturalist:does_not_exist"}}]')
            click_release()
            assert data("@s", "SelectedItem.id") == f'"{sack}"' and data("@s", f"{custom}.id") is None
            print("Unknown released ID cleared the sack as in Java", flush=True)

            teleport(8.5, 102, 6.5)
            cmd('summon minecraft:pig 8.5 102 8.5 {NoAI:1b,Age:-1234,Tags:[knapsack_vehicle],'
                'Passengers:[{id:"minecraft:chicken",NoAI:1b,Age:-1234,Tags:[knapsack_passenger]}]}')
            vehicle = target("knapsack_vehicle")
            passenger = target("knapsack_passenger")
            vehicle_uuid = data(vehicle, "UUID")
            passenger_uuid = data(passenger, "UUID")
            assert vehicle_uuid and passenger_uuid
            assert data(vehicle, "Passengers[0].UUID") == passenger_uuid
            step(1)
            cmd(f"item replace entity @s weapon.mainhand with {sack}")
            child_position = nums(data(passenger, "Pos"))
            click_mob(child_position[0], child_position[1] + 0.25, child_position[2], "minecraft:chicken")
            assert exists("knapsack_passenger") and data("@s", f"{custom}.id") is None
            click_mob(8.5, 102.18, 8.5, "minecraft:pig")
            assert data("@s", f"{custom}.Passengers[0].UUID") == passenger_uuid
            step(1)
            assert not exists("knapsack_vehicle") and data(passenger, "UUID") == passenger_uuid
            detached = data(passenger, "Pos")
            click_release()
            assert data(vehicle, "UUID") == vehicle_uuid
            assert data(vehicle, "Passengers[0].UUID") == passenger_uuid
            assert nums(data(vehicle, "Passengers[0].Pos")) == reference["release"][2:5]
            assert data(passenger, "Pos") == detached
            num = cmd("execute if entity @e[tag=knapsack_passenger]")
            assert any("count: 1" in row.lower() or "found 1" in row.lower() for row in num), num
            print("Baby vehicle saved the passenger without moving or duplicating the real rider", flush=True)

            cmd("kill @e[tag=knapsack_passenger]")
            cmd("kill @e[tag=knapsack_vehicle]")
            step(25)
            teleport(8.5, 102, 6.5)
            cmd("clear @s")
            cmd("summon minecraft:cow 8.5 102 8.5 {NoAI:1b,Age:-1234,Tags:[knapsack_overstack_target]}")
            cmd(f"item replace entity @s weapon.mainhand with {sack}[minecraft:max_stack_size=2] 2")
            assert data("@s", "SelectedItem.count") == "2"
            click_mob(8.5, 102.45, 8.5, "minecraft:cow")
            assert data("@s", "SelectedItem.count") == "1" and data("@s", f"{custom}.id") is None
            filled = data("@s", 'Inventory[{id:"naturalist:knapsack",components:{"minecraft:custom_data":{id:"minecraft:cow"}}}].count')
            assert filled == "1", (filled, cmd("data get entity @s Inventory"))
            print("Oversized stack split one filled Knapsack into inventory like Java", flush=True)

            if "overstack_release" in reference:
                cmd("clear @s")
                overfilled = f'{sack}[minecraft:max_stack_size=2,minecraft:custom_data={{id:"minecraft:cow",'
                overfilled += 'Age:-1234,NoAI:1b,Tags:[knapsack_oversized_release]}]'
                cmd(f"item replace entity @s weapon.mainhand with {overfilled} 2")
                assert data("@s", "SelectedItem.count") == "2"
                click_release()
                retained_count = int(data("@s", "SelectedItem.count") or 0)
                held_id = data("@s", f"{custom}.id")
                inventory = cmd("data get entity @s Inventory")
                replacement = data("@s", 'Inventory[{Slot:1b,id:"naturalist:knapsack"}]')
                empty_count = 1 if replacement and "minecraft:custom_data" not in replacement else 0
                retained = [exists("knapsack_oversized_release"), retained_count, held_id == '"minecraft:cow"', empty_count]
                assert retained == reference["overstack_release"], (retained, replacement, inventory)
                print("Oversized filled stock kept one captured copy and inserted one empty sack", flush=True)

        finally:
            for profile in (guest_socket, host_socket):
                if profile.exists():
                    ctl("quit", via=profile)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    parser.add_argument("--baseline", action="store_true")
    parser.add_argument("--lan", action="store_true")
    parser.add_argument("--metadata", action="store_true")
    parser.add_argument("--restart", action="store_true")
    args = parser.parse_args()
    if args.restart and (args.lan or args.metadata or args.baseline):
        parser.error("--restart exercises a standalone world without diagnostics or LAN")
    capture(args.version, json.loads(args.reference.read_text()), args.baseline, args.lan, args.metadata, args.restart)
