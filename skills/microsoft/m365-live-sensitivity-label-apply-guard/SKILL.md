---
name: m365-live-sensitivity-label-apply-guard
description: Mutating-runtime live-guard for applying ONE Microsoft Purview sensitivity label to ONE specified item (file/driveItem) via the Microsoft Graph assignSensitivityLabel action. Strictly scoped — one item, one label application. Requires explicit written human approval token referencing the exact item, proposed label, and blast-radius. PREFLIGHT reads the item's current label before any write. Fully reversible — prior label captured; re-apply prior label is the rollback. Gate-only; never auto-dispatched. Phase B mutating-runtime.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-17"
  category: compliance
  execution_tier: mutating-runtime
  mcp_servers: []
  oauth_scopes:
    - "InformationProtectionPolicy.Read.All"
    - "Files.ReadWrite (scoped to the single target driveItem — verify exact scope against Graph permissions reference)"
  run_as_permissions:
    required:
      - "InformationProtectionPolicy.Read.All — to read available sensitivity labels and verify the proposed label ID (application permission, admin-consented)"
      - "Files.ReadWrite or Sites.Selected scoped to the target drive/site only — to call assignSensitivityLabel on the one driveItem (verify exact minimum scope against Graph permissions reference)"
      - "Application permission, admin-consented — no delegated/user-context for background agent operations"
    denied:
      - "Directory.ReadWrite.All"
      - "Sites.FullControl.All"
      - "Files.ReadWrite.All (broad, tenant-wide — use Sites.Selected or drive-scoped permission instead)"
      - "InformationProtectionPolicy.ReadWrite.All (label policy management — not permitted)"
      - "LabelPolicyManagement (any scope)"
      - "RoleManagement.ReadWrite.Directory"
      - "User.ReadWrite.All"
      - "Bulk labeling (any operation targeting more than one item)"
      - "Label policy changes (any write to label policy resources)"
      - "Removing protection that would downgrade classification without explicit approval token"
  required_egress:
    - "graph.microsoft.com"
    - "login.microsoftonline.com"
  requires_credentials:
    - "GRAPH_CLIENT_ID"
    - "GRAPH_TENANT_ID"
  output_attestation:
    schema: "sensitivity-label-attestation-v1"
    signed_with: "idempotency-key"
    audit_log: "required"
  liveAgentFields:
    execution_tier: "mutating-runtime"
    single_op: true
    reversible: true
    requires_approval_token: true
    dry_run_preflight: true
    idempotency_key: true
    blast_radius_required: true
  companion_agents:
    - "m365-live-sensitivity-label-apply-guard-agent"
---

# M365 Live Sensitivity Label Apply Guard

## Purpose

Act as the live mutating-runtime Microsoft Purview sensitivity label application guard. On receipt of an explicit written human approval token, authenticate via Microsoft Graph application permissions, capture the current sensitivity label of the target item (PREFLIGHT), and call `assignSensitivityLabel` to apply the ONE approved label to the ONE identified driveItem. Emit a signed, idempotency-keyed attestation and update the audit log. Reverse path is always available — re-apply the prior label.

## When to use

- A single file or driveItem must have its sensitivity label upgraded or set as part of a compliance remediation
- The operation requires a human-approved, blast-radius-reviewed gate before any label write proceeds
- A prior compliance discovery identified a specific item that must be labeled to meet a data-classification policy
- Auditable, traceable label application is required for regulatory or governance purposes

## Gate-only classification

This skill id contains `-live-` and ends in `-guard`. The maestro MUST NOT auto-dispatch this skill. Invocation requires:

1. An explicit **written human approval token** that references:
   - The target tenant (by env-var name `GRAPH_TENANT_ID`, not value)
   - The target drive ID and driveItem ID (or equivalent item path)
   - The proposed sensitivity label ID and label display name
   - The assignment method (`standard` or `privileged`)
   - The justification text (required for label downgrades or privileged assignments)
   - The blast-radius assessment (who/what reads or is protected by the current label; what changes with the new label)
2. Completion of PREFLIGHT (GET current label, confirm item exists, confirm scope)
3. Prior-label capture before any write

## Strict-control contract

