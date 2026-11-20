"""Replay the pinned Piranha school and retaliation in Horizon."""
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import time

version = Path(sys.argv[1])
reference = json.loads(Path(sys.argv[2]).read_text())
with tempfile.TemporaryDirectory(prefix="nat-piranha-", ignore_cleanup_errors=True) as work:
    home = Path(work)
    socket = home / "fish.sock"

    def ctl(*args):
        result = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                capture_output=True, check=True, text=True, timeout=75)
        return json.loads(result.stdout.splitlines()[-1])

    def cmd(line):
        return ctl("command", line)["chat"]

    def data(target, key):
        return next(row.split("following entity data: ", 1)[1]
                    for row in cmd(f"data get entity {target} {key}") if "following entity data: " in row)

    def step(ticks):
        cmd(f"tick step {ticks}")
        ctl("wait", ticks + 2)

    child = None
    try:
        launcher = os.environ.get("NATURALIST_STOCK_LAUNCHER")
        if launcher:
            log = Path(sys.argv[2]).parent.parent / "piranha/play.log"
            with log.open("w") as handle:
                child = subprocess.Popen([launcher, "play", "--version", version.name,
                    "--world", f"Piranha School {home.name}", "--creative", "--arg=--control-socket",
                    f"--arg={socket}", "--arg=--headless", "--arg=--test-terrain", "--arg=--render-distance", "--arg=4"],
                    stdout=handle, stderr=subprocess.STDOUT)
            for _ in range(50):
                if child.poll() is not None:
                    raise RuntimeError(f"Horizon Play exited with code {child.returncode}")
                if socket.exists() and ctl("state")["screen"] == "playing":
                    break
                time.sleep(1)
            else:
                raise RuntimeError("Timed out waiting for Horizon Play to load the fixture world")
        else:
            ctl("launch", "--headless", "--game", version / "horizon-client", "--resources",
                version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                "--world", "Piranha School", "--creative", "--test-terrain", "--render-distance", 4, "--wait")
        cmd("gamerule minecraft:spawn_mobs false")
        cmd("gamerule minecraft:max_entity_cramming 0")
        cmd("difficulty normal")
        cmd("tp @s 7.5 101 6.5")
        ctl("wait", 20)
        cmd("fill 2 100 2 20 100 15 minecraft:stone")
        cmd("fill 2 101 2 20 107 15 minecraft:air")
        cmd("tick freeze")
        cmd("fill 4 101 8 18 104 14 minecraft:water")

        def fixture(command):
            return next(row.split("Piranha fixture ", 1)[1]
                        for row in cmd("naturalist_fixture_piranha " + command) if "Piranha fixture " in row)

        def inspect(tag):
            identity, leader, followers, target = map(int, fixture("inspect " + tag).split())
            return dict(identity=identity, leader=leader, followers=followers, target=target)

        for index in range(25):
            data_tag = f"{{NoAI:1b,PersistenceRequired:1b,Tags:[piranha{index}]}}"
            cmd(f"summon naturalist:piranha 14.1 102.25 8.5 {data_tag}")
        cmd('summon naturalist:piranha 16.25 102.25 8.5 {PersistenceRequired:1b,Tags:[leader]}')
        assert fixture("start leader") == "ready"
        step(4)
        leader = "@e[type=naturalist:piranha,tag=leader,limit=1]"
        output = {}
        for attribute in ("max_health", "movement_speed", "attack_damage"):
            rows = cmd(f"attribute {leader} minecraft:{attribute} base get")
            output[attribute] = float(next(m[1] for row in rows if (m := re.search(r" is ([0-9.eE+-]+)", row))))
        output["variant"] = data(leader, "Variant").strip('"')
        school = inspect("leader")
        output["attached_at_capacity"] = sum(inspect(f"piranha{index}")["leader"] == school["identity"]
                                               for index in range(25))
        output["first_recruitment"] = output["attached_at_capacity"]
        assert output["attached_at_capacity"] == school["followers"]
        assert inspect("piranha0")["leader"] == school["identity"], output

        cmd(f"data merge entity {leader} {{NoAI:1b}}")
        cmd('summon minecraft:cod 10.5 102.25 9.5 {NoAI:1b,PersistenceRequired:1b,Tags:[cod]}')
        cmd('summon minecraft:salmon 10.5 102.25 10.5 {NoAI:1b,PersistenceRequired:1b,Tags:[salmon]}')
        cod, salmon = inspect("cod")["identity"], inspect("salmon")["identity"]
        output["hurt_landed"] = fixture("hurt piranha0 cod") == "true"
        output["follower_retaliates"] = inspect("piranha0")["target"] == cod
        step(2)
        output["leader_retaliates"] = inspect("leader")["target"] == cod
        assert fixture("set_target leader salmon") == "set"
        step(2)
        output["follower_retargets"] = inspect("piranha0")["target"] == salmon
        assert fixture("set_target leader") == "set"
        step(2)
        output["follower_clears_target"] = inspect("piranha0")["target"] == 0

        cmd("setblock 8 33 8 minecraft:water")
        cmd("setblock 8 34 8 minecraft:air")
        cmd("setblock 8 64 8 minecraft:water")
        cmd("setblock 7 33 8 minecraft:air")
        output.update(json.loads(fixture("spawning")))
        print(json.dumps(output, indent=2), flush=True)
        assert output == reference, (output, reference)
    finally:
        if socket.exists():
            ctl("quit")
        if child is not None:
            try:
                child.wait(timeout=8)
            except subprocess.TimeoutExpired:
                child.terminate()
                child.wait(timeout=8)
