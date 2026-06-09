# Plan: Oracle NetSuite Agent Ecosystem (Wave-1, 25 agents)

## Context

**Goal:** Build an enterprise-grade Oracle NetSuite agent + companion-skill domain inside
`vanguard-frontier-agentic`, following the repository's REAL conventions (not the AWS
AgentCore pattern the original brief assumed), with web-verified Oracle citations, least-
privilege roles, and a green `npm run validate`.

**Why:** The repo is a curated marketplace for cloud/zero-trust/compliance-aware AI
workflows. It already ships a `salesforce` provider (10 agents + 9 skills) as the exact
analog. NetSuite is the next ERP/cloud provider. The work must survive a hostile Fortune-50
review board and must not fabricate certification/release facts.

**Driving method (eval-harness / eval-driven development):** Validation gates are the
spec. We define "done" as passing `npm run validate` (19+ gates) + the adversarial scenario
suite BEFORE writing agents. Build to the gates, not to prose.

**Branch:** `feature/oracle-netsuite-agents` (already created off `claude/sleepy-bell-qpbvfu`).
Tooling installed earlier: `rtk` v0.42.3 (~/.local/bin), ECC full profile (~/.claude).

---

## CRITICAL FORENSICS FINDING (already verified by direct inspection)

The original brief's packaging spec is WRONG for this repo and would fail validation.
Do NOT use `agents/<id>/openai.yaml` or `references/least-privilege.md`. The REAL pattern,
copied from `agents/salesforce/*` and `skills/salesforce/*`:

### Agent layout — `agents/netsuite/<agent-id>/`
- `AGENT.md` — canonical agent spec (sections: purpose, when-to-use, refusal rules,
  escalation, evidence labels, output contract). Match salesforce house style.
- `metadata.json` — schema-bound (see fields below).
- `LEAST-PRIVILEGES.md` — UPPERCASE, hyphen-S. (NOT `least-privilege.md`.)
- `harnesses/` — SEVEN adapter files per agent:
  - `codex.toml`, `copilot.agent.md`, `claude-code.agent.md`, `cursor.agent.md`,
    `gemini.agent.md`, `kiro-ide.agent.md`, `kiro-cli.agent.json`
- Maestro agent also has `README.md` (routing table).

### Agent `metadata.json` fields (from real salesforce sample)
`id, name, type:"agent", provider:"netsuite", harnesses:[codex,copilot,claude-code,cursor,
gemini,kiro], harness_variants:{...7 paths...}, summary, source_type, official_docs:[urls],
security_notes, last_verified:"YYYY-MM-DD", path, companion_skills:[skill-id],
execution_tier:"static-review", lifecycle:"experimental", author:"github: Raishin",
version:"0.1.0"`

### Skill layout — `skills/netsuite/<skill-id>/`
- `SKILL.md` — MUST have `allowed-tools` frontmatter conforming to
  `schemas/skill.frontmatter.schema.json` (least-privilege baseline).
- `metadata.json` — schema-bound skill metadata.
- `references/*.md` — focused reference docs (official-sources, workflow-and-output,
  safety-checklist, control-matrix, release-drift, least-privilege, + topic-specific).

> EXACT schema field lists, frontmatter enums, harness adapter formats, and the catalog/
> validation pipeline are being confirmed by 3 background Explore agents
> (ids a88cf40ae95c122d5, afb1401d5b5cb7125, aec97df4e7e5db835). Fold their findings into
> the "Definition of Done" section before execution. If their output is lost (new session),
> re-run the same exploration against `schemas/` + `agents/salesforce/` + `tests/` +
> `package.json`.

---

## DEFINITION OF DONE (eval gates — build to these)

1. `npm run validate` passes all 19+ gates (catalog integrity, schema compliance, asset
   integrity, maestro routing, multi-harness consistency).
2. `catalog/skill-manifest.json` refreshed via `npm run manifest:write`.
3. Counts/docs regenerated: `npm run manifest:write:all` (covers readme-counts:write,
   docs-data:write, plugin-manifest:write, cursor-plugin:write, kiro-powers:write).
