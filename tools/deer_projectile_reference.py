"""Compare simultaneous projectile hits relayed across a three-deer herd."""
import argparse
import json
from pathlib import Path
import time

import java_reference


def scenario(command, wait):
    def snapshot():
        key = "Naturalist projectile herd "
        rows = command("naturalist_fixture_deer_projectiles")
        return next(json.loads(row.split(key, 1)[1]) for row in rows if key in row)

    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 31 15")
    command("tick freeze")
    command("kill @e[tag=projectile_herd]")
    command("fill 0 100 0 31 100 15 minecraft:stone")
    command("fill 0 101 0 31 107 15 minecraft:air")
    for x in ("8.5", "14.5", "20.5"):
        command(f"summon naturalist:deer {x} 101 8.5 {{NoAI:1b,Tags:[projectile_herd]}}")
    assert len(snapshot()) == 3
    for x in ("8.5", "14.5"):
        rows = command(f"summon minecraft:arrow {x} 101.8 5.5 " +
                       "{NoGravity:1b,Motion:[0.0d,0.0d,2.0d],damage:2.0d,Tags:[projectile_herd]}")
        assert any("Summoned" in row for row in rows), rows

    for tick in range(1, 9):
        command("tick step 1")
        wait()
        after = snapshot()
        if all(mob[1] < 10 for mob in after[:2]):
            times = [mob[0] for mob in after]
            result = {"ticks_until_two_hits": tick, "timers_agree": len(set(times)) == 1,
                      "relay_to_third": 100 <= times[2] <= 199,
                      "health": [mob[1] for mob in after]}
            return result
    raise AssertionError(after)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = lambda command, _: scenario(command, lambda: time.sleep(0.2))
    with java_reference.deer_behavior.reference_probe(
        args.server, args.java_home, str(Path(__file__).parent / "reference/DeerProjectileProbe.java")
    ):
        java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "deer-projectile-fixture"])


if __name__ == "__main__":
    main()
