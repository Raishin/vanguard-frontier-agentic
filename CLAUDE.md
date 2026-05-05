# Vanguard Frontier Agentic

This repository is a curated marketplace for **cloud**, **zero-trust**, and **compliance-aware** AI workflows.

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
- Run `npm run validate` before finishing. The pipeline runs seven gates:
  `validate:catalog`, `validate:aws`, `manifest:check`,
  `validate:allowed-tools`, `validate:skill-schema`, `validate:agent-schema`,
  `validate:links`. Markdownlint and codespell run separately as the
  `Docs Quality` workflow (advisory `npm run lint:docs`).
- If `skills/**` changed intentionally, also refresh
  `catalog/skill-manifest.json` with `npm run manifest:write`.
- Every `SKILL.md` must declare an `allowed-tools` field
  (least-privilege baseline) and conform to
  `schemas/skill.frontmatter.schema.json`.
- For agents that have a 1:1 companion skill, declare it explicitly via
  `companion_skills: [<skill-id>]` in the agent's `metadata.json` rather
  than relying on the name-stripping convention.

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