4. `python3 tests/validate-asset-integrity.py --write` run; `catalog/asset-integrity.json`
   committed.
5. Every `SKILL.md` declares `allowed-tools` and validates against the frontmatter schema.
6. Every agent with a 1:1 skill declares `companion_skills:[<skill-id>]`.
7. Provider `netsuite` registered everywhere `salesforce` is (provider list, platform
   matrix, docs data) — confirm exact files from Explore agent #3.
8. Adversarial scenario suite (Phase 4 of brief, 20 scenarios) documented with expected
   route/refusal per scenario; no scenario yields an Administrator-role or credential or
   live-mutation outcome.

---

## EVIDENCE / RESEARCH (must be real — NO fabrication)

Before writing agent content, fetch and cite with URL + verification date. The brief's
`[oaicite]` markers are NOT sources. Required verifications (WebFetch / WebSearch / Context7):

| Claim | Source to fetch |
|---|---|
| NetSuite cert tracks & "Coming Soon" status | NetSuite Certification Resource Center (live) |
| SOAP deprecation: new SOAP integrations end 2027.1; REST+OAuth2 default 2026.1 | NetSuite release notes / SuiteTalk docs |
| OAuth 2.0 supports REST/RESTlets/SuiteAnalytics Connect, NOT SOAP | NetSuite OAuth 2.0 help docs |
| TBA only where OAuth can't be used; no user creds for RESTlets/SOAP | NetSuite auth docs |
| 2FA required for Administrator + highly-privileged roles | NetSuite 2FA help doc |
| AI Connector cannot run under Administrator; needs MCP Server Connection + "Log in using OAuth 2.0 Access Tokens" perms | NetSuite AI Connector / MCP docs |
| Standard-roles-as-starting-point, create custom role | NetSuite standard roles doc |
| OAuth apps authorized separately in sandboxes | NetSuite sandbox/OAuth doc |

Produce an **Evidence Matrix**: claim | source URL | label (OFFICIAL_DOCUMENTATION) |
last_verified | release-sensitive (Y/N). Label release-sensitive items explicitly.

Cross-check against Oracle's upstream **SuiteCloud Agent Skills** repo (netsuite-ai-connector-
instructions, netsuite-owasp-secure-coding, netsuite-sdf-project-documentation,
netsuite-sdf-roles-and-permissions, netsuite-suitescript-records-reference,
netsuite-suitescript-upgrade, netsuite-uif-spa-reference). For each → classify reuse:
REFERENCE / DEPENDENCY / ADAPTED_WRAPPER / NO_ACTION, with attribution + license note.
Do NOT silently duplicate Oracle skills.

---

## AGENT BOARD (Wave-1, 25 agents) — 3 layers

**Layer 1 — Routing & Governance (5)**
- `netsuite-maestro-agent` (router; has README.md routing table)
- `netsuite-live-org-mutation-guard-agent` (blocks live mutations by default)
- `netsuite-evidence-release-drift-agent` (evidence labels + biannual release drift)
- `netsuite-enterprise-architecture-agent`
- `netsuite-audit-controls-sox-agent`

**Layer 2 — Domain Specialists (cert-aligned + enterprise-role) (18)**
Cert-aligned: `netsuite-suitefoundation-agent`, `netsuite-administrator-agent`,
`netsuite-erp-consultant-agent`, `netsuite-financial-foundations-agent`,
`netsuite-bi-reporting-agent`, `netsuite-saved-searches-workbook-agent`,
`netsuite-application-developer-agent`, `netsuite-web-services-integration-agent`,
`netsuite-suitecloud-developer-agent`, `netsuite-ai-foundations-agent`.
Enterprise-role: `netsuite-identity-access-role-permission-agent`,
`netsuite-sso-oauth-tba-agent`, `netsuite-sdf-devops-release-agent`,
`netsuite-suitescript-secure-code-review-agent`, `netsuite-suiteflow-automation-agent`,
`netsuite-oneworld-multisubsidiary-agent`, `netsuite-data-governance-privacy-agent`,
`netsuite-ai-connector-mcp-agent`, `netsuite-integration-migration-agent`,
`netsuite-sandbox-nonproduction-governance-agent`.

