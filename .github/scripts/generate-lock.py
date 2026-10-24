import hashlib
import json
from pathlib import Path

root = Path(__file__).resolve().parents[2]
dist = root / "dist"
registry = json.loads((root / "registry.json").read_text())
packages = {}
for language in registry["languages"]:
    artifacts = {}
    for artifact in sorted(dist.glob(f"{language['id']}-*.*")):
        if artifact.name == "SHA256SUMS":
            continue
        platform = artifact.name.removeprefix(language["id"] + "-").rsplit(".", 1)[0]
        digest = hashlib.sha256(artifact.read_bytes()).hexdigest()
        artifacts[platform] = {
            "target": platform,
            "url": f"https://github.com/MarioDeJesusRochaPosse/forja-syntax-registry/releases/download/grammars/{artifact.name}",
            "sha256": digest,
            "sizeBytes": artifact.stat().st_size,
        }
    query = root / language["queries"]["path"]
    packages[language["id"]] = {
        "id": language["id"],
        "version": language["version"],
        "revision": language["revision"],
        "artifacts": artifacts,
        "queriesSha256": hashlib.sha256(query.read_bytes()).hexdigest(),
    }

(root / "registry.lock.json").write_text(json.dumps({
    "lockfileVersion": 1,
    "releaseTag": "grammars",
    "packages": packages,
}, indent=2) + "\n")
