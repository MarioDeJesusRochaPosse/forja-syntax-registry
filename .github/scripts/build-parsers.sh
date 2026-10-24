#!/usr/bin/env bash
set -euo pipefail

PLATFORM="${PLATFORM:?PLATFORM is required}"
OUT_DIR="${OUT_DIR:-dist}"
mkdir -p "$OUT_DIR"
EXT=so
[[ "$PLATFORM" == windows-* ]] && EXT=dll

PYTHON=python3
command -v "$PYTHON" >/dev/null 2>&1 || PYTHON=python
"$PYTHON" - <<'PY' "$PLATFORM" "$OUT_DIR" "$EXT"
import json, os, subprocess, sys, tempfile, urllib.request, zipfile
platform, out_dir, ext = sys.argv[1:]
with open("registry.json", encoding="utf-8") as f:
    registry = json.load(f)

for entry in registry["languages"]:
    name = entry["id"]
    repo = entry["githubRepo"]
    revision = entry["revision"]
    with tempfile.TemporaryDirectory() as tmp:
        archive = os.path.join(tmp, "source.tar.gz")
        url = f"https://github.com/{repo}/archive/{revision}.tar.gz"
        urllib.request.urlretrieve(url, archive)
        source = os.path.join(tmp, "source")
        os.makedirs(source)
        subprocess.run(["tar", "-xzf", archive, "-C", source], check=True)
        roots = [os.path.join(source, d) for d in os.listdir(source)]
        parser = next((os.path.join(root, "src", "parser.c") for root in roots if os.path.exists(os.path.join(root, "src", "parser.c"))), None)
        if not parser:
            raise SystemExit(f"parser.c not found for {name}")
        src_dir = os.path.dirname(parser)
        output = os.path.abspath(os.path.join(out_dir, f"{name}-{platform}.{ext}"))
        compiler = "clang"
        flags = ["-shared", "-O2"]
        if not platform.startswith("windows-"):
            flags.append("-fPIC")
        subprocess.run([compiler, *flags, parser, "-o", output], check=True)
        print(f"built {output}")
PY
