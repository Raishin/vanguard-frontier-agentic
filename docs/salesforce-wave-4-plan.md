# Salesforce Wave 4 — Operational T1 Tier Plan

> Status: PLANNED — execution in progress.
> Branch: `claude/salesforce-integration-6KE5h`
> Verified: 2026-05-21

## Genesis — Ruthless Mentor Findings

Wave 4 exists because four parallel investigations (Wave 3 retrospective) found
that Waves 1–3 built an excellent **governance review portfolio** with
zero **operational** capability. Specifically:

| Investigation | Verdict |
|---|---|
| `forcedotcom/sf-skills` (official, 57 skills) | We have 0 of the 6+ Apex/SOQL lifecycle skills, 0 Agentforce telemetry, 0 OmniStudio, 0 Data Cloud. 25 reference files total vs. their 657. |
| SyncGTM (RevOps-focused, 7 skills) | Every skill we have is a *review*. None solve daily admin pain (SOQL gen, flow debug, bulk ops, field mapping, validation rules). |
| MCP Market salesforce-assistant | We are MCP-blind. Every Salesforce skill is `Read Grep Glob` only — never reaches for the org. |
| Context7 MCP scope research | The least-privilege model is well-defined (T0/T1/T2/T3 + "Run As" service account). We have not implemented any of it. |

**Buyer translation:**
- Fortune 50 CISO: "Compelling governance lens, but where is the live evidence?"
- Fortune 50 RevOps leader: "Nothing here for my 8 sales-ops analysts and 3 admins."
- 11pm Salesforce admin firefighting a broken flow: "I'd pick salesforce-assistant. Your portfolio is a filing cabinet; I need a flashlight."

## Strategic Pivot — Add T1 Operational Tier

```
T0 Static Review (current 30 agents, 12 skills) — no MCP calls
  ↓ feeds sanitized data into
T1 Read-Only Operational (NEW — Wave 4) — MCP-aware
  - allowed-tools: Bash (scoped to `sf` CLI / MCP-proxy)
  - OAuth: api + refresh_token ONLY (no full, no web, no sfap_api)
  - "Run As" service account permissions:
    - REQUIRED: View Setup and Configuration, per-object Read FLS
    - DENIED: Modify All Data, View All Data (system), View Encrypted Data
    - DENIED: Modify Metadata, Manage Connected Apps, Author Apex
  ↓ proposes to
T2 Sandbox Dry-Run (CI/staging only, NEVER production)
  - `sf project deploy validate` (no commit)
  - Apex tests in sandbox only (View All Data acceptable on test-only account)
  - Strict org allowlist enforced at Connected App
  ↓ blocked by Live Guard from
T3 Production Mutation — PROHIBITED for agents
  - Production deploy commits, DML on prod records, ModifyAllData operations
  - Must be routed through synchronous human approval (Slack bot, Agentforce HITL)
```

## Wave 4 Deliverables (Priority Order)

### Group A — T1 Operational MCP Skills (5)

| Skill ID | Tier | OAuth Scope | Solves |
|---|---|---|---|
| `salesforce-soql-explorer-skill` | T1 | `api refresh_token` | Live record lookup — admin's #1 daily task |
| `salesforce-metadata-fetcher-skill` | T1 | `api refresh_token` | Feeds every existing review skill — kills the hand-paste requirement |
| `salesforce-apex-test-runner-skill` | T1 (sandbox) | `api refresh_token` | Test execution without dev ticket |
| `salesforce-agentforce-stdm-observer-skill` | T1 | `api refresh_token` + `cdp_query_api` | The CISO question: "is Agentforce working correctly in production?" |
| `salesforce-deployment-validator-skill` | T2 (sandbox dry-run) | `api refresh_token` | Pre-deployment change impact via `sf project deploy validate` |

### Group B — T0 Generation Skills (5)

