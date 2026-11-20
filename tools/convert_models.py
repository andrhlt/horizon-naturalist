"""Translate the pinned Java animal layer definitions into Horizon model builders."""
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
out = Path(sys.argv[2])
species = sys.argv[3] if len(sys.argv) > 3 else "deer"
clips = {
    "mole": (("IDLE", "WALK", "DIG_DOWN", "DIG_UP", "PEEK", "SPAWN", "UNDERGROUND_HIDDEN"),),
    "elephant": (("IDLE", "WALK", "RUN", "SWING"), ("IDLE", "WALK", "RUN")),
    "mammoth": (("IDLE", "WALK", "RUN", "SWING"), ("IDLE", "WALK", "RUN")),
    "dirt_trail": (("SPAWN",),),
    "whale": (("IDLE", "SWIM", "FLOP"), ("SWIM_IDLE", "SWIM", "FLOP")),
    "great_white_shark": (("IDLE", "SWIM", "SWIM_FAST", "FLOP", "ATTACK"),),
    "hedgehog": (("IDLE", "IDLE_EVENT", "WALK", "SIT", "HIDE", "UNHIDE", "ROLL_GROUND", "ROLL_AIR"),),
    "ostrich": (("IDLE", "WALK_B", "RUN_B_1", "BURY_HEAD", "ATTACK", "SIT", "FLAP"),
                ("IDLE", "WALK", "RUN", "SIT_IDLE")),
    "hippo": (("IDLE", "WALK", "RUN", "SWIM", "SWIM_IDLE", "BITE", "SLEEP"), ("IDLE", "WALK", "RUN", "SIT", "UNSIT", "SIT_IDLE", "HEADBUTT")),
    "alligator": (("IDLE", "WALK", "SWIM", "BITE"), ("IDLE", "WALK", "SWIM")),
    "zebra": (("IDLE", "WALK", "RUN"), ("IDLE", "WALK", "RUN")),
    "deer": (("IDLE", "WALK", "RUN", "EAT"), ("IDLE", "WALK", "RUN", "EAT")),
    "bass": (("SWIM", "FLOP"),),
    "catfish": (("SWIM", "FLOP", "BITE"),),
    "piranha": (("SWIM", "FLOP", "ATTACK"),),
    "anglerfish": (("SWIM", "SWIM_FAST", "FLOP", "ATTACK"),),
    "ray": (("IDLE", "SWIM_IDLE_EVENT"),),
    "jellyfish": (("IDLE", "SWIM", "LAND"),),
    "starfish": (("IDLE",),),
    "crab": (("IDLE", "WALK", "HIDE_LOOP", "SIT_LOOP", "HIDE", "UNHIDE", "SWING_RIGHT", "DANCE", "HIDE_PEEK"), ("IDLE", "WALK", "SIT_IDLE")),
    "clam": (("OPEN", "CLOSE", "IDLE_OPEN", "IDLE_CLOSED"),),
    "giant_isopod": (("IDLE", "WALK", "SWIM", "HIDE_START", "HIDE_END"),),
    "blobfish": (("GRAY_SWIM", "PINK_IDLE"),),
    "desert_scorpion": (("IDLE", "WALK", "RUN", "ATTACK"),),
    "jungle_scorpion": (("IDLE", "WALK", "RUN", "ATTACK"),),
    "komodo_dragon": (("IDLE", "WALK", "BITE", "BABY_TRANSFORM", "BASK"),),
    "bear": (("IDLE", "WALK", "RUN", "SLEEP", "SIT", "SIT_IDLE", "SNIFF", "EAT", "ATTACK"),
             ("IDLE", "WALK", "RUN", "SLEEP", "SIT", "SIT_IDLE", "SNIFF")),
    "black_bear": (("IDLE", "WALK", "RUN", "SLEEP", "SIT", "SNIFF", "EAT", "ATTACK"),
                   ("IDLE", "WALK", "RUN", "SLEEP", "SIT", "SIT_IDLE", "SNIFF")),
    "giraffe": (("IDLE", "WALK", "RUN", "BABY_TRANSFORM", "EAT"), ("IDLE", "WALK", "RUN")),
    "rhino": (("IDLE", "WALK", "RUN", "ATTACK", "FOOT", "STUNNED"),),
    "lizard": (("SIT", "WALK"),),
    "lizard_tail": (("FLOP",),),
    "boar": (("IDLE", "WALK", "RUN", "ATTACK"), ("IDLE", "WALK", "RUN")),
    "capybara": (("WALK", "SWIM", "SIT", "SLEEP", "SLEEP2"), ("IDLE", "WALK", "RUN", "SIT", "SLEEP")),
    "turkey": (("IDLE", "WALK", "RUN", "PICKING_ON_GROUND"),),
    "lion": (("IDLE", "WALK", "RUN", "PREY", "SLEEP", "SLEEP2", "ATTACK"), ("IDLE", "WALK", "RUN", "SLEEP", "SIT_IDLE")),
    "rat": (("IDLE", "WALK", "RUN", "SWIM", "STANDING", "SLEEP", "SIT"),),
    "snake": (("BELLIED", "CLIMB", "SLEEP", "TONGUE", "ATTACK", "RATTLE", "IDLE", "MOVE"),),
    "tiger": (("IDLE", "WALK", "RUN", "PREY", "SLEEP", "SLEEP2", "ATTACK"), ("IDLE", "WALK", "RUN", "SLEEP")),
    "tortoise": (("IDLE", "WALK", "SIT", "HIDE", "DIG", "HURT"), ("IDLE", "WALK", "SIT")),
    "bird": (("IDLE", "WALK", "FLY"), ("IDLE", "WALK")),
    "butterfly": (("FLY",),),
    "caterpillar": (("IDLE", "CRAWL"),),
    "dragonfly": (("FLY",),),
    "vulture": (("FLY", "SIT"),),
    "snail": (("IDLE", "CRAWL", "HIDE_START", "HIDE_END"),),
    "firefly": (("FLY",), ("IDLE", "WALK")),
    "duck": (("IDLE", "WALK", "SWIM", "SIT", "FLAP"), ("IDLE", "WALK", "SWIM", "SIT_IDLE")),
}[species]
java_name = species.title().replace("_", "")
model_folder = {"dirt_trail": "mole", "mammoth": "elephant", "great_white_shark": "greatwhiteshark", "giant_isopod": "giantisopod", "anglerfish": "fish", "ray": "fish", "black_bear": "bear", "bass": "fish", "catfish": "fish", "piranha": "fish", "blobfish": "fish", "caterpillar": "butterfly", "desert_scorpion": "scorpion", "jungle_scorpion": "scorpion", "lizard_tail": "lizard", "komodo_dragon": "komododragon", "zebra": "equine"}.get(species, species)
origin = root / "common/src/main/java/com/crispytwig/naturalist/client/model/animal" / model_folder
model_files = {
    "mole": [("MoleModel", "adult")],
    "snake": [("SnakeModel", "adult")],
    "dirt_trail": [("DirtTrailModel", "adult")],
    "rat": [("RatModel", "adult")],
    "great_white_shark": [("GreatWhiteSharkModel", "adult")],
    "hedgehog": [("HedgehogModel", "adult")],
    "anglerfish": [("AnglerfishModel", "adult")],
    "ray": [("RayModel", "adult")],
    "jellyfish": [("JellyfishModel", "adult")],
    "starfish": [("StarfishModel", "adult")],
    "crab": [("CrabModel", "adult"), ("CrabBabyModel", "baby")],
    "clam": [("ClamModel", "adult")],
    "giant_isopod": [("GiantIsopodModel", "adult")],
    "vulture": [("VultureModel", "adult")],
    "snail": [("SnailModel", "adult")],
    "blobfish": [("BlobfishPinkModel", "pink"), ("BlobfishGrayModel", "gray")],
    "rhino": [("RhinoModel", "adult")],
    "piranha": [("PiranhaModel", "adult")],
    "komodo_dragon": [("KomodoDragonModel", "adult")],
    "lizard": [("LizardModel", "adult")],
    "lizard_tail": [("LizardTailModel", "adult")],
    "catfish": [("CatfishModel", "adult")],
    "desert_scorpion": [(f"{java_name}Model", "adult")],
    "jungle_scorpion": [(f"{java_name}Model", "adult")],
    "butterfly": [("ButterflyModel", "adult")],
    "caterpillar": [("CaterpillarModel", "adult")],
    "dragonfly": [("DragonflyModel", "adult")],
    "turkey": [("TurkeyModel", "adult")],
    "bass": [("BassModel", "adult"), ("MediumBassModel", "medium"), ("LargeBassModel", "large")],
}.get(species, [(f"{java_name}Model", "adult"), (f"{java_name}BabyModel", "baby")])
lines = [
    "// Based on Naturalist 2.0.7 Java model source (MIT).",
    "use horizon_client::mobs::model_layers::{CubeDeformation, CubeListBuilder, LayerDefinition, MeshDefinition, ModelPart, PartPose};",
]


