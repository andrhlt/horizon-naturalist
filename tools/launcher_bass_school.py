"""Compare the instrumented launcher build's bass schooling with the Java fixture."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-bass-school-", ignore_cleanup_errors=True) as work:
    home = Path(work)
    socket = home / "fish.sock"

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                check=True, capture_output=True, text=True, timeout=45)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def fixture(line):
        rows = cmd(f"naturalist_fixture_bass_school {line}")
        return next(row.split("Bass school fixture ", 1)[1]
                    for row in rows if "Bass school fixture " in row)

    def fish(tag, variant="bass", x=8.5, no_ai=False):
        kind = "minecraft:cod" if variant == "cod" else "naturalist:bass"
        rows = cmd(f'summon {kind} {x} 102.5 8.5 {{Variant:"naturalist:{variant}",'
                   f'NoAI:{int(no_ai)}b,PersistenceRequired:1b,Tags:[{tag}]}}')
        assert any("Summoned" in row for row in rows), rows

    def inspect(tag):
        id_, leader, followers, tier, alive = fixture(f"inspect {tag}").split()
        return dict(id=int(id_), leader=int(leader), followers=int(followers), tier=int(tier),
                    alive=alive == "true")

    def step():
        cmd("tick step 4")
        ctl("wait", 5)

    try:
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", "Bass School Fixture", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("tp @s 8.5 106 5.5")
        ctl("wait", 20)
        cmd("fill 3 99 3 17 106 14 minecraft:stone")
        cmd("fill 4 100 4 16 105 13 minecraft:water")
        cmd("fill 3 106 3 17 108 14 minecraft:air")
        cmd("tp @s 8.5 106 5.5")
        cmd("tick freeze")

        fish("leader")
        fish("same1", x=8.9)
        fish("mixed", "bass_medium", x=9.3, no_ai=True)
        fish("large", "bass_large", x=9.7, no_ai=True)
        fish("cod", "cod", x=10.1, no_ai=True)
        fish("same2", x=10.5)
        fish("same3", x=12.9)
        fish("same4", x=13.3)
        fish("same5", x=13.7)
        assert fixture("start leader") == "ready"
        step()
        leader = inspect("leader")
        observed = {
            "same_tier_follows": inspect("same1")["leader"] == leader["id"],
            "mixed_tier_follows": inspect("mixed")["leader"] == leader["id"],
            "large_follows": inspect("large")["leader"] == leader["id"],
            "cod_follows": inspect("cod")["leader"] == leader["id"],
            "mixed_candidate_count": leader["followers"],
        }
        # Put the established leader in the following section while keeping it in
        # sight, so the self-skip does not consume the next joiner's stream slot.
        cmd("tp @e[tag=leader,limit=1] 16.15 102.5 8.5")
        cmd("tp @e[tag=mixed,limit=1] 4.15 102.5 8.5")
        cmd("tp @e[tag=large,limit=1] 4.5 102.5 8.5")
        for tag in ("same3", "same4", "same5"):
            fixture(f"start {tag}")
            step()
        capacity = inspect("leader")["followers"]
        observed.update(school_max=capacity + 1, count_at_capacity=capacity)

        # Set up a medium follower and exercise the real melee damage entry point.
        fish("medium_leader", "bass_medium", no_ai=True)
        fish("growing", "bass_medium", no_ai=True)
        fish("grove_prey", no_ai=True)
        fish("nongrowing", "bass_medium", no_ai=True)
        fish("other_prey", no_ai=True)
        assert fixture("attach growing medium_leader") == "attached"
        assert inspect("medium_leader")["followers"] == 1
        assert fixture("attack growing grove_prey growth") == "true"
        assert fixture("attack nongrowing other_prey nongrowth") == "true"
        observed.update(
            growth_variant="naturalist:bass_large" if inspect("growing")["tier"] == 2 else None,
            growth_discards_prey=not inspect("grove_prey")["alive"],
            growth_removes_follower=inspect("growing")["leader"] == 0,
            growth_updates_old_leader=inspect("medium_leader")["followers"] == 0,
            nongrowth_variant="naturalist:bass_medium" if inspect("nongrowing")["tier"] == 1 else None,
        )
        print(json.dumps(observed), flush=True)
        assert observed == reference, (observed, reference)
    finally:
        if socket.exists():
            ctl("quit")