- `execution_tier: mutating-runtime`
- **EXACTLY ONE** driveItem is labeled per approved run. Target is identified by drive ID + driveItem ID.
- **ONE label application** — the PATCH/action body contains only the approved sensitivity label ID.
- **No bulk labeling** — no operation targeting more than one item.
- **No label policy changes** — `InformationProtectionPolicy.ReadWrite.All` and any label-policy write scope are explicitly denied.
- **No classification downgrade without explicit approval** — if the proposed label would lower the classification tier, the approval token must explicitly acknowledge the downgrade and its justification text must be present.
- **No irreversible label removal** — removing protection without re-applying another label requires additional sign-off.

## Critical permission note

The Graph `assignSensitivityLabel` API for driveItem is a **metered, protected API** that requires:
- Metered API setup (Azure subscription linked to the tenant)
- Admin consent for the required permissions
- The exact minimum permission scopes should be verified against the official Graph permissions reference at: https://learn.microsoft.com/graph/permissions-reference

The scope `Files.ReadWrite.All` grants tenant-wide file write access and is explicitly denied. Use `Sites.Selected` (application permission, admin-consented, scoped to the target site) combined with `InformationProtectionPolicy.Read.All` where possible, or the narrowest available driveItem-labeling scope. Verify exact scope identifiers against the permissions reference before deployment.

## Credential posture

- App registration: use a certificate credential or managed identity — never a long-lived client secret.
- Credentials are referenced by environment variable name only (`GRAPH_CLIENT_ID`, `GRAPH_TENANT_ID`). Never print, echo, or log credential values.
- The app registration must have admin consent granted for all required application permissions before this skill runs.

## PREFLIGHT (dry-run) requirements

Before issuing the `assignSensitivityLabel` action:

1. Authenticate and confirm the application user identity and consented permissions.
2. Perform a GET on the target driveItem to retrieve the CURRENT sensitivity label:

   ```http
   GET https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}?$select=id,name,sensitivityLabel
   Authorization: Bearer <token>
   ```

3. Confirm the item exists (200 OK). If 404, stop — the item does not exist.
4. Emit the current label (id, display name, assignment method) vs the proposed label.
5. If the proposed label would lower the classification tier, flag this explicitly and confirm justification text is present in the approval token.
6. Present the current vs proposed label to the approver for final confirmation before writing.

## Write operation

```http
POST https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}/assignSensitivityLabel
Authorization: Bearer <token>
Content-Type: application/json

{
  "sensitivityLabelId": "<approved-label-id>",
  "assignmentMethod": "standard",
  "justificationText": "<justification from approval token>"
}
```

Note: `assignSensitivityLabel` is an async action on the Graph API. The response is a long-running operation. Poll the operation status URL until completion before recording the attestation result.

## Rollback path

- Prior sensitivity label ID captured in PREFLIGHT GET must be retained.
- Rollback = call `assignSensitivityLabel` again on the same item with the prior label ID (re-apply prior label).
- See ROLLBACK.md for owner, time-box, and verification steps.

## Output attestation

Every completed run must emit:

- Idempotency key (generated before the write; used to detect replay)
- Record of: tenant (env-var reference), drive ID, driveItem ID, item name, prior label ID + name, new label ID + name, assignment method, justification text, approval token reference
- Audit log entry written before and after the write
- Operation result: success (operation completed) or failure with error detail

## Lean operating rules

- Prefer Microsoft Learn documentation through the configured documentation MCP for Microsoft Graph and Microsoft Purview service behavior.
- Use live Graph API evidence; label it as live configured-environment evidence.
- Never request or accept credential values — env-var names only.
- If the request implies bulk labeling, label policy changes, or label removal without re-application — refuse and explain why this skill cannot perform that operation.
- State what is unknown; documentation proves service behavior, not the environment's deployed state.

## Refuse conditions

Immediately refuse and do not proceed if:

- More than one item ID is specified
- A wildcard, filter, or query targeting multiple items is used
- A label policy write operation is requested
- The approval token does not contain a justification text for a downgrade operation
- No written approval token is provided
- The approval token does not reference the exact drive ID + driveItem ID + label ID
- The proposed label would remove all protection (downgrade to unlabeled) without explicit additional sign-off

## Official sources

- https://learn.microsoft.com/graph/api/driveitem-assignsensitivitylabel?view=graph-rest-1.0
- https://learn.microsoft.com/graph/permissions-reference
- https://learn.microsoft.com/microsoft-365/compliance/sensitivity-labels
- https://learn.microsoft.com/graph/metered-api-overview
- https://learn.microsoft.com/entra/identity-platform/app-only-access-primer
