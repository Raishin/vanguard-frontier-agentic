# Execution Tiers — Least-Privilege Contract for Skills and Agents

> Status: stable contract from Wave 4 onward.
> Verified: 2026-05-21

This document defines the **execution tiers** that every skill and
agent in the marketplace must declare — the four skill tiers (T0–T3)
below, plus the agent-level `mutating-runtime` tier used by gated
live-guards (see [Live-Guard Mutating Runtime](#agent-level-tier--mutating-runtime-phase-b-live-guards)).
The tier model exists to make blast radius and authorization explicit,
machine-checkable, and enforceable at the harness layer.

## Tier Definitions

### T0 — Static Review (`execution_tier: "static-review"`)

**Posture.** No network egress. No CLI invocation against any external
service. The skill or agent operates entirely on files and content
provided in the conversation.

**Allowed tools.** Read, Grep, Glob. No Bash unless restricted to local
file operations on the agent's working tree.

**Authentication.** None — no OAuth scopes, no credentials, no MCP
servers.

**Run As account permissions.** Not applicable.

**Blast radius.** Zero by construction.

**Typical use cases.** Adversarial reviewers, schema validators,
dependency-graph analysis from checked-in source, governance audits
from sanitized exports, code-review agents.

**Examples in this repo.**
- `salesforce-org-assessment-skill`
- `salesforce-permission-model-review-skill`
- `salesforce-agentforce-risk-review-skill`
- all 30 Salesforce specialist agents from Waves 1–3

### T1 — Read-Only Runtime (`execution_tier: "read-only-runtime"`)

**Posture.** Read-only operational access to a connected external system.
The skill or agent can fetch live data and metadata. It cannot mutate
records, modify metadata, deploy artifacts, or run privileged operations.

**Allowed tools.** Bash with an allowlist constrained to read-only CLI
patterns. For Salesforce: `Bash(sf data query:*)`, `Bash(sf org list:*)`,
`Bash(sf org display:*)`, `Bash(sf sobject describe:*)`, etc. Never
`Bash(*)`.

**Authentication.** Minimum OAuth scopes only. For Salesforce: `api` +
`refresh_token`. Never `full`, `web`, `chatbot_api`, or `sfap_api`
unless explicitly justified.

**Run As account permissions (Salesforce).**
- **REQUIRED:** View Setup and Configuration, per-object Read FLS
  (scoped to the objects the skill legitimately needs)
- **DENIED:** Modify All Data, View All Data (system permission),
  View Encrypted Data, Modify Metadata Through Metadata API
  Functions, Author Apex, Customize Application, Manage Connected
  Apps

**Blast radius.** Bounded by the Run As account's record-level sharing
chain (OWD + sharing rules + manual sharing). Outputs are sanitized —
org IDs, user IDs, encrypted field values are redacted before being
emitted.

**Mandatory controls.**
- Audit envelope on every CLI invocation
- Org allowlist enforced at the Connected App layer
- Refresh-token rotation cadence documented
- Output sanitization rules documented in `references/sanitization-rules.md`

**Typical use cases.** Live SOQL exploration, metadata fetching,
production telemetry observation, audit log reading, test coverage
inspection.

**Examples in this repo (Wave 4).**
- `salesforce-soql-explorer-skill`
- `salesforce-metadata-fetcher-skill`
- `salesforce-agentforce-stdm-observer-skill`

### T2 — Sandbox Mutating (`execution_tier: "sandbox-mutating"`)

**Posture.** Mutation is allowed only when reversible or never
committed. Production targets are REFUSED at the skill level. Common
patterns: dry-run deployment validation, Apex test execution in
sandboxes, data masking in non-production environments.

**Allowed tools.** Bash with allowlist; mutation verbs permitted only
when the command flag set guarantees no production commit (e.g.,
`sf project deploy validate` — validation only, no commit).

**Authentication.** Same as T1 — `api refresh_token` only. T2 does NOT
imply elevated OAuth scopes. The mutation capability comes from the
Run As account having `Deploy (Metadata API)` permission in a
sandbox-only allowlisted Connected App.

**Run As account permissions (Salesforce).**
- **REQUIRED:** Deploy (Metadata API), Modify Metadata Through Metadata
  API Functions (narrower than Modify All Data), View All Data on the
  sandbox-only service account (acceptable because sandbox data is
  disposable)
- **DENIED on production-eligible service accounts:** Modify All Data,
  Customize Application, Manage Connected Apps

**Mandatory controls.**
- Production org detection and HARD REFUSAL
- Sandbox-only Connected App org allowlist
- Audit envelope on every CLI invocation
- Handoff to T0 review skills and the Live Guard agent for any
  promotion proposal

**Typical use cases.** Pre-deployment validation, change impact
analysis, Apex test runs in sandbox, sandbox data masking dry-runs.

**Examples in this repo (Wave 4).**
- `salesforce-deployment-validator-skill`

### T3 — Production Mutation (PROHIBITED for agents)

**Posture.** Production mutations are NEVER agent-callable. Every
production-affecting operation must be routed through a synchronous
human-in-the-loop approval flow.

**Examples of T3-prohibited operations.**
- Production deployment commits (`sf project deploy start --target-org
  <production>`)
- DML on production records (any CREATE/UPDATE/DELETE in a production
  org)
- Connected App configuration with `full` scope
- Any operation requiring `Modify All Data` on a production org
- Any access to encrypted field values via `View Encrypted Data`

**Enforcement.** The Live Guard agent and Live Change Approval Protocol
skill exist to gate every T3 operation. Skills must REFUSE the
operation and emit an escalation to Live Guard with the proposed
change envelope. Live Guard routes the request through human
approval (Slack approval bot, Agentforce Human-in-the-Loop, or
equivalent) before any T3 command is issued.

## Agent-level tier: `mutating-runtime` (Phase B live-guards)

The skill-side `liveAgentFields.execution_tier` enum covers
`static-review`, `read-only-runtime`, and `sandbox-mutating`. At the
**agent** level, `execution_tier` additionally allows `mutating-runtime`
— the tier for **live-guard agents that perform a real, controlled
mutation against a production system**. This is the safe, gated
realization of an operation that T3 prohibits for *ungated* agents: it
is never performed implicitly, never in bulk, and always reversible.

**Posture.** Exactly **one** narrow, reversible operation per
invocation against a live system. No bulk, no wildcard, no
DELETE/DROP, no ownership transfer, no privilege escalation.

**Gate-only.** The maestro **never auto-dispatches** a `mutating-runtime`
agent. Its `*-live-*-guard` naming forces gate-only classification.
Every mutation requires, in order:

1. **Explicit written human approval token** naming the exact target
   (e.g. table + record GUID, driveItem + label ID, securable +
   privilege + principal) and a blast-radius statement.
2. **PREFLIGHT dry-run diff** — capture current state (GET / `SHOW
   GRANTS` / `SHOW`) and show the proposed change before any write.
3. **Idempotency key** generated before the write, recorded in the
   audit log, and used to detect replay.
4. **Signed attestation** (`signed_with: idempotency-key`) referencing
   the approval token, idempotency key, statement executed, and prior
   state.
5. **ROLLBACK** with prior-state capture, a named inverse operation, a
   named human owner, and a time-box (e.g. 30 minutes).

**Least privilege.** The run-as principal is granted only the single
write needed, with an explicit **denied list** for broad/admin scopes.
`requires_credentials` lists **environment-variable names only — never
secret values**. Each guard ships `PERMISSIONS.md`, `PREFLIGHT.md`, and
`ROLLBACK.md` alongside `AGENT.md`.

**Examples in this repo.**
- `d365-live-record-field-update-guard-agent` — PATCH named fields on
  one Dataverse row (data plane); inverse-PATCH rollback.
- `m365-live-sensitivity-label-apply-guard-agent` — `assignSensitivityLabel`
  on one driveItem (Graph); re-apply-prior-label rollback.
- `databricks-live-unity-catalog-grant-guard-at-azure-agent` — one
  schema-scoped Unity Catalog `GRANT`; `REVOKE` rollback.
- `snowflake-live-rbac-grant-guard-at-azure-agent` — one RBAC `GRANT`
  to a custom role; `REVOKE` rollback.

> Phase A (`read-only-runtime`) live-guards discover and propose;
> Phase B (`mutating-runtime`) live-guards execute one approved,
> reversible change. Both are gate-only.

## Declaration Contract

Every skill in this marketplace SHOULD declare its tier explicitly:

```yaml
---
name: salesforce-soql-explorer-skill
description: |
  ...
allowed-tools: Bash(sf data query:*) Bash(sf org list:*) Bash(sf org display:*) Read Grep Glob
metadata:
  version: "0.1.0"
  execution_tier: read-only-runtime
  oauth_scopes: ["api", "refresh_token"]
  mcp_servers: []
  run_as_permissions:
    required: ["View Setup and Configuration"]
    denied: ["ModifyAllData", "ViewAllData", "ViewEncryptedData", "ModifyMetadata"]
---
```

Skills without an `execution_tier` declaration are treated as `static-review`
(T0) by default. This is safe because T0 has zero blast radius.

## Schema References

- `schemas/skill.frontmatter.schema.json` — `$defs.liveAgentFields`
- `schemas/skill.schema.json` — accepts the same fields at the catalog
  level via `additionalProperties: true`

## Enforcement Roadmap

Wave 4 ships the **declaration model**. Wave 5+ will ship harness-side
enforcement:
- Pre-tool-use hooks that block Bash commands outside the declared
  allowlist
- Validators that cross-check `oauth_scopes` against documented
  least-privilege baselines per provider
- Audit log emission requirements per tier
- Live Guard integration for T2 → T1 → T0 demotion (refusal cascade)
