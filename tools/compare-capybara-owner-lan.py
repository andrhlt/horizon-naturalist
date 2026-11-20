"""Measure whether a headless Capybara follows its saved owner past a nearer guest."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-capybara-owner-lan-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        guest = "CapyGuest"

        def ctl(side, *args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                                   text=True, check=True, capture_output=True, timeout=75)
            return json.loads(reply.stdout.splitlines()[-1])

        def launch(side, *args):
            home = root / side
            home.mkdir()
            ctl(side, "launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", home,
                "--game-dir", home, "--render-distance", 4, *args)

        def command(line):
            return ctl("host", "command", line)["chat"]

        def data(target, field):
            marker = "following entity data: "
            return next((line.split(marker, 1)[1] for line in command(f"data get entity {target} {field}")
                         if marker in line), None)

        def step(ticks):
            command(f"tick step {ticks}")
            ctl("host", "wait", ticks + 2)

        def position(target):
            return [float(value.rstrip("d")) for value in data(target, "Pos")[1:-1].split(", ")]

        try:
            launch("host", "--world", "Capybara Owner Identity", "--creative", "--test-terrain", "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("gamerule minecraft:advance_time false")
            command("forceload add 0 0 31 31")
            command("tp @s 18.5 101 8.5")
            ctl("host", "wait", 20)
            command("fill 1 100 1 29 100 15 minecraft:stone")
            command("fill 1 101 1 29 106 15 minecraft:air")
            port = ctl("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            ctl("guest", "connect", f"127.0.0.1:{port}", guest)
            ctl("guest", "wait-for", "loaded", "--timeout", 55)
            host_uuid, guest_uuid = data("@s", "UUID"), data(guest, "UUID")
            assert host_uuid and guest_uuid and host_uuid != guest_uuid
            command("time set midnight")
            command("tick freeze")
            result = {}
            cases = (
                ("owner_only", True, False, False, False),
                ("near_bystander", True, True, False, False),
                ("disabled", False, True, False, False),
                ("ordered_sit", True, True, True, False),
                ("bystander_melon", True, True, False, True),
            )
            for owner, other, owner_uuid, role in (("@s", guest, host_uuid, "host"), (guest, "@s", guest_uuid, "guest")):
                for condition, following, near, sitting, melon in cases:
                    target = "@e[tag=capy_owner_probe,limit=1]"
                    command(f"kill {target}")
                    step(25)
                    command(f"gamemode survival {owner}")
                    command(f"gamemode {'survival' if near else 'spectator'} {other}")
                    command(f"tp {owner} 17.0 101 8.5")
                    command(f"tp {other} 7.5 101 4.5")
                    command(f"clear {owner}")
                    command(f"clear {other}")
                    if melon:
                        command(f"item replace entity {other} weapon.mainhand with minecraft:melon_slice")
                    command(f"summon naturalist:capybara 6.5 101 8.5 "
                            f"{{Age:0,Owner:{owner_uuid},FollowingOwner:{int(following)}b,Sitting:{int(sitting)}b,"
                            "OnGround:1b,Tags:[capy_owner_probe]}")
                    assert data(target, "Owner") == owner_uuid
                    assert data(target, "FollowingOwner") == f"{int(following)}b"
                    points = [[0, *position(target)]]
                    goal_samples = []
                    for time in range(1, 25):
                        step(1)
                        rows = [m for m in ctl("host", "mobs", 32)["near"] if m["kind"] == "naturalist:capybara"]
                        assert len(rows) == 1, rows
                        goal_samples.append(rows[0]["goals"])
                        if time in (4, 8, 12, 24):
                            points.append([time, *position(target)])
                    dx = points[-1][1] - points[0][1]
                    dz = points[-1][3] - points[0][3]
                    result[f"{role}_{condition}"] = {
                        "points": points, "goals": goal_samples, "toward_owner": dx > 0.5 and abs(dz) < 0.75,
                        "toward_bystander": dz < -0.5,
                    }
                    print(role, condition, result[f"{role}_{condition}"], flush=True)
            return result
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    ctl(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--reference", type=Path)
    args = parser.parse_args()
    result = capture(args.version)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    if args.reference:
        reference = json.loads(args.reference.read_text())
        assert set(result) == set(reference)
        for name, value in result.items():
            gold = reference[name]
            if not name.endswith("disabled"):
                assert value["toward_owner"] == gold["toward_owner"], (name, value, gold)
            goals = value["goals"]
            active = lambda goal: any(goal in sample for sample in goals)
            if name.endswith(("owner_only", "near_bystander")):
                assert active("FollowPetOwner"), (name, goals)
            else:
                assert not active("FollowPetOwner"), (name, goals)
            if name.endswith("ordered_sit"):
                assert active("SitWhenOrdered"), (name, goals)
            if name.endswith("bystander_melon"):
                assert active("Tempt"), (name, goals)
            if not name.endswith("disabled"):
                assert value["toward_bystander"] == gold["toward_bystander"], (name, value, gold)
            if gold["toward_owner"] or gold["toward_bystander"]:
                positive = tuple(value["points"][1][axis] for axis in (1, 3))
                expected = tuple(gold["points"][1][axis] for axis in (1, 3))
                assert all(abs(a-b) < 1e-7 for a,b in zip(positive, expected, strict=True)), (name, value, gold)
