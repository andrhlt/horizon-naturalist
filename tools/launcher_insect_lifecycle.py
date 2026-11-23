"""Run the butterfly life cycle in the Horizon launcher build without opening a window."""
import argparse
import json
from pathlib import Path
import shutil
import subprocess
import tempfile


def capture(version, reference, jar=None):
    with tempfile.TemporaryDirectory(prefix="nat-insect-", ignore_cleanup_errors=True) as work:
        home = Path(work)
        socket = home / "insect.sock"
        if jar is not None:
            import hashlib
            from prepare import JAR_SHA
            if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA:
                raise ValueError("expected the pinned Naturalist 2.0.7 NeoForge 26.3 jar")
            (home / "jars").mkdir()
            shutil.copyfile(jar, home / "jars" / f"{JAR_SHA}.jar")
            (home / "horizon-mods.txt").write_text("mod.naturalist:true\n")

        def ctl(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=80)
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def observed(predicate):
            return any("fixture_condition_true" in row for row in
                       cmd(f"execute if {predicate} run say fixture_condition_true"))

        def step(ticks):
            cmd(f"tick step {ticks}")
            ctl("wait", ticks + 2)

        def summon(entity):
            output = cmd("summon " + entity)
            assert any("Summoned" in row for row in output), output

        try:
            manifest = json.loads((version / "native-mods.json").read_text())
            base = manifest["base"]
            resources = Path.home() / "Library/Application Support/horizon-launcher/versions" / base / ".local/horizon-client/26.3"
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                resources, "--home", home, "--game-dir", home,
                "--world", "Insect Lifecycle Fixture", "--creative", "--test-terrain",
                "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("gamerule minecraft:random_tick_speed 0")
            cmd("tp @s 8.5 110 8.5")
            ctl("wait", 20)
            cmd("tick freeze")
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 107 15 minecraft:air")
            cmd("setblock 10 100 8 minecraft:oak_log")
            cmd("setblock 10 99 8 minecraft:oak_log")
            shell = "block 9 100 8 naturalist:chrysalis[age=0,facing=east]"
            summon("naturalist:caterpillar 9.5 100 8.5 {Age:-24000,NoGravity:1b,Tags:[fixture_baby]}")
            step(16)
            baby_makes_cocoon = observed(shell)
            assert observed("entity @e[tag=fixture_baby]")
            cmd("kill @e[tag=fixture_baby]")
            summon("naturalist:caterpillar 9.5 100 8.5 {Age:0,NoGravity:1b,Tags:[fixture_adult]}")
            step(16)
            adult_makes_cocoon = observed(shell)
            caterpillar_discarded = not observed("entity @e[tag=fixture_adult]")
            cmd("setblock 9 100 8 naturalist:chrysalis[age=3,facing=east]")
            cmd("gamerule minecraft:random_tick_speed 4096")
            for _ in range(8):
                step(2)
                if observed("block 9 100 8 minecraft:air"):
                    break
            chrysalis_removed = observed("block 9 100 8 minecraft:air")
            hatched = ctl("entities", 48, "naturalist:butterfly")["entities"]
            butterfly_count = len(hatched)
            butterfly_hatches = butterfly_count > 0
            step(20)
            after_hatch_window = ctl("entities", 48, "naturalist:butterfly")["entities"]
            butterfly_count_after_window = len(after_hatch_window)
            cmd("kill @e[type=naturalist:butterfly]")
            cmd("gamerule minecraft:random_tick_speed 0")

            cmd("setblock 4 99 4 minecraft:dirt")
            cmd("setblock 4 100 4 minecraft:dandelion")
            cmd("setblock 5 99 4 minecraft:farmland[moisture=7]")
            cmd("setblock 5 100 4 minecraft:wheat[age=0]")
            summon("naturalist:butterfly 4.5 101.5 4.5 {NoGravity:1b,PersistenceRequired:1b,Tags:[fixture_pollinator]}")
            step(180)
            grew_crop = not observed("block 5 100 4 minecraft:wheat[age=0]")
            cmd("kill @e[type=naturalist:butterfly]")
            summon("naturalist:butterfly 2.5 100 2.5 {NoGravity:1b,InLove:600,PersistenceRequired:1b}")
            summon("naturalist:butterfly 3.5 100 2.5 {NoGravity:1b,InLove:600,PersistenceRequired:1b}")
            step(100)
            breeds_caterpillar = observed("entity @e[type=naturalist:caterpillar,x=2,y=100,z=2,distance=..5]")
            result = {
                "baby_makes_cocoon": baby_makes_cocoon,
                "adult_makes_cocoon": adult_makes_cocoon,
                "caterpillar_discarded": caterpillar_discarded,
                "chrysalis_faces_log": adult_makes_cocoon,
                "chrysalis_removed": chrysalis_removed,
                "butterfly_hatches": butterfly_hatches,
                "butterfly_count": butterfly_count,
                "butterfly_count_after_window": butterfly_count_after_window,
                "grew_crop": grew_crop,
                "breeds_caterpillar": breeds_caterpillar,
            }
            expected = {
                **{name: reference[name] for name in result.keys() & reference.keys()},
                "chrysalis_removed": True,
                "butterfly_hatches": True,
                "butterfly_count": 1,
                "butterfly_count_after_window": 1,
                "grew_crop": True,
            }
            print(json.dumps(result, indent=2), flush=True)
            assert result == expected, result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    parser.add_argument("--jar", type=Path, help="original jar required by the current external-jar package")
    args = parser.parse_args()
    capture(args.version, json.loads(args.reference.read_text()), args.jar)
