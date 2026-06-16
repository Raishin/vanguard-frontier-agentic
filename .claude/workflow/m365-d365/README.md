# Workflow Plan — Microsoft M365 / D365 Agentic Board

> Status: **PLAN ONLY — not yet implemented.** This directory is the execution
> plan for adding a Microsoft (M365, D365, Power Platform, Copilot) agent and
> skill board to `vanguard-frontier-agentic`. No agents, skills, or schema
> changes have been made yet. Read `00-discovery-and-thesis.md` first.

## Why this exists

A long architectural brief asked for a Fortune-50-grade Microsoft agent board
(maestros, ~30 specialist agents, 15 cross-functional protocols, 24 skill
packs). This plan converts that brief into **repo-accurate, sequenced,
gated work** instead of a wishlist. It is deliberately ruthless: every item is
tied to repo conventions that already exist, and the brief's assumptions that
do **not** hold against this repo are called out and corrected.

## The single most important finding

The brief's skill/agent templates declare `provider: microsoft`. **This repo's
schemas and validators reject that today.** `provider` is a closed enum/list that
must be extended in **seven** places (confirmed by exploration):

- `schemas/agent.schema.json` (provider enum)
- `schemas/skill.schema.json` (provider enum)
- `schemas/rule.schema.json` (provider enum — if MS rules are planned)
- `schemas/mcp-reference.schema.json` (provider enum — if MS MCP refs are planned)
- `tests/validate-catalog.py` (`ALLOWED_PROVIDERS` set)
- `scripts/generate-kiro-powers.mjs` (`PROVIDERS` object — then run `kiro-powers:write`)
- `scripts/generate-docs-data.mjs` (taxonomy group)

(`scripts/generate-plugin-manifest.mjs` and `generate-cursor-plugin.mjs` derive
providers dynamically from the catalog — no manual edit, just regenerate. The
catalogs `catalog/agents.json` / `catalog/skills.json` are generated, never
hand-edited.) There is also no `powers/vanguard-microsoft/` (auto-created by
`npm run kiro-powers:write` once the generator is updated). `npm run validate` runs 19+
gates (including `validate:agent-schema`, `validate:skill-schema`,
`validate:maestro-routing`, `validate:kiro-powers`, `validate:asset-integrity`,
`validate:readme-counts`). **Until `microsoft` is a first-class provider across
schemas + generators + powers, zero Microsoft assets can merge.** This is
Phase 0 and it gates everything.

Likewise, the brief's `category: microsoft` is invalid — `category` is a fixed
taxonomy enum in `schemas/skill.frontmatter.schema.json` (security, platform,
data, compliance, governance is NOT a value, etc.). Microsoft skills must map
onto existing category buckets. See `00-discovery-and-thesis.md`.

## Plan documents (read in order)

| File | Purpose |
|------|---------|
| `00-discovery-and-thesis.md` | Phase 0 repo discovery, brutal thesis, brief-vs-repo corrections, evidence ledger |
| `01-architecture-and-agent-board.md` | Maestro layer, full agent board table, scope/merge/kill decisions |
| `02-skill-packs-and-templates.md` | Skill pack inventory, repo-accurate folder tree, file templates |
| `03-routing-matrix-and-protocols.md` | Routing matrix (15 scenarios) + cross-functional protocols |
| `04-implementation-roadmap.md` | Phased, gated roadmap with per-phase exit criteria |
| `05-red-team-and-acceptance.md` | Eval-harness-style red-team scorecard + Fortune 50 acceptance gates |

## Evidence labels used throughout

`E0` assumption · `E1` user-provided · `E2` repo pattern observed (this repo)
· `E3` official Microsoft docs · `E4` Microsoft Learn cert/study guide ·
`E5` tenant telemetry required · `E6` implementation artifact required ·
`E7` board/audit evidence required.

> Context7 / Microsoft Learn MCP are available in this session but were **not**
> used to pin product behavior while authoring the *plan*; all Microsoft product
> claims in implementation phases are marked `E3 (verify)` and must be
> re-grounded against Microsoft Learn at build time. SDK/CLI syntax must be
> reverified via Context7 before any implementation artifact ships.

## How to execute this plan

1. Do Phase 0 (`04-implementation-roadmap.md`) — provider registration. **Hard gate.**
2. Build the 5 maestros + shared Microsoft rules (Phase 1).
3. Build the 6 highest-risk skills first (Phase 2), then high-value process skills.
4. Run the red-team eval (`05-red-team-and-acceptance.md`) and remediate to ≥4/5.
5. `npm run validate` must pass before every commit; refresh catalogs with
   `npm run manifest:write:all` after any cataloged-asset change.
