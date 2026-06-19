# Permissions — M365 Live Sensitivity Label Apply Guard

## Execution tier

`mutating-runtime` (Phase B). This agent applies a single sensitivity label to one driveItem via the Microsoft Graph `assignSensitivityLabel` action. No bulk labeling, label policy changes, or broad write permissions are permitted.

## Required Microsoft Graph application permissions

All permissions are **application permissions** (app-only, no delegated user context) and require **tenant admin consent**.

| Permission | Purpose | Scope note |
|---|---|---|
| `InformationProtectionPolicy.Read.All` | Read available sensitivity labels to verify the proposed label ID and read the current label on the target item | Application permission; admin-consented |
| `Files.ReadWrite.All` | Call `assignSensitivityLabel` on the ONE target driveItem | The **least-privileged APPLICATION permission** documented for this API ([permissions table](https://learn.microsoft.com/graph/api/driveitem-assignsensitivitylabel?view=graph-rest-1.0#permissions)). The higher-privileged alternative is `Sites.ReadWrite.All`. Graph exposes **no** per-item or `Sites.Selected` application permission for this protected API, and `Files.ReadWrite` (without `.All`) is delegated-only. Constrain blast radius outside the grant — see compensating controls below. |

> **Important**: The Graph `assignSensitivityLabel` API is a **metered, protected API**. It requires metered API setup (Azure subscription linked to the M365 tenant) in addition to the permission grants above. See https://learn.microsoft.com/graph/metered-api-setup for prerequisites.

> **Compensating controls (the permission floor is coarse)**: Because no per-item application scope exists for this API, restrict the app's effective reach *outside* the Graph permission — via an app-only access policy / resource-specific consent (RSC), or a `Sites.Selected` site-level grant where the tenant supports it — combined with this guard's one-item written-approval gate, PREFLIGHT diff, and idempotency-keyed attestation.

## Denied permissions (must NOT be granted to the application)

- `Directory.ReadWrite.All` — tenant-wide directory write; not permitted
- `Sites.FullControl.All` — full control over all sites; not permitted
- `Sites.ReadWrite.All` — higher-privileged alternative; `Files.ReadWrite.All` is the narrower documented permission for this API
- `InformationProtectionPolicy.ReadWrite.All` — label policy management write; not permitted
- `LabelPolicyManagement` (any scope) — label policy management; not permitted
- `RoleManagement.ReadWrite.Directory` — role management write; not permitted
- `User.ReadWrite.All` — user write; not permitted
- Any permission that enables bulk labeling, label policy changes, or label removal without re-applying another approved label

## Approval token requirement

Before any write, a **written human approval token** must be provided that explicitly names:

- The tenant (referenced by env-var name `GRAPH_TENANT_ID`, not by value)
- The drive ID and driveItem ID (exact identifiers, not a wildcard or search result)
- The proposed sensitivity label ID and label display name
- The assignment method (`standard` or `privileged`)
- Justification text (required when proposing a downgrade in classification tier or a `privileged` assignment)
- The blast-radius assessment (what access controls, encryption, or DLP policies change with the new label; who currently has access under the prior label)

An approval token that omits any of the above is rejected. For label downgrades, additional sign-off beyond the standard approval token is required.

## Credential posture

- **Preferred**: certificate credential on the Entra app registration.
- **Acceptable**: client secret with short rotation (90 days maximum). Secret values must never appear in repo, chat, or logs.
- **Forbidden**: broad admin credentials used as a shortcut, sharing credentials across agents, storing secret values in configuration files committed to source control.
- Credentials are referenced by environment variable name only: `GRAPH_CLIENT_ID`, `GRAPH_TENANT_ID`.

## Egress allow-list

- `graph.microsoft.com` — Microsoft Graph API endpoint
- `login.microsoftonline.com` — Microsoft Entra OAuth 2.0 token endpoint

No other egress destinations are required or permitted for this agent.

## Blast-radius boundary

This agent applies one sensitivity label to one driveItem. The blast radius is:

- **Direct**: the driveItem's sensitivity label changes, which may alter encryption settings, access rights, DLP policy enforcement, and visual markings applied by Office clients.
- **Encryption impact**: if the new label applies encryption, users without the corresponding Rights Management permissions may lose access to the file.
- **Downgrade risk**: lowering the classification tier may remove protection controls (encryption, DLP policies). This is a higher-risk operation requiring justification and additional sign-off.
- **Audit trail**: the label change is recorded in the Microsoft Purview compliance audit log.
- **Metered API cost**: each `assignSensitivityLabel` call incurs metered API usage charges.

Blast-radius must be assessed and documented in the approval token before any label write proceeds.
