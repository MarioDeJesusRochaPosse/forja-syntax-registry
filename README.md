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
