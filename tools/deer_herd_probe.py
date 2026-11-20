"""Compare the Java herd fixture against a separately instrumented Horizon build."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile
import time


def enable_trace(engine):
    commands = engine / "crates/horizon-client/src/game/commands.rs"
    marker = '    pub(super) fn run_command(&mut self, command: &str) -> bool {'
    source = commands.read_text()
    if source.count(marker) != 1 or 'naturalist_fixture_herd' in source:
        raise ValueError("Expected a fresh staged Horizon with the Naturalist overlay")
    commands.write_text(source.replace(marker, marker + '''
        if command == "naturalist_fixture_herd" {
            let state: Vec<_> = self.mobs.iter()
                .filter(|mob| !mob.removed && mob.alive() && mob.kind.id == "naturalist:deer")
                .map(|mob| (mob.id, mob.extra.deer_panic_ticks, mob.ai.last_hurt_by.is_some()
                    || mob.ai.last_hurt_by_ref.is_some()))
                .collect();
            self.feedback(Text::literal(format!("naturalist_fixture_herd {}", serde_json::json!(state))));
            return true;
        }'''))


def capture(ctl, game, resources, pack, reference):
    reference = json.loads(reference.read_text())
    with tempfile.TemporaryDirectory(prefix="deer-herd-", ignore_cleanup_errors=True) as temp:
        root = Path(temp)
        socket = root / "herd.sock"

        def control(*words):
            result = subprocess.run([ctl, "--socket", socket, *map(str, words)],
                                    check=True, capture_output=True, text=True, timeout=60)
            return json.loads(result.stdout.splitlines()[-1])

        def command(text):
            return control("command", "/" + text)["chat"]

        def snapshot():
            rows = command("naturalist_fixture_herd")
            for row in rows:
                if "naturalist_fixture_herd " in row:
                    state = json.loads(row.split("naturalist_fixture_herd ", 1)[1])
                    return {id: (timer, attacker) for id, timer, attacker in state}
            raise AssertionError(rows)

        try:
            control("launch", "--headless", "--game", game, "--resources", resources,
                    "--home", root, "--game-dir", root, "--world", "Deer Herd Fixture",
                    "--creative", "--test-terrain", "--render-distance", 4,
                    "--dev-mod", pack, "--wait")
            command("tp @s 8.5 102 5.5")
            control("wait", 40)
            command("gamerule minecraft:spawn_mobs false")
            command("difficulty normal")
            command("time set midnight")
            command("tick freeze")
            command("fill 0 100 0 31 100 15 minecraft:stone")
            command("fill 0 101 0 31 105 15 minecraft:air")
            for tag, x in (("deer_hit", 8.5), ("deer_near", 14.5), ("deer_far", 24.5)):
                feedback = command(f"summon naturalist:deer {x} 101 8.5 {{Tags:[{tag}]}}")
                assert any("Summoned" in line for line in feedback), (tag, feedback)
            # The trace iterates the server's mob list in insertion order.
            # Client tracking may not have streamed the far deer while ticks are frozen.
            subjects = snapshot()
            assert len(subjects) == 3, subjects
            hit, near, far = subjects
            command("summon minecraft:zombie 6.5 101 8.5 {NoAI:1b,Tags:[fixture_attacker]}")
            command("damage @e[tag=deer_hit,limit=1] 1 minecraft:mob_attack by @e[tag=fixture_attacker,limit=1]")
            immediately = snapshot()
            roll, has_attacker = immediately[hit]
            low, high = reference["herd_panic_roll"]
            assert low <= roll <= high and has_attacker, immediately
            assert (immediately[far][0] == roll) == reference["shares_roll_at_16_blocks"], immediately
            shares_immediately = immediately[near][0] == roll

            command("tick step 1")
            time.sleep(0.2)
            start = snapshot()
            assert start[hit][0] == start[near][0] == roll - 1, start
            assert start[far][0] == reference["timer_stops_at"], start
            command(f"tick sprint {roll + 3}")
            deadline = time.monotonic() + 25
            while True:
                end = snapshot()
                if end[hit][0] == reference["timer_stops_at"]:
                    break
                assert time.monotonic() < deadline, end
                time.sleep(0.2)
            assert end[near][0] == end[far][0] == reference["timer_stops_at"], end
            assert not end[hit][1], end
            print(json.dumps({"original_roll": roll, "timers_on_damage": immediately,
                              "shares_immediately": shares_immediately,
                              "java_shares_immediately": reference["shares_roll_in_range"],
                              "timers_after_tick": start, "timers_after_expiry": end}, sort_keys=True))
            assert shares_immediately == reference["shares_roll_in_range"], (
                "Java shares the herd panic roll during the damage call itself", immediately
            )
        finally:
            if socket.exists():
                control("quit")


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    sub = cli.add_subparsers(dest="action", required=True)
    stage = sub.add_parser("stage", help="add the probe to a disposable staged engine")
    stage.add_argument("engine", type=Path)
    run = sub.add_parser("run", help="compare the instrumented Horizon build with Java")
    for name in ("ctl", "game", "resources", "pack", "reference"):
        run.add_argument(name, type=lambda value: Path(value).resolve(strict=True))
    args = cli.parse_args()
    if args.action == "stage":
        enable_trace(args.engine)
    else:
        capture(args.ctl, args.game, args.resources, args.pack, args.reference)


if __name__ == "__main__":
    main()
