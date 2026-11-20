import argparse
import os
from pathlib import Path
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument("java_home", type=Path)
parser.add_argument("client_jar", type=Path)
parser.add_argument("naturalist_jar", type=Path)
parser.add_argument("client_libs", type=Path)
parser.add_argument("server_libs", type=Path)
parser.add_argument("output", type=Path)
args = parser.parse_args()
root = Path(__file__).resolve().parent
jars = [args.client_jar, args.naturalist_jar]
jars.extend(sorted(args.client_libs.rglob("*.jar")))
jars.extend(sorted(args.server_libs.rglob("*.jar")))
classpath = os.pathsep.join(map(str, jars))
subprocess.run([args.java_home / "bin/javac", "-proc:none", "-cp", classpath, "-d", root / "classes",
                root / "reference/BirdPoseProbe.java"], check=True)
result = subprocess.run([args.java_home / "bin/java", "-Djava.awt.headless=true", "-cp",
                         str(root / "classes") + os.pathsep + classpath, "BirdPoseProbe"],
                        text=True, capture_output=True)
if result.returncode:
    raise SystemExit(result.stderr)
args.output.write_text(result.stdout)
