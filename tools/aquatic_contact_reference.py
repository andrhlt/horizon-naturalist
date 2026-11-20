"""Observe real mob impulses from the aquatic contact matrix in the pinned Java mod."""
import argparse
from pathlib import Path
import time
from aquatic_contact_cases import capture_contact
import java_reference


def capture(command, _):
    command("forceload add 0 0 31 31")
    command("tick freeze")

    def step(ticks):
        command(f"tick step {ticks}")
        time.sleep(ticks / 20 + 0.12)

    return capture_contact(command, step)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "aquatic-contact"])
