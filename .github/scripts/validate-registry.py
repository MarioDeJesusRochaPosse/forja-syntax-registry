import json
from pathlib import Path

root = Path(__file__).resolve().parents[2]
registry = json.loads((root / "registry.json").read_text())
assert registry["schemaVersion"] == "1"
ids = set()
for language in registry["languages"]:
    assert language["id"] not in ids
    ids.add(language["id"])
    assert len(language["revision"]) == 40
    assert "/" in language["githubRepo"]
    query_path = root / language["queries"]["path"]
    assert query_path.is_relative_to(root)
    assert query_path.exists()
    assert language["license"]
print(f"validated {len(ids)} languages")
