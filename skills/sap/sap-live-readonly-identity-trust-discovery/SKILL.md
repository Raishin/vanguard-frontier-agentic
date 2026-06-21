---
name: sap-live-readonly-identity-trust-discovery
description: Inspect SAP Cloud Identity Services (IAS/IPS), BTP trust and federation configuration, XSUAA role collections, and identity provider settings using read-only list, get, describe, and export operations only. Use when read-only discovery of IAS application assignments, IPS connector configuration, XSUAA role collection assignments, corporate identity provider federation, or BTP trust configurations is needed as evidence for advisory, audit, or compliance purposes. Requires pre-authorized read-only credentials. Never creates, updates, deletes, assigns, rotates, modifies trust, or triggers any mutation.
allowed-tools: Read Grep Glob WebSearch WebFetch Bash
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-19"
  category: security
  lifecycle: experimental
  execution_tier: read-only-runtime
---

# SAP Live Read-Only Identity and Trust Discovery

## Purpose

Enumerate SAP Cloud Identity Services (IAS and IPS), BTP trust and federation configuration, XSUAA role collections, and identity provider settings using strictly read-only operations to produce structured evidence for advisory, audit, compliance, or security review purposes. Every action taken by this skill must appear on the allowed-actions list below. This skill never changes state, never modifies trust configuration, and never assigns or revokes any identity or role.

## When to use

Use this skill when the user needs to:

- list IAS applications and their assigned identity providers, risk-based authentication policies, and MFA enforcement settings,
- enumerate IPS connectors (source and target systems), transformation scripts, and provisioning job history in read-only mode,
- list and describe XSUAA role collections, their role template assignments, and user or group assignments within a BTP subaccount,
- export BTP trust configurations (trusted identity providers, federation attributes, principal propagation settings) as structured audit evidence,
- enumerate corporate identity provider federation settings in IAS: SAML 2.0 or OIDC trust metadata, allowed redirect URIs, and attribute mapping,
- gather identity and trust facts needed as input to `sap-security-iam-grc-sod-review` or as evidence for a compliance audit,
- confirm read-only identity posture for zero-trust assessment without modifying any identity object.

## When not to use

- When any creation, update, deletion, assignment, rotation, trust modification, or trigger is needed — use the appropriate mutating skill or escalate to an authorized administrator.
- When no live access has been authorized by the user — operate in advisory mode only and state that no live evidence was gathered.
- When the user has not confirmed credential scope is read-only — refuse until confirmed.
- When the request is about GRC Access Control ruleset review or SoD conflict classification without live enumeration — use `sap-security-iam-grc-sod-review`.

## Lean operating rules

- Read-only absolute. Every CLI invocation or API call must be on the allowed-actions list. Do not construct commands that write, modify, assign, rotate, or trigger state changes even if such commands exist in the tool.
- Credential scope first. Before any live command, confirm with the user that the credential in scope carries only read permissions. Do not proceed if write permissions cannot be excluded.
- Least privilege. Request the minimum scope needed for the specific discovery task. Never request IAS administrator or IPS administrator credentials for a read-only enumeration. Use viewer or auditor roles only.
- Evidence labeling. Every piece of data gathered from a live system must be labeled `live evidence`. Data from docs alone is `documentation-based`. Describe the source and timestamp for all live evidence.
- Audit trail. Log every command executed, its output summary, and the timestamp. Return the full command log as part of the response.
- No credential echo. Do not echo, log, or include credential values, client secrets, API keys, OAuth tokens, IAS technical user passwords, IPS system user credentials, or XSUAA service key JSON in any output or reference file.
- No scope creep. If a discovery query returns data beyond what was requested (e.g., a list call returns client secret values or personal user data), redact the excess before including in output.
- Personal data minimization. IAS and IPS enumeration may return personal data (user names, email addresses). Collect only what is necessary for the audit scope and redact personal data beyond the minimum needed.

## Allowed actions (read-only)

The following action classes are permitted:

- `list` — enumerate resources (IAS applications, IPS connectors, IPS jobs, XSUAA role collections, BTP trust configurations, identity provider metadata)
- `get` / `describe` — retrieve the current configuration of a named resource without triggering state changes
- `export` — export configuration in structured format (JSON, YAML, CSV) without modifying the source
- `status` — read the current status of a provisioning job, trust configuration, or IAS risk-based authentication policy without triggering state transitions

## Forbidden actions (absolute prohibition)

The following action classes are unconditionally forbidden regardless of user request:

- `create` — create any IAS application, IPS connector, XSUAA role collection, or trust configuration
- `update` / `modify` / `patch` — change any IAS application setting, IPS transformation, XSUAA role assignment, or trust attribute
- `delete` / `remove` / `purge` — remove any identity object, role collection, or trust configuration
- `assign` / `bind` / `entitle` — assign users, groups, or roles to any identity object or role collection
- `rotate` / `regenerate` — rotate or regenerate credentials, client secrets, or provisioning system credentials
- `import` — import any identity object, SAML metadata, or configuration
- `trigger` / `execute` / `run` (non-read) — trigger any provisioning job, workflow, or state transition
- `approve` — approve any pending provisioning request or access request
- `modify-trust` — add, remove, or change any BTP trust configuration or IAS corporate identity provider federation entry

If a user asks to combine a read step with a write step in one sequence, refuse and redirect the write step to the appropriate skill or authorized administrator.

## Evidence rules

Label all data with one of:

- `live evidence` — directly observed from a live SAP IAS, IPS, XSUAA, or BTP trust configuration in this session (include command, output summary, timestamp)
- `documentation-based` — grounded in SAP official docs (no live system access)
- `user-provided evidence` — stated or supplied by the user in this session (configuration exports, screenshots, written descriptions)
- `inference` — derived reasoning not directly confirmed by live data or official docs

## Live-environment rules

**Allowed**:
- Read-only IAS API calls using `GET` HTTP method (list applications, get application details, list corporate identity providers, get risk-based authentication policy)
- Read-only IPS API calls using `GET` HTTP method (list source and target systems, get connector configuration, list transformation scripts, get provisioning job history)
- Read-only BTP CLI commands for trust enumeration (`btp list security/trust`, `btp get security/trust`)
- Read-only XSUAA service API calls using `GET` HTTP method (list role collections, get role collection details, list role templates, list scopes)
- Read-only BTP CLI commands for role collection enumeration (`btp list security/role-collection`, `btp get security/role-collection`)

**Forbidden**:
- Any API call using `POST`, `PUT`, `PATCH`, or `DELETE` HTTP methods against IAS, IPS, XSUAA, or BTP trust endpoints
- Any CLI command with a write, delete, create, deploy, assign, rotate, import, trigger, or modify-trust verb
- Any IPS provisioning job trigger via API or UI action
- Any IAS configuration change (application settings, risk-based authentication policy, MFA enforcement, corporate identity provider federation)
- Any XSUAA role collection creation, modification, or assignment

**Least-privilege credential rules**:
- IAS API: use an IAS administrator account scoped to read-only operations, or use a service user with `Manage Applications` set to read-only; never use a full IAS tenant administrator for enumeration
- IPS API: use an IPS administrator account with read-only access to connector configuration and job history; never use credentials with provisioning job execution rights
- XSUAA / BTP CLI: use a subaccount viewer or security auditor role only; never use a subaccount administrator or security administrator for discovery
- BTP trust endpoints: use Global Account Viewer or Subaccount Viewer role; never use Global Account Administrator

**Approval gate**:
Before executing any live command, confirm:
1. The user has authorized live read-only access for this session.
2. The credential in scope is confirmed read-only (viewer, auditor, or read-only API user role or equivalent).
3. The target system and scope are explicitly identified by the user (IAS tenant ID, IPS tenant, BTP subaccount ID, XSUAA service instance).

**Rollback**: This skill makes no changes; rollback is not applicable. However, if a command unexpectedly triggers a state change (e.g., an API bug or misconfigured tool), stop immediately and escalate to the user without continuing.

**Post-discovery**:
- Return a command log with all commands executed and a summary of evidence gathered.
- Redact all credential values, client secrets, passwords, personal user data beyond the minimum necessary, and sensitive tokens from output.
- Label all gathered data as `live evidence` with source command and timestamp.

**Audit evidence**:
- Every response that includes live evidence must include the full command log.
- Audit evidence format: `[COMMAND] [TIMESTAMP_UTC] [SYSTEM] [SUMMARY_OF_OUTPUT]`

## References

Load only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — discovery workflow, command patterns, output format.
- [Safety checklist](references/safety-checklist.md) — non-negotiables, forbidden action verification, credential rules.
- [Official sources](references/official-sources.md) — SAP IAS, IPS, XSUAA, BTP trust documentation.
- [Live environment access](references/live-environment-access.md) — credential setup, role requirements, audit log format, redaction rules.

## Response minimum

Return, at minimum:

- **Problem classification**: what identity and trust data is needed and why.
- **Evidence used**: live evidence (with command log) / documentation-based / user-provided / inference.
- **Risk level**: none — this skill is read-only.
- **Recommended action**: which list/get/describe commands to run and in what order.
- **Refusal / escalation triggers**: refuse if any requested action is on the forbidden list; escalate to the appropriate skill or an authorized administrator.
- **Business impact**: what security posture decisions depend on this identity and trust data; what is the cost of incomplete discovery.
- **Next verification step**: confirm credential scope with the user before the first live command.