def floats(text):
    return [re.sub(r"[fF]$", "", n.strip()) for n in text.split(",")]


def array(items):
    return "[" + ", ".join(items) + "]"


for java, rust in model_files:
    text = (origin / f"{java}.java").read_text()
    parts = list(re.finditer(
        r'PartDefinition (\w+) = (\w+)\.addOrReplaceChild\("([^"]+)",\s*'
        r'CubeListBuilder\.create\(\)(.*?),\s*PartPose\.(\w+)(?:\(([^)]*)\))?\);',
        text, re.S))
    assert len(parts) == text.count(".addOrReplaceChild("), java
    lines.extend([f"pub(super) fn {rust}() -> ModelPart {{",
                  "    let mut mesh = MeshDefinition::new();",
                  "    let partdefinition = mesh.root();"])
    for part in parts:
        variable, parent, label, boxes, pose, xyz = part.groups()
        tokens = re.compile(
            r'\.texOffs\(([^)]+)\)|\.mirror\(([^)]*)\)|'
            r'\.addBox\(([^\n]*?),\s*new CubeDeformation\(([^)]*)\)\)')
        line = f'    let {variable} = mesh.add_or_replace_child({parent}, "{label}", CubeListBuilder::create()'
        for token in tokens.finditer(boxes):
            uv, mirror, xyzw, deform = token.groups()
            if uv is not None:
                line += f".tex_offs({uv})"
            elif mirror is not None:
                line += f".set_mirror({mirror or 'true'})"
            else:
                sides = floats(xyzw)
                assert len(sides) == 6, token.group()
                grow = floats(deform)
                deformation = (f"CubeDeformation::new({grow[0]})" if len(grow) == 1
                               else f"CubeDeformation::xyz({', '.join(grow)})")
                line += f".add_box_with({array(sides[:3])}, {array(sides[3:])}, {deformation})"
        assert boxes.count(".addBox(") == sum(x.group(3) is not None for x in tokens.finditer(boxes)), label
        pose_fn = {"offset": "offset", "offsetAndRotation": "offset_and_rotation", "ZERO": "ZERO"}[pose]
        line += f", PartPose::{pose_fn}" + (f"({', '.join(floats(xyz))})" if xyz else "") + ");"
        lines.append(line)
    dims = re.search(r"LayerDefinition\.create\(meshdefinition, (\d+), (\d+)\)", text)
    lines += [f"    LayerDefinition::create(mesh, {dims[1]}, {dims[2]}).bake_root()", "}"]