(Note: that's 20 in Layer 2 → total 25. Layer 3 = shared escalation/evidence/security/
financial-control/environment/release-drift PROTOCOLS embedded in skills, not separate agents.)

**Consolidation review:** before building, run the agent-boundary attack — confirm each
agent has a unique remit, inputs, outputs, escalation, refusal rules. Merge/defer any
overlaps (candidate overlaps to scrutinize: saved-searches-workbook vs bi-reporting;
web-services-integration vs integration-migration vs sso-oauth-tba). Record APPROVE/MERGE/
DEFER/SPLIT/REJECT per agent. A smaller well-bounded board beats sprawl.

Each agent: 1:1 companion skill under `skills/netsuite/<agent>-skill/` (or shared where a
merge happens). Every agent declares `companion_skills`.

---

## CROSS-CUTTING SAFETY RULES (embed in every agent + skill)

- Static review / advisory ONLY. NO live NetSuite mutations (no workflow activation, no SDF
  deploy, no data edits, no saved-search publish, no permission changes, no cert rotation)
  absent a separate authorized live-op protocol.
- Least privilege by default; never depend on Administrator. Custom roles from standard
  roles. 2FA noted for privileged roles. AI Connector never under Administrator.
- Auth posture: OAuth 2.0 for REST/RESTlets/SuiteAnalytics Connect; TBA only as fallback;
  no user credentials; SOAP = migration risk (timeline cited).
- Refusal triggers: requests for credentials/tokens/Administrator role; live mutations w/o
  approval; coming-soon cert claimed as available; broad MCP tool grants; cross-subsidiary
  data exposure.
- Evidence hierarchy in every SKILL.md: LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED
  > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED.
- Context7 MCP registered as the live-docs provider; agents call it for code/library lookups
  instead of memory.

---

## EXECUTION ORDER (incremental, resumable)

1. **Confirm conventions** — ingest 3 Explore agent reports; lock exact schema field lists,
   frontmatter enums, 7 harness adapter templates, and the catalog/provider-registration
   file list. Update "Definition of Done" with exact paths.
2. **Research pass** — real web/Context7 verification → build Evidence Matrix + Oracle
   upstream reuse matrix. (Blocks all content writing.)
3. **Board consolidation** — finalize the agent list (apply merges/defers), write the
   routing table for the maestro.
4. **Build a vertical slice FIRST (de-risk):** maestro + live-mutation-guard + one finance
   specialist + their skills, ALL files, then run `npm run validate`. Fix gate failures.
   This proves the template before mass production.
5. **Mass-produce remaining agents** in layer order (governance → specialists), each with
   full file set + companion skill. Re-run validate after each layer.
6. **Regenerate** all manifests/counts/asset-integrity (`npm run manifest:write:all`,
   `manifest:write`, asset-integrity `--write`).
7. **Adversarial board pass** — run the 20 scenarios + 6 review passes; score 0-5 per
   category; issue verdict (APPROVE / APPROVE-WITH-REMEDIATIONS / BLOCK). Apply remediation
   patches.
8. **Final validate + commit + push** to `feature/oracle-netsuite-agents`. (No PR unless
   user asks.)

---

## Verification

- `npm run validate` → all gates green (primary eval gate).
- `npm run manifest:write` then `git diff --exit-code catalog/skill-manifest.json` → no drift.
- `python3 tests/validate-asset-integrity.py --write` → asset-integrity clean.
- Spot-check: a new `netsuite-*` agent resolves through maestro routing validation.
- Adversarial suite: each of 20 scenarios documented with expected route + refusal/escalation
  + residual risk; zero Administrator/credential/live-mutation outcomes.

## Open items to resolve at execution start
- Exact skill `metadata.json` schema fields + `allowed-tools` enum (Explore #1).
- Exact 7 harness adapter file formats — hand-written vs generated by a `scripts/` generator
  (Explore #2). If a generator exists, USE IT rather than hand-authoring 175 harness files.
- Exact provider-registration file set + catalog regeneration order (Explore #3).
- Final consolidated agent count after boundary attack (may drop below 25 if merges win).
