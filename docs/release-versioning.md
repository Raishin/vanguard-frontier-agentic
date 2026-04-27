# Release and Versioning

This repository can be published as an npm package that carries a catalog of
agentic assets: skills, agents, rules, MCP references, schemas, docs, and
manifests.

## Semantic Versioning Policy

Use `MAJOR.MINOR.PATCH`.

### PATCH: safe fixes

Use a patch bump for changes that should not break consumers:

- typo fixes in docs or prompts,
- clearer wording without changing behavior,
- metadata corrections that preserve IDs and paths,
- adding missing official-doc links,
- regenerating manifests after non-behavioral edits,
- fixing validation scripts without changing the manifest format.

Example:

```bash
npm version patch
```

### MINOR: additive capabilities

Use a minor bump for backwards-compatible additions:

- adding a new skill, agent, rule, MCP reference, or asset,
- adding new metadata fields while preserving old fields,
- adding a new provider folder,
- adding new validation checks that current valid assets can pass,
- adding a new manifest section without removing existing fields.

Example:

```bash
npm version minor
```

### MAJOR: breaking changes

Use a major bump when consumers must adapt:

- renaming or removing skill IDs,
- moving cataloged paths without compatibility aliases,
- changing schema-required fields,
- changing manifest format incompatibly,
- removing catalog entries,
- changing package file layout,
- changing the trust/security contract of an asset in a way that alters how it
  should be used.

Example:

```bash
npm version major
```

## Pre-1.0 Reality

While the package is below `1.0.0`, npm and SemVer convention treat the API as
not fully stable. Be disciplined anyway:

- `0.1.x` patch: safe fixes.
- `0.x.0` minor: meaningful feature additions or breaking changes during early
  design.
- Call out breaking changes explicitly in release notes even before `1.0.0`.

Ruthless rule: do not hide a breaking catalog/schema change inside a patch
release just because the project is young.

## Release Gate

Before publishing:

```bash
npm run manifest:write
npm run validate
npm pack --dry-run
```

Review the `npm pack --dry-run` output. If secrets, local caches, or generated
junk appear in the package, stop and fix `files` or `.npmignore`.

## Skill Integrity Manifest

`catalog/skill-manifest.json` records SHA-256 hashes for every file under every
cataloged skill directory.

Use it for:

- detecting accidental edits,
- verifying copied skills,
- release review,
- npm package integrity checks,
- downstream installer trust decisions.

Workflow:

```bash
# After intentional skill edits
npm run manifest:write

# In CI or before publish
npm run manifest:check
```

The manifest proves file integrity for repository contents. It does not prove
that a skill is safe, correct, compliant, or officially endorsed.

