"""Replay Piranha school recruitment and retaliation headlessly."""
import json
from pathlib import Path
import subprocess
import sys
import time
from launcher_selected_headless import LAUNCHER, SelectedHeadless

version = Path(sys.argv[1])
world = f"APP2129 Piranha School {int(time.time())}"
socket = Path("/private/tmp") / f"{world.replace(' ', '-')}.sock"
client = SelectedHeadless(version, socket, world, "--creative")
ctl = client.call
cmd = client.command

def step(ticks):
    cmd(f"tick step {ticks}")
    ctl("wait", ticks + 3)

def probe(query):
    marker = "Piranha fixture "
    rows = cmd("naturalist_fixture_piranha " + query)
    return next(json.loads(row.split(marker, 1)[1]) for row in rows if marker in row)

def tags_present(tags):
    return [tag for tag in tags if any("present" in row for row in cmd(
        f"execute if entity @e[type=naturalist:piranha,tag={tag},limit=1] run say present"))]

try:
    cmd("gamerule minecraft:spawn_mobs false")
    cmd("tp @s 28.5 102 28.5")
    ctl("wait", 20)
    cmd("tick freeze")
    cmd("forceload add 0 0 31 31")
    cmd("fill 0 91 0 31 91 31 minecraft:stone")
    cmd("fill 0 92 0 31 96 31 minecraft:water")
    summon_count = 0
    summon_outcomes = []
    fish_tags = []
    for index in range(25):
        tag = f"app2129_school_fish_{index}"
        fish_tags.append(tag)
        x, z = 14.1 + ((index % 5) - 2) * 1.2, 8.5 + ((index // 5) - 2) * 1.2
        rows = cmd(f"summon naturalist:piranha {x:.1f} 92.5 {z:.1f} {{NoAI:1b,NoGravity:1b,PersistenceRequired:1b,Tags:[{tag}]}}")
        succeeded = any("summoned" in row.lower() for row in rows)
        summon_count += int(succeeded)
        immediate_present = bool(tags_present([tag]))
        summon_outcomes.append({"tag": tag, "succeeded": succeeded,
                                "immediate_present": immediate_present, "chat": rows[-2:]})
    rows = cmd("summon naturalist:piranha 16.25 92.5 8.5 {PersistenceRequired:1b,Tags:[app2129_school_leader]}")
    leader_summoned = any("summoned" in row.lower() for row in rows)
    summon_count += int(leader_summoned)
    summon_outcomes.append({"tag": "app2129_school_leader", "succeeded": leader_summoned, "chat": rows[-2:]})
    leader_tag_before_tick = cmd("data get entity @e[type=naturalist:piranha,tag=app2129_school_leader,limit=1] UUID")
    step(1)
    present_before_school_tick = tags_present(fish_tags + ["app2129_school_leader"])
    started = cmd("naturalist_fixture_piranha start app2129_school_leader")
    assert any("start Some" in row for row in started), started
    before_tick = probe("before-tick")
    school = probe("school")
    checkpoints = []
    for school_tick in range(20):
        if school["attached_followers"] >= 23:
            break
        step(1)
        school = probe("school")
        checkpoints.append({"tick": school_tick + 1, **school})
    present_after_school = tags_present(fish_tags + ["app2129_school_leader"])
    cmd("summon minecraft:zombie 24.5 92 25.5 {PersistenceRequired:1b,NoAI:1b,Tags:[app2129_attacker]}")
    cmd("summon naturalist:piranha 25.5 92.5 25.5 {PersistenceRequired:1b,Tags:[app2129_retaliator]}")
    step(1)  # Let the spawn listener install target goals before the hit.
    before_retaliation = probe("before-retaliation")
    damage = cmd("damage @e[type=naturalist:piranha,tag=app2129_retaliator,limit=1] 1 minecraft:mob_attack by @e[type=minecraft:zombie,tag=app2129_attacker,limit=1,sort=nearest]")
    step(10)
    retaliation = probe("retaliation")
    present_tags = tags_present(fish_tags + ["app2129_school_leader", "app2129_retaliator"])
    result = {"artifact": version.name, "summon_count": summon_count,
              "before_school_tick": before_tick, "school": school, "retaliation": retaliation,
              "live_school_checkpoints": checkpoints,
              "setup": "25 NoAI fish in loaded water within Java FollowFlockLeaderGoal's 8-block recruitment box; offset positions avoid coincident-entity collision. One active leader has only its PiranhaSchool delay triggered.",
              "before_retaliation": before_retaliation,
              "present_before_school_tick": present_before_school_tick,
              "present_after_school": present_after_school,
              "present_piranha_tags_after_replay": present_tags,
              "leader_tag_before_tick": leader_tag_before_tick,
              "start_result": started,
              "damage_result": damage,
              "retaliation_setup": "A separate live Piranha is hit by a NoAI Zombie to isolate hurt-by retaliation from the school cohort.",
              "summon_outcomes": summon_outcomes,
              "retaliator_target_matches_attacker": f"Mob({retaliation['attacker_id']})" in str(retaliation["retaliator_target"]),
              "java_baseline": json.loads(Path("fixtures/piranha-java.json").read_text())}
    print(json.dumps(result, indent=2), flush=True)
    Path(sys.argv[2]).write_text(json.dumps(result, indent=2) + "\n")
    assert summon_count == 26 and school["members"] == 26 and school["largest_school"] == 24 and school["attached_followers"] == 23, school
    assert len(present_tags) == 27, {"present": present_tags, "expected": fish_tags + ["app2129_school_leader", "app2129_retaliator"]}
    assert f"Mob({retaliation['attacker_id']})" in str(retaliation["retaliator_target"]), retaliation
finally:
    client.close()
    subprocess.run([LAUNCHER, "worlds", "delete", "--instance", "horizon", world],
                   capture_output=True, text=True, env=client.env)
