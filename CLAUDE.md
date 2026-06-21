# Vanguard Frontier Agentic

This repository is a curated marketplace for **cloud**, **zero-trust**, and **compliance-aware** AI workflows.

## Quick start

```bash
npm install                # one-time install
# ...make catalog/doc changes...
npm run validate           # 19+ gates (catalog, schema, asset integrity, routing, marketplace)
npm run lint:spell         # codespell — separate CI gate, NOT part of validate
npx --yes markdownlint-cli2 "**/*.md" "#node_modules"   # markdown lint — separate CI gate
```

If you touched the catalog (agents/skills/roles/providers), run `npm run manifest:write:all` then `npm run asset-integrity:write` **last, on its own** (see the ordering caveat at the bottom) before `npm run validate`.

## What this repo contains

- `skills/` — reusable workflows for recurring engineering tasks
- `agents/` — expert roles with judgment for review, architecture, and operations
- `rules/` — harness-specific operating guidance
- `mcp/` — MCP references and trust-boundary notes
- `catalog/` — machine-readable indexes
- `schemas/` — metadata contracts
- `docs/` — governance, taxonomy, compatibility, and quality guidance

## Operating stance

- Prefer **official docs** and **live evidence** over memory.
- Default to **least privilege**, **zero trust**, and **safe rollback paths**.
- Separate **verified facts**, **judgment**, **assumptions**, and **unknowns**.
- Treat broad permissions, destructive automation, and MCP mutation paths as high risk.
- Do not add secrets, credentials, tokens, tenant IDs, or customer data.

## When working in this repo

- Keep changes scoped and traceable to the task.
- Update catalog metadata when adding, moving, or removing cataloged assets.
- Run `npm run validate` before finishing. The pipeline runs 19+ validation gates covering catalog integrity, schema compliance, asset integrity, maestro routing, and multi-harness marketplace consistency.
- If `skills/**` changed intentionally, also refresh `catalog/skill-manifest.json` with `npm run manifest:write`.
- Every `SKILL.md` must declare an `allowed-tools` field (least-privilege baseline) and conform to `schemas/skill.frontmatter.schema.json`.
- For agents that have a 1:1 companion skill, declare it explicitly via `companion_skills: [<skill-id>]` in the agent's `metadata.json`.

## Documentation & Version Sync (DRY)

**Never hardcode counts, versions, or provider/role lists in documentation.**

After any catalog change (agents, skills, roles, providers):
```bash
npm run readme-counts:write      # Update README inline count markers
npm run docs-data:write          # Update Jekyll docs/_data/catalog.yml
npm run plugin-manifest:write    # Sync .claude-plugin version + agents
npm run cursor-plugin:write      # Sync .cursor-plugin version + agents
npm run kiro-powers:write        # Regenerate Kiro Powers for all providers
python3 tests/validate-asset-integrity.py --write  # Refresh SHA256 hashes
```

Or all-in-one: `npm run manifest:write:all`

**Version parity:** `package.json`, `.claude-plugin/plugin.json`, `.cursor-plugin/plugin.json` must always show the same version. The generators read from `package.json` automatically.

**Jekyll docs:** All pages in `docs/` use `{{ site.data.catalog.X }}` Liquid variables sourced from `docs/_data/catalog.yml`. Never hardcode agent/skill/provider counts in markdown.

