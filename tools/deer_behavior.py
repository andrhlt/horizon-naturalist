"""Reference deer herd damage and player avoidance under controlled inputs."""
from contextlib import contextmanager
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
import time


@contextmanager
def reference_probe(server, java_home, filename="DeerProbe.java"):
    destination = server / "mods/local-naturalist-fixture.jar"
    if destination.exists():
        raise FileExistsError(destination)
    source = Path(__file__).resolve().parent / "reference" / filename
    with tempfile.TemporaryDirectory(prefix="naturalist-java-probe-") as temp:
        scratch = Path(temp)
        classes = scratch / "classes"
        meta = classes / "META-INF"
        meta.mkdir(parents=True)
        (meta / "neoforge.mods.toml").write_text(
            'modLoader="javafml"\nloaderVersion="[4,)"\nlicense="MIT"\n'
            '[[mods]]\nmodId="naturalist_fixture"\nversion="0.1.0"\n'
            'displayName="Local Naturalist parity fixture"\n')
        jars = sorted((server / "libraries").rglob("*.jar")) + [server / "mods/naturalist.jar"]
        subprocess.run([java_home / "bin/javac", "-proc:none", "--release", "25",
                        "-classpath", os.pathsep.join(map(str, jars)), "-d", classes, source], check=True)
        archive = scratch / "fixture.jar"
        subprocess.run([java_home / "bin/jar", "--create", "--file", archive,
                        "-C", classes, "."], check=True)
        with archive.open("rb") as original, destination.open("xb") as installed:
            shutil.copyfileobj(original, installed)
        try:
            yield
        finally:
            destination.unlink()


def capture(command, y):
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0 47 15")
    time.sleep(2.0)
    command("tick freeze")
    command("kill @e[tag=deer_behavior_fixture]")
    command(f"fill 0 {y - 1} 0 31 {y - 1} 15 minecraft:stone")
    command(f"fill 0 {y} 0 31 {y + 4} 15 minecraft:air")
    for tag, x in (("fixture_hit", 8.5), ("fixture_near", 14.5),
                   ("fixture_far", 24.5), ("fixture_sneak", 8.5)):
        z = 12.5 if tag == "fixture_sneak" else 6.5
        command(f"summon naturalist:deer {x} {y} {z} "
                f"{{Tags:[{tag},deer_behavior_fixture]}}")
    command(f"summon minecraft:zombie 6.5 {y} 6.5 "
            "{NoAI:1b,Tags:[deer_attacker,deer_behavior_fixture]}")
    command("damage @e[tag=fixture_hit,limit=1] 1 minecraft:mob_attack by @e[tag=deer_attacker,limit=1]")

    def probe(name, pattern):
        rows = command(f"deer_fixture_probe {name}")
        for row in rows:
            if (match := re.search(pattern, row)) is not None:
                return match.groups()
        raise AssertionError((name, rows))

    herd_pattern = r"deer_fixture_herd (-?\d+) (-?\d+) (-?\d+) (true|false)"
    hit, near, far, attacker = probe("herd", herd_pattern)
    assert 100 <= int(hit) <= 199 and hit == near and far == "0" and attacker == "true", (hit, near, far, attacker)
    # Determining the seeded timer is unnecessary for runtime parity. It is
    # sampled by the entity, so measure relative to the observed draw.
    command(f"tick sprint {int(hit) + 3}")
    deadline = time.monotonic() + 20
    while True:
        hit_end, near_end, far_end, attacker_end = probe("herd", herd_pattern)
        if int(hit_end) < 0:
            break
        assert time.monotonic() < deadline, (hit_end, near_end, far_end, attacker_end)
        time.sleep(0.2)
    assert (hit_end, near_end, far_end, attacker_end) == ("-1", "-1", "-1", "false")
    standing, sneaking = probe("sneak", r"deer_fixture_sneak (true|false) (true|false)")
    assert (standing, sneaking) == ("true", "false"), (standing, sneaking)
    return {"herd_panic_roll": [100, 199],
            "shares_roll_in_range": True, "shares_roll_at_16_blocks": False,
            "timer_stops_at": -1, "clears_attacker": True,
            "player_avoidance": {"standing": True, "sneaking": False}}
