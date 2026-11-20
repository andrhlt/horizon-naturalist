"""Replay Java Capture Net recipes by using the net against live Horizon mobs."""
import argparse
import json
from math import dist
from pathlib import Path
import re
import subprocess
import tempfile

def collect(game, actor, player, release):
    def command(line):
        return game.ctl("command", line)["chat"]

    def data(entity, path):
        return next((line.split("following entity data: ", 1)[1]
                     for line in command(f"data get entity {entity} {path}")
                     if "following entity data: " in line), "")

    command("gamerule minecraft:spawn_mobs false")
    command("gamerule minecraft:spawn_monsters false")
    command("gamerule minecraft:mob_drops false")
    command("tp @s 7.5 100 6.5")
    game.ctl("wait", 24)
    command("fill 0 99 0 15 99 15 minecraft:stone")
    command("fill 0 100 0 15 107 15 minecraft:air")

    slots = {2: "hotbar.2", 10: "inventory.1", 40: "weapon.offhand"}

    def run(kind, water_slots=(), *, offhand=False, creative=False, mounted=False):
        command("tick unfreeze")
        command("tp @e[tag=net_recipe] 13.5 105 13.5")
        command("tp @e[tag=net_fixture_pig] 13.5 105 13.5")
        command("kill @e[tag=net_recipe]")
        command("kill @e[tag=net_fixture_pig]")
        command("kill @e[type=minecraft:item]")
        command(f"clear {player}")
        actor.ctl("select", 1)
        command(f"tp {player} 7.5 100 6.5")
        command(f"gamemode {'creative' if creative else 'survival'} {player}")
        hand = "offhand" if offhand else "mainhand"
        command(f"item replace entity {player} weapon.{hand} with naturalist:capture_net")
        for slot in water_slots:
            assert any("Replaced" in line for line in command(
                f"item replace entity {player} {slots[slot]} with minecraft:water_bucket")), slot
        assert any("Summoned" in line for line in command(
            f"summon minecraft:{kind} 7.5 100 8.5 {{NoAI:1b,NoGravity:1b,Tags:[net_recipe]}}"))
        if mounted:
            command("summon minecraft:pig 7.5 100 8.5 {NoAI:1b,NoGravity:1b,Tags:[net_fixture_pig]}")
            riding = command("ride @e[tag=net_recipe,limit=1] mount @e[tag=net_fixture_pig,limit=1]")
            assert any("started riding" in message for message in riding), riding
        game.ctl("wait", 3)
        tag = "@e[tag=net_recipe,limit=1]"
        before = data(tag, "Pos")
        original = data(tag, "UUID")
        assert before and original, kind
        position = [float(axis.strip().rstrip("d")) for axis in before[1:-1].split(",")]
        kind_id = f"minecraft:{kind}"
        for _ in range(8):
            entities = actor.ctl("entities", 8)["entities"]
            candidates = [entity for entity in entities
                          if entity["type"] == kind_id and dist(position, entity["pos"]) < 0.75]
            if candidates:
                chosen = min(candidates, key=lambda entity: dist(position, entity["pos"]))
                try:
                    hit = actor.ctl("look-at", "entity", chosen["id"])["target"]
                except subprocess.CalledProcessError:
                    continue
                if hit["kind"] == "entity" and hit["entity"]["id"] == chosen["id"]:
                    break
            game.ctl("wait", 3)
        else:
            raise AssertionError((kind, position, entities, actor.ctl("looking-at")))
        command("tick freeze")
        actor.ctl("mouse", "right", "click")
        game.ctl("wait", 5)
        inventory = data(player, "Inventory")
        offhand_stack = actor.ctl("status")["offhand"] or {}
        mid = re.search(r'id: "(naturalist:(?:bee|silverfish|endermite))"', inventory)
        expected_id = f'minecraft:{kind}'
        result = {"removed": not data("@e[tag=net_recipe,limit=1]", "UUID"),
                  "net_damage": offhand_stack.get("damage", 0) if offhand else
                    int(re.search(r'"minecraft:damage": (\d+)', inventory)[1])
                    if '"minecraft:damage"' in inventory else 0,
                  "water_slots": [java for java in (2, 10) if data(
                      player, f'Inventory[{{Slot:{java}b}}].id') == '"minecraft:water_bucket"'],
                  "dropped": [],
                  "caught": mid[1] if mid else "",
                  "original_uuid": (
                      bool(mid) and original in inventory and f'id: "{expected_id}"' in inventory
                  )}
        if offhand_stack.get("item") == "minecraft:water_bucket":
            result["water_slots"].append(40)
        stack = data("@e[type=minecraft:item,limit=1]", "Item.id")
        if stack:
            result["dropped"].append(json.loads(stack))
        if release and result["caught"]:
            command("tick unfreeze")
            game.ctl("wait", 5)
            actor.ctl("select", result["caught"])
            actor.ctl("look-at-block", 9, 99, 7)
            assert actor.ctl("looking-at")["kind"] == "block", kind
            actor.ctl("mouse", "right", "click")
            game.ctl("wait", 5)
            spawned = f"@e[type={kind_id},x=9.5,y=100,z=7.5,distance=..1,limit=1]"
            assert data(spawned, "UUID") == original, (kind, original, data(spawned, "UUID"))
            assert data(spawned, "PersistenceRequired") in ("true", "1b", "1"), kind
            assert result["caught"] not in data(player, "Inventory"), (kind, actor.ctl("state"))
            command(f"tp {spawned} 13.5 105 13.5")
            command(f"kill @e[type={kind_id},x=13.5,y=105,z=13.5,distance=..1,limit=1]")
        return result

    return {
        "bee": run("bee"),
        "bee_mounted": run("bee", mounted=True),
        "silverfish": run("silverfish"),
        "endermite": run("endermite"),
        "cod_dry": run("cod"),
        "cod_first_bucket": run("cod", (2, 10)),
        "cod_creative_without_bucket": run("cod", creative=True),
        "cod_creative_with_bucket": run("cod", (10,), creative=True),
        "cod_offhand": run("cod", (10,), offhand=True),
        "cod_bucket_offhand": run("cod", (40,)),
        "salmon": run("salmon", (10,)),
    }