| Skill ID | Tier | Solves |
|---|---|---|
| `salesforce-soql-generator-skill` | T0 | Plain English → SOQL (SyncGTM's #1 use case) |
| `salesforce-validation-rule-writer-skill` | T0 | Business rule English → formula syntax |
| `salesforce-field-mapping-skill` | T0 | CSV column → Salesforce API name w/ type mismatch detection |
| `salesforce-flow-debugger-skill` | T0 | Pasted error + flow export → diagnosis + corrected node config |
| `salesforce-bulk-data-ops-skill` | T0 (gen) / T2 (dry-run) | Owner reassign, dedup, mass update templates for Data Loader / Apex Anonymous |

### Group C — Schema Infrastructure

1. Extend `schemas/skill.frontmatter.schema.json` to add:
   - `execution_tier`: enum `[static-review | read-only-runtime | sandbox-mutating | production-prohibited]`
   - `mcp_servers`: array (optional) — explicit MCP server declarations
   - `oauth_scopes`: array (optional) — explicit allowed scopes
   - `run_as_permissions`: object (optional) — `required` + `denied` permission arrays
2. Extend `schemas/skill.schema.json` similarly for catalog entries
3. Document the tier model in `docs/execution-tiers.md`
4. Add new harness adapter pattern: `allowed-tools: Bash(sf:*)` for T1 skills

### Group D — sf-skills Pattern Adoption (cross-cutting)

For new Wave 4 skills:
- Mandatory `references/` subdirectory with minimum 3 reference files
- Mandatory explicit `TRIGGER when:` and `DO NOT TRIGGER when:` in description
- Mandatory delegation routing in SKILL.md "When This Skill Owns the Task"
- Mandatory scoring rubric (numeric, machine-checkable) for output quality
- Mandatory `license: MIT` field
- Optional: `examples/` with TRANSCRIPT.md before/after

## Defensible Differentiation to Preserve

These are areas where Vanguard exceeds sf-skills and should NOT be diluted:

- **Zero-trust NIST 800-207 framing** — `salesforce-zero-trust-maturity-skill`
- **Compliance/regulatory lens** — `salesforce-compliance-privacy-agent`, marketing-consent-review, infrastructure-audit
- **Multi-agent orchestration** — `salesforce-maestro-agent` + 30 specialists
- **Multi-harness delivery** — Wave 2 adapters (codex/copilot/cursor/gemini/kiro)
- **Permission model depth** — toxic combinations, ViewAllData on PII, guest-user exposure
- **Live Guard architecture** — `salesforce-live-guard-agent` + `salesforce-live-change-approval-protocol`

## Live Guard Integration

The new T1/T2 skills must integrate with Live Guard:
- T1 skills emit a structured audit envelope on every MCP call (operation,
  scope, org_id placeholder, timestamp, run_as_user_id placeholder)
- T2 skills (sandbox dry-run) must declare org allowlist and refuse if the
  target appears to be production
- Live Guard inherits authority — no T2 promotion to production without
  explicit human approval routed through Live Guard's protocol skill

## Out of Scope for Wave 4

Deferred to future waves:
- OmniStudio skills (5 sf-skills equivalents)
- Data Cloud skills (9 sf-skills equivalents)
- LWC component generation
- B2B Commerce store creation
- SLDS2 uplift
- T2 production-deploy gate (this is what Live Guard does)

## Success Criteria

- All 20 npm validate gates remain green
- Each new T1 skill declares `oauth_scopes: ["api","refresh_token"]` and
  `run_as_permissions.denied: ["ModifyAllData","ViewAllData","ViewEncryptedData","ModifyMetadata"]`
- Each new skill has a `references/` directory with ≥3 supporting docs
- Each new skill description has explicit `TRIGGER when:` and `DO NOT TRIGGER when:` clauses
- Live Guard agent's `metadata.json` is updated to declare authority over new T1/T2 skills

## Buyer-Facing Outcome

After Wave 4 the portfolio answers:
- "Can you query my org live?" → Yes (T1 soql-explorer)
- "Can you generate a SOQL query from English?" → Yes (T0 soql-generator)
- "Can you tell me if my Agentforce agent is working in prod?" → Yes (T1 stdm-observer)
- "Can you validate this deployment in my sandbox?" → Yes (T2 deployment-validator)
- "Can you commit to production?" → No, that requires human approval through Live Guard
