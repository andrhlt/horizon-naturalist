"""Shared setup for headless fixtures using the pinned external Naturalist jar."""
import hashlib
import shutil
from pathlib import Path

JAR_SHA512 = "b1a220dc6bcb6101911226a57d75d62f2797461c3e529e8cf7110673b69632538c4a129b135f9572411cbc60336c096e056488fbe1528638196c247eef6492cd"

def prepare_home(home: Path, jar: Path) -> None:
    if hashlib.sha512(jar.read_bytes()).hexdigest() != JAR_SHA512:
        raise ValueError("expected the pinned Naturalist 2.0.7 NeoForge 26.3 jar")
    jars = home / "jars"
    jars.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(jar, jars / f"{JAR_SHA512}.jar")
    (home / "horizon-mods.txt").write_text("mod.naturalist:true\n")