out.write_text("\n".join(lines) + "\n")

animations = root / "common/src/main/java/com/crispytwig/naturalist/client/animation/definitions"
lines = ["// Keyframes translated from Naturalist 2.0.7 Java source (MIT).",
         "use super::{Clip, Channel, Interpolation, Target};"]
animation_name = "Elephant" if species == "mammoth" else java_name
for file, prefix, names in zip((f"{animation_name}Animation", f"{animation_name}BabyAnimation"),
                               ("ADULT", "BABY"), clips):
    text = (animations / f"{file}.java").read_text()
    for name in names:
        prefix_name = {"OstrichBabyAnimation": "OSTRICH_BABY", "ElephantAnimation": "ELEPHANT", "ElephantBabyAnimation": "ELEPHANT", "GreatWhiteSharkAnimation": "SHARK", "WhaleBabyAnimation": "WHALE_BABY", "BirdAnimation": "PASSERINE", "LizardTailAnimation": "REPTILE_TAIL", "BlackBearAnimation": "BEAR", "BlackBearBabyAnimation": "BEAR", "CrabBabyAnimation": "CRAB_BABY", "TigerBabyAnimation": "TIGER_BABY"}.get(file, species.upper())
        clip = re.search(rf'{prefix_name}_{name} = AnimationDefinition.Builder.withLength\(([^)]*)\)'
                         r'(\.looping\(\))?(.*?)\.build\(\);', text, re.S)
        assert clip
        duration, looped, body = clip.groups()
        lines.append(f"pub(super) const {prefix}_{name}: Clip = Clip {{")
        lines.append(f"    duration: {floats(duration)[0]}, loops: {'true' if looped else 'false'}, channels: &[")
        for channel in re.finditer(
            r'\.addAnimation\("([^"]+)", new AnimationChannel\(AnimationChannel.Targets\.(\w+),(.*?)\)\)',
            body, re.S
        ):
            bone, target, keyframes = channel.groups()
            keys = re.findall(
                r'new Keyframe\(([^,]+), KeyframeAnimations\.(\w+)\(([^)]*)\), AnimationChannel.Interpolations\.(LINEAR|CATMULLROM)\)',
                keyframes,
            )
            assert keys and len(keys) == keyframes.count("new Keyframe("), (file, bone)
            catmull_indices = [i for i, (_, _, _, method) in enumerate(keys) if method == "CATMULLROM"]
            if len(catmull_indices) == len(keys):
                interpolation = "CatmullRom"
            elif catmull_indices:
                interpolation = f"Mixed(&{catmull_indices})"
            else:
                interpolation = "Linear"
            lines.append(f'        Channel {{ bone: "{bone}", target: Target::{target.title()}, '
                         f'interpolation: Interpolation::{interpolation}, keys: &[')
            for time, kind, vector, _ in keys:
                assert kind in ("degreeVec", "posVec", "scaleVec"), kind
                lines.append(f"            ({floats(time)[0]}, {array(floats(vector))}),")
            lines.append("        ] },")
        lines.append("    ],\n};")
out.with_name(f"{species}_animations.rs").write_text("\n".join(lines) + "\n")
