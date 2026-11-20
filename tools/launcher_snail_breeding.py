"""Check real Beetroot feeding and egg-carrying versus Cow-template Wheat."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version):
    with tempfile.TemporaryDirectory(prefix="nat-snail-breeding-", ignore_cleanup_errors=True) as work:
        directory = Path(work)
        socket = directory / "snail-breeding.sock"

        def ctl(*args):
            reply = subprocess.run(
                [version / "horizon-ctl", "--socket", socket, *map(str, args)],
                check=True, capture_output=True, text=True, timeout=75,
            )
            return json.loads(reply.stdout.splitlines()[-1])

        def cmd(line):
            return ctl("command", line)["chat"]

        def data(target, nbt):
            marker = "following entity data: "
            return next((row.split(marker, 1)[1] for row in cmd(f"data get entity {target} {nbt}")
                         if marker in row), None)

        def click_at(pitch):
            ctl("look", 0, pitch)
            ctl("mouse", "right", "click")
            ctl("wait", 4)

        try:
            ctl("launch", "--headless", "--game", version / "horizon-client",
                "--resources", version / ".local/horizon-client/26.3", "--home", directory,
                "--game-dir", directory, "--world", "Snail Breeding Fixture", "--creative",
                "--test-terrain", "--render-distance", 4, "--wait")
            cmd("gamerule minecraft:spawn_mobs false")
            cmd("tp @s 7.5 100 6.5")
            ctl("wait", 20)
            cmd("fill 0 99 0 15 99 15 minecraft:stone")
            cmd("fill 0 100 0 15 107 15 minecraft:air")
            cmd("tp @s 7.5 100 6.5")
            cmd("gamemode survival @s")
            cmd("clear @s")
            for label, x in (("a", 7.5), ("b", 9.0)):
                cmd(f"summon naturalist:snail {x} 100 8.5 {{Tags:[breeding_{label}]}}")
            first, second = ("@e[tag=breeding_a,limit=1]", "@e[tag=breeding_b,limit=1]")
            cmd("item replace entity @s weapon.mainhand with minecraft:wheat 2")
            ctl("wait", 5)
            click_at(32)
            retained_wheat = data("@s", "SelectedItem.count") == "2"
            wheat_did_not_feed = data(first, "InLove") in (None, "0")
            cmd("item replace entity @s weapon.mainhand with minecraft:beetroot 2")
            ctl("wait", 5)
            click_at(32)
            first_ate = data("@s", "SelectedItem.count") == "1" and int(data(first, "InLove") or 0) > 0
            cmd("tp @s 9.0 100 6.5")
            ctl("wait", 5)
            click_at(32)
            second_ate = data("@s", "SelectedItem.id") is None and int(data(second, "InLove") or 0) > 0

            cmd("gamemode creative @s")
            cmd("tp @s 9.0 104 6.5")
            cmd("tick freeze")
            before = [data(snail, "InLove") for snail in (first, second)]
            cmd("tick step 90")
            ctl("wait", 94)
            gravid = [data(snail, "HasEgg") in ("true", "1b", "1") for snail in (first, second)]
            cleared_love = all(data(snail, "InLove") in (None, "0") for snail in (first, second))
            result = {"retained_wheat": retained_wheat,
                      "wheat_did_not_feed": wheat_did_not_feed,
                      "first_ate": first_ate, "second_ate": second_ate,
                      "gravid_count": sum(gravid), "love_cleared": cleared_love}
            print(json.dumps({"result": result, "love_before_ticks": before}, indent=2), flush=True)
            return result
        finally:
            if socket.exists():
                ctl("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    args = parser.parse_args()
    result = capture(args.version)
    assert all(result[key] for key in ("retained_wheat", "wheat_did_not_feed", "first_ate", "second_ate", "love_cleared")), result
    assert result["gravid_count"] == 1, result
