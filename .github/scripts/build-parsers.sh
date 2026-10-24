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
import json, os, shutil, subprocess, sys, tempfile, urllib.request, tarfile
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
        with tarfile.open(archive, "r:gz") as bundle:
            bundle.extractall(source, filter="data")
        roots = [os.path.join(source, d) for d in os.listdir(source)]
        parser = next((os.path.join(root, "src", "parser.c") for root in roots if os.path.exists(os.path.join(root, "src", "parser.c"))), None)
        if not parser:
            raise SystemExit(f"parser.c not found for {name}")
        src_dir = os.path.dirname(parser)
        output = os.path.abspath(os.path.join(out_dir, f"{name}-{platform}.{ext}"))
        compiler = shutil.which("clang")
        if not compiler and platform.startswith("windows-"):
            compiler = r"C:\Program Files\LLVM\bin\clang.exe"
        if not compiler or not os.path.exists(compiler):
            raise SystemExit("clang was not found on the build runner")
        flags = ["-shared", "-O2"]
        if not platform.startswith("windows-"):
            flags.append("-fPIC")
        sources = [parser]
        scanner = os.path.join(src_dir, "scanner.c")
        if os.path.exists(scanner):
            sources.append(scanner)
        subprocess.run([compiler, *flags, *sources, "-o", output], check=True)
        print(f"built {output}")
PY
