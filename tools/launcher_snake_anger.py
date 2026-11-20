"""Exercise real Snake revenge and its saved actor after player and mob damage."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import time

version = Path(sys.argv[1])
name = "Naturalist Snake persistent anger"

with tempfile.TemporaryDirectory(prefix="nat-snake-anger-", ignore_cleanup_errors=True) as folder:
    home = Path(folder)
    socket = home / "control.sock"

    def ctl(*args):
        proc = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                              check=True, capture_output=True, text=True, timeout=75)
        return json.loads(proc.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def step(ticks):
        cmd(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    def data(target, key):
        tag = " following entity data: "
        return next((line.split(tag,1)[1] for line in cmd(f"data get entity {target} {key}") if tag in line), None)

    def launch(fresh=False):
        mode = ("--creative",) if fresh else ()
        ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
            version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
            "--world", name, *mode, "--test-terrain", "--render-distance", 4, "--wait")

    def snapshot(target, expected, label):
        end, actor = data(target,"anger_end_time"), data(target,"angry_at")
        print(label, json.dumps({"absolute_end": end, "actor": actor, "expected": expected}), flush=True)
        assert end is not None and int(end.rstrip("Ll")) > 0, (label, end)
        assert actor == expected, (label, actor, expected)
        return end, actor

    try:
        launch(fresh=True)
        cmd("difficulty normal")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:advance_time false")
        cmd("gamerule minecraft:random_tick_speed 0")
        cmd("tp @s 7.5 92 3.5")
        ctl("wait", 24)
        cmd("tick freeze")
        cmd("fill 2 91 2 28 91 15 minecraft:stone")
        cmd("fill 2 92 2 28 99 15 minecraft:air")
        cmd("tp @s 7.5 92 3.5")
        cmd("time set 1000")
        step(24)
        cmd("gamemode survival @s")
        owner = data("@s", "UUID")
        p_snake = "@e[type=naturalist:snake,tag=player_hit,limit=1]"
        m_snake = "@e[type=naturalist:snake,tag=mob_hit,limit=1]"
        cow = "@e[type=minecraft:cow,tag=attacker,limit=1]"
        spawned = cmd('summon naturalist:snake 6.5 92 5.5 {Tags:[player_hit],Variant:"naturalist:snake",PersistenceRequired:1b}')
        assert any("Summoned" in line for line in spawned), spawned
        step(3)
        cmd("tp @s 7.5 92 3.5")
        ctl("wait", 4)
        entities = ctl("entities", 6, "naturalist:snake")["entities"]
        assert entities, {"spawned": spawned, "player": data("@s", "Pos"), "snake": data(p_snake, "Pos")}
        target = entities[0]
        assert ctl("look-at-entity", target["id"])["target"]["kind"] == "entity"
        assert data(p_snake, "Health") == "12.0f"
        ctl("mouse", "left", "click")
        ctl("wait", 4)
        assert data(p_snake, "Health") != "12.0f"
        step(8)
        first = snapshot(p_snake, owner, "after physical survival swing")
        assert snapshot(p_snake, owner, "second NBT read") == first
        cmd(f"data merge entity {p_snake} {{NoAI:1b}}")
        step(12)
        assert snapshot(p_snake, owner, "after elapsed ticks") == first

        cmd('summon naturalist:snake 13.5 92 5.5 {Tags:[mob_hit],Variant:"naturalist:snake",PersistenceRequired:1b}')
        cmd('summon minecraft:cow 14.5 92 5.5 {Tags:[attacker],PersistenceRequired:1b,NoAI:1b}')
        step(3)
        cow_uuid = data(cow, "UUID")
        assert cow_uuid
        cmd(f"damage {m_snake} 1 minecraft:mob_attack by {cow}")
        assert data(m_snake, "Health") == "11.0f"
        assert data(m_snake, "last_hurt_by_mob") == cow_uuid
        step(8)
        second = snapshot(m_snake, cow_uuid, "after attributed mob damage")
        assert snapshot(m_snake, cow_uuid, "second NBT read") == second
        cmd(f"data merge entity {m_snake} {{NoAI:1b}}")
        step(12)
        assert snapshot(m_snake, cow_uuid, "after elapsed ticks") == second

        cmd("save-all flush")
        ctl("quit")
        deadline = time.monotonic() + 10
        while socket.exists() and time.monotonic() < deadline:
            time.sleep(0.1)
        launch()
        cmd("tick freeze")
        assert snapshot(p_snake, owner, "after player-anger reload") == first
        assert snapshot(m_snake, cow_uuid, "after mob-anger reload") == second
        print("Snake kept the exact actor UUID and absolute end through queries and save reload for both damage sources.")
    finally:
        if socket.exists():
            try:
                ctl("quit")
            except subprocess.SubprocessError:
                pass
