"""Capture an isolated Naturalist deer fixture over the Java server console."""
import argparse
from forest_dasher import brew
import deer_variants
import deer_grazing
import deer_behavior
import json
import queue
import re
import subprocess
import threading
import time
from pathlib import Path


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("server", type=Path)
    cli.add_argument("java_home", type=Path)
    cli.add_argument("output", type=Path)
    task = cli.add_mutually_exclusive_group()
    task.add_argument("--variants", action="store_true")
    task.add_argument("--grazing", action="store_true")
    task.add_argument("--behavior", action="store_true")
    args = cli.parse_args()
    server_args = ["sh", "run.sh", "nogui"]
    if args.variants:
        world = "deer-variant-fixture"
        deer_variants.write_pack(args.server / world / "datapacks" / deer_variants.PACK)
        server_args += ["--world", world]
    if args.grazing:
        server_args += ["--world", "deer-grazing-fixture"]
    if args.behavior:
        server_args += ["--world", "deer-behavior-fixture"]
    if args.behavior:
        with deer_behavior.reference_probe(args.server, args.java_home):
            run_server(args, server_args)
    else:
        run_server(args, server_args)


def run_server(args, server_args):
    process = subprocess.Popen(
        server_args, cwd=args.server,
        env={"JAVA_HOME": str(args.java_home), "PATH": f"{args.java_home}/bin:/usr/bin:/bin"},
        stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
        text=True, bufsize=1,
    )
    lines = queue.Queue()
    log = args.output.with_suffix(".log")

    def pump():
        with log.open("w") as file:
            for line in process.stdout:
                file.write(line)
                file.flush()
                lines.put(line.strip())
            lines.put(None)

    threading.Thread(target=pump, daemon=True).start()

    def through(wanted, seconds=90):
        received = []
        while True:
            try:
                line = lines.get(timeout=seconds)
            except queue.Empty as error:
                raise RuntimeError(f"Missing {wanted!r}. Read {log}") from error
            if line is None:
                raise RuntimeError(f"Java exited waiting for {wanted!r}. Read {log}")
            if wanted in line:
                return received
            received.append(line)

    def command(text):
        process.stdin.write(f"{text}\nsay deer_fixture_end\n")
        process.stdin.flush()
        return through("deer_fixture_end", 15)

    try:
        through("Done (")
        if args.variants or args.grazing or args.behavior:
            task = (deer_variants if args.variants else
                    deer_grazing if args.grazing else deer_behavior)
            output = task.capture(command, 71)
            args.output.write_text(json.dumps(output, indent=2) + "\n")
            print(json.dumps(output, indent=2))
            return
        command("gamerule minecraft:spawn_mobs false")
        command("forceload add 0 0")
        command("kill @e[tag=fixture_deer]")
        command("kill @e[tag=fixture_fawn]")
        command("tick freeze")
        command("summon naturalist:deer 8.5 71 8.5 {NoAI:1b,Tags:[fixture_deer]}")
        query = "@e[type=naturalist:deer,tag=fixture_deer,limit=1]"
        output = {}
        for attribute in ("max_health", "movement_speed", "step_height"):
            reply = command(f"attribute {query} minecraft:{attribute} base get")
            match = next((re.search(r" is ([0-9.eE+-]+)", row) for row in reply if " is " in row), None)
            if not match:
                raise RuntimeError(f"Missing {attribute}: {reply}")
            output[attribute] = float(match[1])
        for nbt_key in ("Health", "Age", "Variant"):
            reply = command(f"data get entity {query} {nbt_key}")
            output[nbt_key] = next(
                (row.split("following entity data: ", 1)[1] for row in reply if "following entity data: " in row),
                None,
            )
        command("summon naturalist:deer 9.5 71 8.5 {Age:-24000,NoAI:1b,Tags:[fixture_fawn]}")
        reply = command("data get entity @e[tag=fixture_fawn,limit=1] Age")
        output["baby_age"] = next(row.split("following entity data: ", 1)[1]
                                  for row in reply if "following entity data: " in row)
        command("kill @e[tag=fixture_avoid]")
        command("kill @e[tag=fixture_threat]")
        command("time set midnight")
        command("fill 1 70 1 15 70 15 minecraft:stone")
        command("fill 1 71 1 15 74 15 minecraft:air")
        command("summon minecraft:zombie 7.5 71 8.5 {NoAI:1b,Tags:[fixture_threat]}")
        command("summon naturalist:deer 9.5 71 8.5 {Tags:[fixture_avoid]}")
        command("tick step 35")
        time.sleep(2.0)
        reply = command("data get entity @e[tag=fixture_avoid,limit=1] Pos")
        output["avoidance_position"] = next(
            (row.split("following entity data: ", 1)[1] for row in reply if "following entity data: " in row),
            None,
        )

        command("kill @e[tag=fixture_threat]")
        command("kill @e[tag=fixture_avoid]")
        command("kill @e[tag=fixture_couple]")
        command("kill @e[type=naturalist:deer,tag=!fixture_deer,tag=!fixture_fawn]")
        command("summon naturalist:deer 6.5 71 12.5 {Age:0,InLove:600,Tags:[fixture_couple]}")
        command("summon naturalist:deer 7.5 71 12.5 {Age:0,InLove:600,Tags:[fixture_couple]}")
        command("tick step 85")
        time.sleep(4.6)
        child = "@e[type=naturalist:deer,tag=!fixture_couple,tag=!fixture_deer,tag=!fixture_fawn,limit=1]"
        reply = command(f"data get entity {child} Age")
        output["bred_age"] = next(row.split("following entity data: ", 1)[1]
                                 for row in reply if "following entity data: " in row)
        assert -24000 <= int(output["bred_age"]) < -23900, output["bred_age"]
        output["forest_dasher"] = brew(command, 12, 71, 11)
        args.output.write_text(json.dumps(output, indent=2) + "\n")
        print(json.dumps(output, indent=2))
    finally:
        if process.poll() is None:
            process.stdin.write("stop\n")
            process.stdin.flush()
            try:
                process.wait(timeout=20)
            except subprocess.TimeoutExpired:
                process.terminate()
                process.wait(timeout=10)


if __name__ == "__main__":
    main()