**Hand-written provider lists are NOT auto-generated.** A few docs enumerate providers *by name* in prose/bullets/tables and must be updated by hand when a provider is added or removed: `docs/taxonomy.md` (provider bullet list), `docs/language-stack-boards.md` (board enumeration + tables), and the Powers table in `docs/integrations/installation-guide.md`. **Provider invariant:** `set(provider bullets in docs/taxonomy.md) == provider_list in docs/_data/catalog.yml == {distinct providers that have at least one agent}`. Skill-only providers (no agents) are not "boards" and must not be listed/counted — fix the miscategorization at the source (the skill's `provider` field) rather than inflating the metric.

**Releases:** semantic-release owns versioning. `feat:` → minor, `fix:` → patch. Never manually edit `"version"` in package.json.

## Adding a new provider

A `provider` value is hardcoded in several places that are **not** auto-derived from the catalog. When introducing a new provider (e.g. `sap`), update ALL of these or validation/CI will fail:

1. `schemas/agent.schema.json` — add to the `provider` enum.
2. `schemas/skill.schema.json` — add to the `provider` enum.
3. `tests/validate-catalog.py` — add to the `ALLOWED_PROVIDERS` set (a separate hardcoded list from the schemas — easy to miss; the `validate:catalog` gate fails without it).
4. `scripts/generate-docs-data.mjs` — add the provider to the correct category in the `taxonomy` array (drives `provider_taxonomy` in `catalog.yml`).
5. `scripts/generate-kiro-powers.mjs` — add a `PROVIDERS` entry **only if** the provider should ship a Kiro Power (optional; not every provider has one — e.g. netsuite/finance do not).
6. `docs/taxonomy.md` and `docs/language-stack-boards.md` — add the provider to the hand-written lists (see the provider invariant above).
7. Regenerate derived files: `npm run manifest:write:all` then `npm run docs-data:write`, then asset-integrity last (see the ordering caveat below).

## Adding a maestro / router agent

A `<provider>-maestro-agent` requires a routing fixture at `tests/fixtures/<provider>-maestro-routing/` containing `taxonomy.json` + `inputs/NN-name.json` + `expected/NN-name.json`. Every agent referenced must exist in `catalog/agents.json`. Generate the `expected/` files from the grader (`tests/validate-maestro-routing.py` → `evaluate(task, taxonomy)`) so they stay consistent, and list guarded-mutating-live agents under `live_guards` so they are never auto-dispatched (they only appear in `live-guard-gate` mode). The `validate:maestro-routing` gate enforces all of this.

## CI gates beyond `npm run validate`

`npm run validate` does **not** run spell-check or markdown lint — those are separate CI jobs that fail a PR independently. Before pushing, also run:

```bash
npm run lint:spell    # codespell. For false positives on real API names/acronyms
                      # (e.g. afterAll, AGS), add the lowercase term to
                      # ignore-words-list in .codespellrc — do NOT reword valid code.
npx --yes markdownlint-cli2 "**/*.md" "#node_modules"   # CI lints every markdown file
```

## Cross-platform asset rule

This repo supports multiple harnesses without pretending they are identical.

- Keep portable logic in canonical specs and shared docs.
- Keep harness-specific behavior in the right adapter format.
- Do not invent unsupported metadata fields in executable agent files.

## Important files

- `README.md` — human-facing vision and repository story
- `AGENTS.md` — compressed agent-focused repo guidance
- `CONTRIBUTING.md` — contributor onboarding and submission path
- `SECURITY.md` — vulnerability disclosure policy and SLA
- `CODE_OF_CONDUCT.md` — community standards
- `docs/compatibility.md` — harness support contract
- `docs/normalized-platform-matrix.md` — naming and platform normalization
- `docs/integrations/skills-cli.md` — install-path trust matrix
- `schemas/skill.frontmatter.schema.json` — required SKILL.md frontmatter contract
- `schemas/agent.schema.json` — agent metadata contract (includes `companion_skills`)

## Asset integrity (canonical reference)

`catalog/asset-integrity.json` holds SHA256 hashes of all tracked assets. If any file in `agents/`, `plugins/`, `.github/plugin/`, `package.json`, or a root file changes and the manifest is not refreshed, the `validate:asset-integrity` gate fails and blocks release automation. Regenerate it whenever you add/move/remove agents, plugins, or skills; edit `.github/workflows/release.yml`; or change any root-level file (README.md, AGENTS.md, CLAUDE.md, package.json, …):

```bash
python3 tests/validate-asset-integrity.py --write   # or: npm run asset-integrity:write
git add catalog/asset-integrity.json
```

**Ordering caveat:** `npm run manifest:write:all` runs its generators in parallel (`&` … `wait`), so `asset-integrity:write` can hash the tree *before* the other generators (README counts, plugin manifests, Kiro powers) finish writing files it covers. After `manifest:write:all`, always run `npm run asset-integrity:write` once more **on its own, last**, so it hashes the settled tree. Always run `npm run validate` before finishing to catch staleness early.
