"""Compare the pinned Java attribute map after data commands and reload."""
import argparse
from pathlib import Path
import java_reference


def capture(command, _):
    def datum(query):
        return next((row.split("following entity data: ", 1)[1] for row in command(query)
                     if "following entity data: " in row), None)

    def scale():
        return next((line.split(" is ", 1)[1] for line in command(
                    f"attribute {deer} minecraft:scale get") if " is " in line), None)

    deer = "@e[tag=deer_scale_reference,limit=1]"
    scale_tag = 'attributes[{id:"minecraft:scale"}]'
    command("gamerule minecraft:spawn_mobs false")
    command("forceload add 0 0")
    command("kill @e[tag=deer_scale_reference]")
    command("summon naturalist:deer 8.5 102 8.5 {NoAI:1b,NoGravity:1b,Tags:[deer_scale_reference]}")
    before = scale()
    command(f"attribute {deer} minecraft:scale base set 2")
    command(f"attribute {deer} minecraft:scale modifier add naturalist:fixture_scale 0.5 add_multiplied_total")
    modified = scale()
    tag_before = datum(f"data get entity {deer} {scale_tag}")
    command(f"data merge entity {deer} {{Silent:1b}}")
    merged = scale()
    command(f"data remove entity {deer} {scale_tag}")
    removed = scale()
    tag_after = datum(f"data get entity {deer} {scale_tag}")
    command(f"data merge entity {deer} {{attributes:[{{id:\"minecraft:scale\",base:4d,modifiers:[]}}]}}")
    overwritten = scale()
    return {"initial": before, "modified": modified, "modified_nbt":tag_before,
            "after_unrelated_merge": merged, "after_remove": removed, "after_remove_nbt": tag_after,
            "after_explicit_replacement": overwritten}



def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("server", "java_home", "output"):
        parser.add_argument(name, type=Path)
    args = parser.parse_args()
    args.variants = args.grazing = False
    args.behavior = True
    java_reference.deer_behavior.capture = capture
    java_reference.run_server(args, ["sh", "run.sh", "nogui", "--world", "deer-scale-nbt-fixture"])


if __name__ == "__main__":
    main()
