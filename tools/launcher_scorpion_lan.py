"""Compare active Scorpion venom and real shield blocks between LAN clients."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-lan-", ignore_cleanup_errors=True) as work:
        root = Path(work)

        def call(side, *args):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", root / f"{side}.sock", *map(str, args)],
                check=True, capture_output=True, text=True, timeout=65)
            return json.loads(reply.stdout.splitlines()[-1])

        def launch(side, *options):
            home = root / side
            home.mkdir()
            call(side, "launch", "--headless", "--game", version / "horizon-client",
                 "--resources", version / ".local/horizon-client/26.3", "--home", home,
                 "--game-dir", home, "--render-distance", 4, *options)

        def command(text):
            return call("host", "command", text)["chat"]

        def poisoned(side):
            return next((effect for effect in call(side, "state")["player"]["effects"]
                         if effect["id"] == "poison"), None)

        try:
            launch("host", "--world", "Naturalist LAN Scorpions", "--creative",
                   "--test-terrain", "--wait")
            command("gamerule minecraft:spawn_mobs false")
            command("gamerule minecraft:advance_time false")
            command("time set midnight")
            command("difficulty normal")
            command("tp @s 8.5 101 4.5")
            command("forceload add 0 0")
            call("host", "wait", 25)
            command("fill 0 100 0 15 100 15 minecraft:grass_block")
            command("fill 0 101 0 15 106 15 minecraft:air")
            port = call("host", "lan", "open", 0, "commands")["port"]
            launch("guest")
            call("guest", "connect", f"127.0.0.1:{port}", "ScorpionGuest")
            call("guest", "wait-for", "loaded", "--timeout", 55)
            command("tp ScorpionGuest 8.5 101 5.5")
            command("gamemode survival ScorpionGuest")
            command("effect give ScorpionGuest minecraft:resistance infinite 3 true")
            result = {}
            for species in ("desert_scorpion", "jungle_scorpion"):
                sample = {}
                for blocking in (False, True):
                    command("effect clear ScorpionGuest minecraft:poison")
                    command("effect give ScorpionGuest minecraft:instant_health 1 4 true")
                    command("tp ScorpionGuest 8.5 101 5.5")
                    call("guest", "look-at", 8.5, 101.7, 8.5)
                    if blocking:
                        command("item replace entity ScorpionGuest weapon.mainhand with minecraft:shield")
                        call("guest", "mouse", "right", "down")
                        call("guest", "wait", 10)
                        assert call("guest", "state")["using"] is not None
                    shield = "minecraft:item.shield.block"
                    previous = call("guest", "status")["heard"].get(shield, 0)
                    start = call("guest", "state")["player"]["health"]
                    target = f"@e[type=naturalist:{species},tag=net_venom,limit=1]"
                    rows = command(f"summon naturalist:{species} 8.5 101 7.4 {{Tags:[net_venom],PersistenceRequired:1b}}")
                    assert any("Summoned" in row for row in rows), rows
                    effect = None
                    blocked = False
                    for _ in range(30):
                        call("guest", "wait", 4)
                        effect = poisoned("guest")
                        blocked = call("guest", "status")["heard"].get(shield, 0) > previous
                        if effect or blocked:
                            break
                    sample["shielded" if blocking else "ordinary"] = {
                        "poison": effect, "guest_shield_block": blocked,
                        "guest_lost_health": call("guest", "state")["player"]["health"] < start,
                        "host_poison": poisoned("host"),
                    }
                    command(f"kill {target}")
                    call("guest", "mouse", "right", "up")
                    command("item replace entity ScorpionGuest weapon.mainhand with minecraft:air")
                result[species] = sample
            return result
        finally:
            for side in ("guest", "host"):
                if (root / f"{side}.sock").exists():
                    call(side, "quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("fixture", type=Path)
    options = parser.parse_args()
    result = capture(options.version)
    expected = json.loads(options.fixture.read_text())
    print(json.dumps(result, indent=2), flush=True)
    for species, sample in result.items():
        ordinary, shielded = sample["ordinary"], sample["shielded"]
        assert ordinary["guest_lost_health"] == expected[species]["attack_landed"]
        assert ordinary["poison"]["amplifier"] == expected[species]["poison_amplifier"]
        assert 111 <= ordinary["poison"]["duration_ticks"] <= expected[species]["poison_duration"]
        assert not ordinary["guest_shield_block"]
        assert shielded["guest_shield_block"] == (not expected[species]["shielded_hit_landed"])
        assert shielded["guest_lost_health"] == (not expected[species]["shielded_health_unchanged"])
        assert bool(shielded["poison"]) == expected[species]["shielded_poisoned"]
        assert ordinary["host_poison"] is None and shielded["host_poison"] is None
