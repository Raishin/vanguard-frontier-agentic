# Release and Versioning

The repository ships **one published artifact** to **multiple marketplaces** (npm, Claude Code plugins, Claude Code marketplace catalog, Cursor plugins, Codex plugins, the Copilot marketplace). To keep every consumer on the same number, the version is **computed**, not hand-edited.

This document is the contract. If a file disagrees with this contract, the file is wrong.

---

## Single source of truth

`package.json` field `version` is **the** version of the marketplace. Every other file that carries a version is derived from it.

| Surface | File | How it stays in sync |
|---|---|---|
| npm package | `package.json` | Bumped by `@semantic-release/npm` |
| Claude Code plugin | `.claude-plugin/plugin.json` | Synced by `scripts/release-prepare.mjs` |
| Claude Code marketplace catalog | `.claude-plugin/marketplace.json` (`metadata.version`) | Synced by `scripts/release-prepare.mjs` |
| Cursor plugin | `.cursor-plugin/plugin.json` | Synced by `scripts/release-prepare.mjs` |
| Codex plugin | `plugins/vanguard-frontier-agentic/.codex-plugin/plugin.json` | Synced by `scripts/release-prepare.mjs` |
| Copilot marketplace | `.github/plugin/marketplace.json` | Synced by `scripts/release-prepare.mjs` |
| Security policy banner | `SECURITY.md` | Synced by `scripts/release-prepare.mjs` |
| Asset integrity manifest | `catalog/asset-integrity.json` | Regenerated after sync (`validate-asset-integrity --write`) |
| Changelog counts | `CHANGELOG.md` | Recomputed from live catalog during prepare |

Every file above is listed in `.releaserc.js` under `@semantic-release/git` `assets`, so the `chore(release): X.Y.Z [skip ci]` commit captures the synchronized state atomically.

Parity is enforced by `tests/validate-plugin-manifest.py`: it fails the build if `plugin.json` or `marketplace.json`'s `metadata.version` drifts from `package.json`.

**Never edit a version string by hand outside of `package.json`.** If you do, the validate gate will fail.

---

## How the next version is computed

Conventional commits + `@semantic-release/commit-analyzer` (preset `conventionalcommits`), with these extra rules from `.releaserc.js`:

| Commit type | Bump |
|---|---|
| `feat:` | **minor** |
| `fix:` | patch |
| `security:` | patch |
| `perf:` | patch |
| `refactor:` | patch |
| `build:` | patch |
| `revert:` | patch |
| `BREAKING CHANGE:` in body / `!` after type | **major** |
| `chore:`, `docs:`, `test:`, `ci:`, `style:` | none |
| Any scope of `(no-release)` | none (explicit suppression) |

`semantic-release` walks every commit since the last released tag, picks the highest bump it sees, writes it to `package.json`, runs `scripts/release-prepare.mjs` to sync every derived file, commits the result, tags it, and publishes to npm with OIDC trusted publishing + Sigstore provenance.

### What this PR will publish

The current version is always in `package.json` (source of truth).

When a feature branch merges to `master`, `semantic-release` inspects all commits since the last released tag:
- If it finds any `feat:` commits, it bumps the **minor** version.
- If it finds only `fix:` commits, it bumps the **patch** version.
- If it finds only `docs:`, `chore:`, `test:`, etc., it does not release.

Example: if the current version is `2.9.0` and the branch contains `feat:` commits:

> `semantic-release` will compute and publish **`v2.10.0`** automatically.

You don't have to (and **must not**) bump anything by hand. If you do, semantic-release will either ignore your bump (and pick the correct one) or fail the `chore(release)` push because the tree is dirty.

### Forcing or suppressing a release

| Intent | How |
|---|---|
| Force a major | Add `BREAKING CHANGE: <reason>` in the commit body, or use `feat!:` / `fix!:` |
| Suppress release for an otherwise-bumping commit | Use scope `(no-release)`, e.g. `feat(no-release): wip` |
| Re-run the pipeline on master without new commits | GitHub → Actions → **Release** → *Run workflow* (idempotent) |
| Verify what would happen | Run the workflow with `dry_run: true` |

---

## What you should NOT do

- Do **not** bump `package.json` in a feature PR. Semantic-release owns it.
- Do **not** add a new file with a hardcoded version string unless you also add it to `scripts/release-prepare.mjs` **and** the `.releaserc.js` git-assets list **and** add a parity assertion.
- Do **not** commit `chore(release): ...` from your local machine. Only the `Release` workflow on `master` is allowed to produce that commit.
- Do **not** push a tag manually. Tags are created by `@semantic-release/github`.

---

## Adding a new versioned surface

If you publish to a new marketplace (new IDE harness manifest, packaging format, etc.) and that file carries a version:

1. **`scripts/release-prepare.mjs`**:
   - Top-level version field → add the path to `VERSION_PINNED_PLUGINS`.
   - Nested version field → add a `{ path, key }` entry to `VERSION_PINNED_NESTED` (key is a dot-path, e.g. `"metadata.version"`).
2. **`.releaserc.js`**: add the file to the `@semantic-release/git` `assets` array so the release commit captures it.
3. **`tests/validate-plugin-manifest.py`** (or a dedicated validator): add a version-parity assertion against `package.json`.
4. Run `npm run validate` and confirm the new gate passes.

If a marketplace file is not wired through all four steps, it will go stale silently — exactly the failure this contract exists to prevent.

---

## Verifying an installed release

```bash
# npm
npm view @raishin/vanguard-frontier-agentic version

# Claude Code marketplace catalog
jq -r '.metadata.version' .claude-plugin/marketplace.json

# Asset-integrity manifest cross-check
jq -r '.metadata.version // .version' catalog/asset-integrity.json
```

All three should always agree. If they don't, the validate gate will say so before the release ever ships.

---

## Skill integrity manifest

`catalog/skill-manifest.json` records SHA-256 hashes for every file under every cataloged skill directory.

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

The manifest proves file integrity for repository contents. It does not prove that a skill is safe, correct, compliant, or officially endorsed.

---

## Semantic semantics (the meaning of bumps)

| Bump | When | Examples |
|---|---|---|
| **PATCH** | Backwards-compatible internal change | Typo fixes; clearer prompt wording without behavior change; metadata corrections preserving IDs/paths; adding missing official-doc links; regenerating manifests after non-behavioral edits; fixing a validator without changing manifest format |
| **MINOR** | Backwards-compatible addition | New skill, agent, rule, MCP reference, asset; new metadata fields preserving existing fields; new provider folder; new validation checks that current valid assets pass; new manifest section without removing existing fields |
| **MAJOR** | Breaking change for consumers | Renaming or removing skill IDs; moving cataloged paths without aliases; changing schema-required fields; changing manifest format incompatibly; removing catalog entries; changing package file layout; tightening the trust/security contract of an asset |

Rule: do not hide a breaking catalog/schema change inside a patch release. If consumers must adapt, bump major.
