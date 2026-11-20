"""Capture baby attribute caps and the health fraction when animals mature."""
from argparse import ArgumentParser
from pathlib import Path
import time

import java_reference

SPECIES = ("deer", "duck", "boar", "turkey", "capybara", "komodo_dragon", "rhino")


def capture(command, wait, damage=False, live_merge=False, *, species=SPECIES, isolated=False):
    def number(query, leader):
        rows = command(query)
        return next(float(row.split(leader, 1)[1].split()[0].removesuffix("f"))
                    for row in rows if leader in row)

    def stats(selector):
        return [
            number(f"attribute {selector} minecraft:max_health get", " is "),
            number(f"data get entity {selector} Health", "following entity data: "),
        ]

    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 47 31" if isolated else "forceload add 0 0")
    command("tick freeze")
    if isolated:
        command("fill 0 142 0 46 157 30 minecraft:air")
    for i, kind in enumerate(species):
        command(f"kill @e[tag=baby_health_{i}]")
        x, z = (4.5 + 4 * (i % 7), 4.5 + 4 * (i // 7)) if isolated else (5.5 + i, 8.5)
        invulnerable = ",Invulnerable:1b" if isolated and not damage else ""
        command(f"summon naturalist:{kind} {x} 150 {z} "
                f"{{Age:-3,NoAI:1b,NoGravity:1b,Tags:[baby_health_{i}]{invulnerable}}}")
    selectors = {kind: f"@e[tag=baby_health_{i},limit=1]" for i, kind in enumerate(species)}
    before = {species: stats(selector) for species, selector in selectors.items()}
    command("tick step 1")
    wait()
    young = {species: stats(selector) for species, selector in selectors.items()}
    for selector in selectors.values():
        data = command(f"data get entity {selector} attributes")
        assert not any("naturalist:baby_health" in row for row in data), data
    phases = {"before_first_tick": before, "juvenile": young}
    if live_merge:
        for selector in selectors.values():
            rows = command(f'data merge entity {selector} {{CustomName:"Fixture"}}')
            assert any("modified" in row.lower() for row in rows), rows
        phases["after_live_merge"] = {species: stats(selector) for species, selector in selectors.items()}
    if damage:
        for selector in selectors.values():
            command(f"damage {selector} 1 minecraft:generic")
        phases["juvenile_damaged"] = {species: stats(selector) for species, selector in selectors.items()}
    command("tick step 4")
    wait()
    phases["mature"] = {species: stats(selector) for species, selector in selectors.items()}
    return phases


def main():
    cli = ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        cli.add_argument(name, type=Path)
    cli.add_argument("--damage", action="store_true")
    cli.add_argument("--live-merge", action="store_true")
    cli.add_argument("--species", nargs="+", default=SPECIES, metavar="MOB")
    cli.add_argument("--isolated", action="store_true")
    args = cli.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = lambda command, _: capture(
        command, lambda: time.sleep(0.3), args.damage, args.live_merge, species=args.species, isolated=args.isolated)
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "naturalist-baby-health-fixture"])


if __name__ == "__main__":
    main()
