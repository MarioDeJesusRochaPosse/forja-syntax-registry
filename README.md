# Forja Syntax Registry

Public registry for Forja Studio Tree-sitter grammars.

The registry keeps manifests, queries, scope mappings and fixtures as reviewable
text. GitHub Actions builds native parser libraries from pinned upstream commits:

- Linux x86_64: `.so`
- Windows x86_64: `.dll`

Compiled parsers are published as assets in the `grammars` GitHub Release. They
are never committed to this repository or embedded in Forja Studio.

## Layout

```text
languages/       pinned upstream grammar manifests
queries/         highlight queries
scopes/          Tree-sitter capture to Forja scope mappings
fixtures/        parser/highlight smoke tests
registry.json    public manifest
registry.lock.json resolved artifact metadata
.github/         validation and native build workflows
```

See [`registry.json`](registry.json) for the supported languages and
[`scopes/default.json`](scopes/default.json) for the semantic scope contract.

## Current catalog

The catalog is intentionally limited to pinned upstream repositories and
currently includes Rust, JSON, JavaScript, TypeScript, Python, Go, C, C++,
Java, Bash, Ruby, PHP, C#, TOML, YAML, Lua, Markdown and QML. Each entry records
the upstream Git revision, license, query path and artifact size limit.

Forja Studio reads this manifest from the public `main` branch, downloads the
matching `.so` or `.dll` from the `grammars` release, verifies `SHA256SUMS`,
downloads the query from the same registry, and persists both in its local
syntax cache. No compiled parser is committed to this repository or shipped
inside the desktop binary.
