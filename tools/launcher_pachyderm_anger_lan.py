"""Verify hurt attribution and persistent anger for both real headless LAN roles."""
import argparse
import json
import re
from pathlib import Path
import subprocess
import tempfile

from launcher_pachyderm_anger_reopen import saved_entity


def capture(version, anvil_dump, nearer_bystander, herd):
    with tempfile.TemporaryDirectory(prefix="nat-pachyderm-anger-lan-", ignore_cleanup_errors=True) as work:
        root = Path(work)
        guest = "PachyGuest"

        def control(side, *args):
            call = subprocess.run([version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                                  check=True, capture_output=True, text=True, timeout=75)
            return json.loads(call.stdout.splitlines()[-1])

        def launch(side, *args):
            home = root / side
            home.mkdir()
            return control(side, "launch", "--headless", "--game", version / "horizon-client",
                           "--resources", version / ".local/horizon-client/26.3",
                           "--home", home, "--game-dir", home, "--render-distance", 4, *args)

        def command(line):
            return control("host", "command", line)["chat"]

        def field(target, name):
            prefix = " following entity data: "
            return next((row.split(prefix, 1)[1] for row in command(f"data get entity {target} {name}")
                         if prefix in row), None)

        def game_time():
            rows = command("time query gametime")
            return next(int(match[1]) for row in rows if (match := re.search(r"The game time is (\d+)", row)))

        def sample(label):
            end = field(label, "anger_end_time")
            return {"target": field(label, "angry_at"),
                    "end": int(end.rstrip("L")) if end else None,
                    "time": game_time(), "health": field(label, "Health"),
                    "entity_uuid": field(label, "UUID")}

        def step(ticks):
            command(f"tick step {ticks}")
            control("host", "wait", ticks + 2)
            control("guest", "wait", 1)

        try:
            launch("host", "--world", "Naturalist LAN Anger Verification", "--creative",
                   "--test-terrain", "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("gamerule minecraft:advance_time false")
            command("gamerule minecraft:universal_anger false")
            command("tp @s 8.5 101 5.5")
            control("host", "wait", 20)
            command("fill 1 100 1 15 100 15 minecraft:stone")
            command("fill 1 101 1 15 110 15 minecraft:air")
            port = control("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            control("guest", "connect", f"127.0.0.1:{port}", guest)
            control("guest", "wait-for", "loaded", "--timeout", 55)
            players = {"host": "@s", "guest": guest}
            uuids = {side: field(player, "UUID") for side, player in players.items()}
            assert len(set(uuids.values())) == 2 and all(uuids.values()), uuids
            command("time set midnight")
            command("tick freeze")
            cases = {}
            for species in ("elephant", "mammoth"):
                for hitter, bystander in (("host", "guest"), ("guest", "host")):
                    command("kill @e[tag=anger_lan_mob]")
                    if herd:
                        command("kill @e[tag=anger_lan_young]")
                    step(25)
                    for side, player in players.items():
                        command(f"gamemode survival {player}")
                        command(f"clear {player}")
                        command(f"effect clear {player}")
                        command(f"effect give {player} minecraft:resistance 40 4 true")
                        if herd:
                            x, z = (7.5, 6.5) if side == hitter else (13.8, 6.5)
                        else:
                            bystander_x = 10.1 if nearer_bystander else 11.1
                            x, z = (8.5, 6.5) if side == hitter else (bystander_x, 8.5)
                        command(f"tp {player} {x} 101 {z}")
                    label = "@e[tag=anger_lan_mob,limit=1]"
                    x_adult = 12.5 if herd else 8.5
                    command(f"summon naturalist:{species} {x_adult} 101 8.5 "
                            "{Age:0,OnGround:1b,Tags:[anger_lan_mob]}")
                    if herd:
                        target = "@e[tag=anger_lan_young,limit=1]"
                        command(f"summon naturalist:{species} 7.5 101 8.5 "
                                "{Age:-10000,OnGround:1b,Tags:[anger_lan_young]}")
                    step(8)
                    point = (7.5, 101.8, 8.5) if herd else (8.5, 102.45, 8.5)
                    control(hitter, "look-at", *point)
                    hit = control(hitter, "looking-at")
                    assert hit["kind"] == "entity", (species, hitter, hit)
                    fresh = sample(label)
                    victim_before = field(target, "Health") if herd else None
                    control(hitter, "mouse", "left", "click")
                    step(12)
                    active = sample(label)
                    victim_after = field(target, "Health") if herd else None
                    control("host", "save")
                    saved = sample(label)
                    disk = None
                    if anvil_dump and saved["target"]:
                        stamp = {"adult_uuid": saved["entity_uuid"], "anger_target": saved["target"],
                                 "anger_end": f'{saved["end"]}L'}
                        disk = saved_entity(root / "host" / "saves", anvil_dump, stamp, "anger_lan_mob")
                    command(f"gamemode spectator {players[hitter]}")
                    step(8)
                    calm = sample(label)
                    key = f"{species}_{hitter}"
                    cases[key] = {"roles": {"hitter": uuids[hitter], "bystander": uuids[bystander]},
                                  "fresh": fresh, "active": active, "saved": saved, "calm": calm,
                                  "disk_after_flush": disk}
                    if herd:
                        cases[key]["victim_health"] = [victim_before, victim_after]
                    print(key, json.dumps(cases[key], sort_keys=True), flush=True)
            return cases
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    control(side, "quit")


def compare(cases, reference):
    for name, case in cases.items():
        roles = case["roles"]
        fresh, active, saved, calm = (case[key] for key in ("fresh", "active", "saved", "calm"))
        if "victim_health" in case:
            before, after = (float(value.rstrip("f")) for value in case["victim_health"])
            assert after < before and active["health"] == fresh["health"], (name, case)
        else:
            assert float(active["health"].rstrip("f")) < float(fresh["health"].rstrip("f")), (name, case)
        assert fresh["target"] is None and fresh["end"] == -1, (name, case)
        assert active["target"] == saved["target"] == roles["hitter"], (name, case)
        assert roles["hitter"] != roles["bystander"], (name, case)
        assert active["end"] == saved["end"] and 390 <= active["end"] - active["time"] <= 780, (name, case)
        assert calm["target"] is None and calm["end"] == -1, (name, case)
        if reference:
            species, role = name.split("_")
            expected = reference[f'{species}_{"first" if role == "host" else "second"}']
            assert expected["applied"] and expected["attacker_match"] and expected["health_decreased"]
            assert expected["store_has_anger"] and expected["calm_target_absent"]
            assert fresh["end"] == expected["fresh_end"] and calm["end"] == expected["calm_end"], (name, case)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--reference", type=Path)
    parser.add_argument("--anvil-dump", type=Path)
    geometry = parser.add_mutually_exclusive_group()
    geometry.add_argument("--nearer-bystander", action="store_true")
    geometry.add_argument("--herd", action="store_true", help="Hit a calf and verify the adult's retaliation")
    args = parser.parse_args()
    result = capture(args.version, args.anvil_dump, args.nearer_bystander, args.herd)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    compare(result, json.loads(args.reference.read_text()) if args.reference else None)
