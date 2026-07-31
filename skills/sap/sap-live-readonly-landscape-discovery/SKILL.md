---
name: sap-live-readonly-landscape-discovery
description: Enumerate and export SAP BTP and S/4HANA landscape configuration using read-only list, get, describe, and export operations only. Use when read-only discovery of subaccounts, service instances, entitlements, destinations, ABAP system landscape objects, or transport routes is needed as evidence for advisory, audit, or compliance purposes. Requires pre-authorized read-only credentials. Never creates, updates, deletes, deploys, assigns, rotates, imports, or triggers any change.
allowed-tools: Read Grep Glob WebSearch WebFetch Bash
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-19"
  category: platform
  lifecycle: experimental
  execution_tier: read-only-runtime
---

# SAP Live Read-Only Landscape Discovery

## Purpose

Enumerate SAP BTP and S/4HANA landscape configuration using strictly read-only operations to produce structured evidence for advisory, audit, compliance, or architecture review purposes. Every action taken by this skill must appear on the allowed-actions list below. This skill never changes state.

## When to use

Use this skill when the user needs to:

- list BTP subaccounts, directories, entitlements, service instances, service bindings, or destinations,
- describe ABAP system landscape objects (RFC destinations, transport routes, system definitions) in read-only mode,
- export landscape configuration as structured evidence for a clean core review, transport planning, compliance audit, or architecture assessment,
- enumerate trust configurations, identity providers, or role collections in read-only mode,
- gather landscape facts needed as input to `sap-clean-core-debt-review` or pre-flight facts for `sap-guarded-transport-import`.

## When not to use

- When any creation, update, deletion, deployment, assignment, rotation, import, or trigger is needed — use `sap-guarded-transport-import` for transport-scoped mutations with its full 17-step guarded sequence.
- When no live access has been authorized by the user — operate in advisory mode only and state that no live evidence was gathered.
- When the user has not confirmed credential scope is read-only — refuse until confirmed.

## Lean operating rules

- Read-only absolute. Every CLI invocation must be on the allowed-actions list. Do not construct commands that write, modify, or trigger state changes even if such commands exist in the tool.
- Credential scope first. Before any live command, confirm with the user that the credential in scope carries only read permissions. Do not proceed if write permissions cannot be excluded.
- Least privilege. Request the minimum scope needed for the specific discovery task. Never request administrator-level credentials for a read-only enumeration.
- Evidence labeling. Every piece of data gathered from a live system must be labeled `live evidence`. Data from docs alone is `documentation-based`. Describe the source and timestamp for all live evidence.
- Audit trail. Log every command executed, its output summary, and the timestamp. Return the full command log as part of the response.
- No credential echo. Do not echo, log, or include credential values, client secrets, API keys, OAuth tokens, or service key JSON in any output or reference file.
- No scope creep. If a discovery query returns data beyond what was requested (e.g., a list call returns service key values), redact the excess before including in output.

## Allowed actions (read-only)

The following action classes are permitted:

- `list` — enumerate resources (subaccounts, entitlements, instances, destinations, routes, roles, identity providers)
- `get` / `describe` — retrieve the current configuration of a named resource
- `export` — export configuration in structured format (JSON, YAML, CSV) without modifying the source
- `status` — read the current status of a resource without triggering state transitions

## Forbidden actions (absolute prohibition)

The following action classes are unconditionally forbidden regardless of user request:

- `create` — create any resource
- `update` / `modify` / `patch` — change any resource configuration
- `delete` / `remove` / `purge` — remove any resource
- `deploy` / `push` / `apply` — deploy any artifact
- `assign` / `bind` / `entitle` — assign entitlements, roles, or service bindings
- `rotate` / `regenerate` — rotate or regenerate credentials
- `import` — import any transport request, configuration, or object
- `trigger` / `execute` / `run` (non-read) — trigger any workflow, job, or state transition
- `approve` — approve any pending request or transport

If a user asks to combine a read step with a write step in one sequence, refuse and redirect the write step to the appropriate skill.

## Evidence rules

Label all data with one of:

- `live evidence` — directly observed from a live SAP system in this session (include command, output summary, timestamp)
- `documentation-based` — grounded in SAP official docs (no live system access)
- `user-provided evidence` — stated or supplied by the user in this session
- `inference` — derived reasoning not directly confirmed by live data or official docs

## Live-environment rules

**Allowed**:
- Read-only BTP CLI commands (`btp list`, `btp get`, `btp export`)
- Read-only CF CLI commands (`cf apps`, `cf services`, `cf service-instances`, `cf env` with redaction of secret values)
- Read-only Kyma CLI or kubectl read operations (`kubectl get`, `kubectl describe`) on authorized clusters
- Read-only ABAP RFC/HTTP calls that enumerate landscape objects (system list, transport routes, RFC destinations) — only via pre-authorized read RFC user

**Forbidden**:
- Any CLI command with a write, delete, create, deploy, assign, rotate, import, or trigger verb
- Any ABAP call that changes table data, creates objects, or triggers background jobs
- Any BTP API call using `POST`, `PUT`, `PATCH`, or `DELETE` HTTP methods

**Least-privilege credential rules**:
- BTP CLI: use a subaccount viewer or global account viewer role only
- CF: use SpaceAuditor or OrgAuditor role; never SpaceDeveloper or OrgManager for discovery
- Kyma/kubectl: use a read-only ClusterRole or namespaced Role bound to `get`, `list`, `watch` verbs only
- ABAP: use a display-only user (authorization objects S_TCODE limited to display transactions, no S_DEVELOP change authorization)

**Approval gate**:
Before executing any live command, confirm:
1. The user has authorized live read-only access for this session.
2. The credential in scope is confirmed read-only (viewer/auditor role or equivalent).
3. The target system and scope are explicitly identified by the user.

**Rollback**: This skill makes no changes; rollback is not applicable. However, if a command unexpectedly triggers a state change (e.g., a CLI bug or misconfigured tool), stop immediately and escalate to the user without continuing.

**Post-change / post-discovery**:
- Return a command log with all commands executed and a summary of evidence gathered.
- Redact any credential values, service keys, or sensitive tokens from output.
- Label all gathered data as `live evidence` with source command and timestamp.

**Audit evidence**:
- Every response that includes live evidence must include the full command log.
- Audit evidence format: `[COMMAND] [TIMESTAMP_UTC] [SYSTEM] [SUMMARY_OF_OUTPUT]`

## References

Load only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — discovery workflow, command patterns, output format.
- [Safety checklist](references/safety-checklist.md) — non-negotiables, forbidden action verification, credential rules.
- [Official sources](references/official-sources.md) — SAP BTP CLI, CF CLI, Kyma, ABAP landscape docs.
- [Live environment access](references/live-environment-access.md) — credential setup, role requirements, audit log format, redaction rules.

## Response minimum

Return, at minimum:

- **Problem classification**: what landscape data is needed and why.
- **Evidence used**: live evidence (with command log) / documentation-based / user-provided / inference.
- **Risk level**: none — this skill is read-only.
- **Recommended action**: which list/get/describe commands to run and in what order.
- **Refusal / escalation triggers**: refuse if any requested action is on the forbidden list; escalate to the appropriate skill.
- **Business impact**: what decisions depend on this landscape data; what is the cost of incomplete discovery.
- **Next verification step**: confirm credential scope with the user before the first live command.
