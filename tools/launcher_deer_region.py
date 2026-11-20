"""Compare chunk generation Deer groups with the two Java region RNG implementations."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def capture(version, expected, tagged):
    with tempfile.TemporaryDirectory(prefix="nat-deer-region-", ignore_cleanup_errors=True) as scratch:
        home = Path(scratch)
        socket = home / "region.sock"

        def control(*args):
            reply = subprocess.run([version / "horizon-ctl", "--socket", socket, *map(str, args)],
                                   check=True, capture_output=True, text=True, timeout=65)
            return json.loads(reply.stdout.splitlines()[-1])

        try:
            control("launch", "--headless", "--game", version / "horizon-client", "--resources",
                    version / ".local/horizon-client/26.3", "--home", home, "--game-dir", home,
                    "--world", "Naturalist Deer Region", "--creative", "--test-terrain",
                    "--render-distance", 4, "--wait")
            prefix = "Naturalist Deer region "
            def live_group(stream, seed):
                reply = control("command", f"naturalist_fixture_deer_region {stream} {seed}")["chat"]
                found = next((json.loads(row.partition(prefix)[2]) for row in reply if prefix in row), None)
                assert found is not None, (stream, seed, reply)
                return found

            compare = {}
            for group in expected["groups"]:
                stream = group["stream"]
                seed = group["seed"]
                found = live_group(stream, seed)
                assert found == {"members": group["members"], "next": group["next"]}, (stream, seed, found, group)
                compare[stream] = compare.get(stream, 0) + len(found["members"])

            from deer_variants import PACK, write_pack
            saves = [path for path in (home / "saves").iterdir() if path.is_dir()]
            assert len(saves) == 1, saves
            write_pack(saves[0] / "datapacks" / PACK)
            control("command", "reload")
            control("wait", 40)
            enabled = control("command", "datapack list enabled")["chat"]
            assert any(PACK in line for line in enabled), enabled
            tagged_members = 0
            for group in tagged["groups"]:
                stream, seed = group["stream"], group["seed"]
                found = live_group(stream, seed)
                assert found == {"members": group["members"], "next": group["next"]}, (stream, seed, found, group)
                tagged_members += len(found["members"])
            compare["tagged_pack_members"] = tagged_members
            return compare
        finally:
            if socket.exists():
                control("quit")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version", type=Path)
    parser.add_argument("reference", type=Path)
    parser.add_argument("tagged", type=Path)
    args = parser.parse_args()
    print(json.dumps(capture(args.version,
                             json.loads(args.reference.read_text()),
                             json.loads(args.tagged.read_text())), indent=2))