if __name__ == "__main__":
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("version", type=Path)
    p.add_argument("reference", type=Path)
    p.add_argument("--baseline", action="store_true")
    p.add_argument("--lan", action="store_true")
    p.add_argument("--release", action="store_true")
    args = p.parse_args()
    with tempfile.TemporaryDirectory(prefix="nat-net-recipe-", ignore_cleanup_errors=True) as tmp:
        root = Path(tmp)

        class Game:
            def __init__(self, side):
                self.home = root / side
                self.home.mkdir()
                self.socket = root / f"{side}.sock"

            def ctl(self, *tokens):
                result = subprocess.run(
                    [args.version / "horizon-ctl", "--socket", self.socket, *map(str, tokens)],
                    capture_output=True, check=True, text=True, timeout=65,
                )
                return json.loads(result.stdout.splitlines()[-1])

            def launch(self, *params):
                self.ctl("launch", "--headless", "--game", args.version / "horizon-client", "--resources",
                         args.version / ".local/horizon-client/26.3", "--home", self.home,
                         "--game-dir", self.home, "--render-distance", 4, *params)

        game = Game("host")
        guest = None
        try:
            game.launch("--world", "Naturalist Net Recipes", "--creative", "--test-terrain", "--wait")
            actor, player = game, "@s"
            if args.lan:
                port = game.ctl("lan", "open", 0, "commands")["port"]
                guest = Game("guest")
                guest.launch()
                guest.ctl("connect", f"127.0.0.1:{port}", "NetGuest")
                guest.ctl("wait-for", "loaded", "--timeout", 55)
                actor, player = guest, "NetGuest"
            actual = collect(game, actor, player, args.release)
            print(json.dumps(actual, indent=2), flush=True)
            expected = json.loads(args.reference.read_text())
            if args.baseline:
                assert not actual["bee"]["removed"] and not actual["cod_first_bucket"]["removed"], actual
            else:
                assert actual == expected, (actual, expected)
        finally:
            if guest and guest.socket.exists():
                guest.ctl("quit")
            if game.socket.exists():
                game.ctl("quit")
