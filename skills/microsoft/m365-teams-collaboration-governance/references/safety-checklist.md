# Safety checklist

Use this reference before any recommendation that changes Teams external access settings, guest sharing policies, sensitivity label publishing policies affecting Teams, meeting policies, app permission policies, phone system configuration, or information barrier policies.

## Non-negotiables

- Never recommend weakening tenant-wide external access or guest sharing policies for convenience, deadline pressure, or broad exceptions without compensating controls and access review cadence. State this refusal plainly.
- Never ask users to paste secrets, admin credentials, tenant IDs, client secrets, certificates, private keys, or customer data into chat.
- Use read-only Teams admin center evidence or Microsoft Graph read evidence for live state when available; otherwise use repository evidence, sanitized user evidence, or official documentation and label the evidence level.
- Do not invent Teams external access settings, expiration policy coverage, guest access states, sensitivity label deployment, meeting policy configurations, or app permission policy assignments.
- Require explicit user approval before recommending tenant-wide external access changes, sensitivity label publishing policy changes affecting Teams, meeting policy changes with security implications, or app permission policy modifications.
- Keep remediation least-privilege, reversible, staged (pilot group before org-wide), and scoped to the requested Teams policy boundary.
- Treat any tenant with guest access enabled but no expiration policies or access review cadence as medium-high risk for guest sprawl.
- Treat any sensitive team (finance, legal, executive, HR) without a sensitivity label enforcing privacy and external user access control as a governance gap.

## Stress checks

- What path allows a guest or external user to access sensitive team content after their business relationship ends, with no expiration or review forcing removal?
- What Teams sprawl or ownerless team contains sensitive data with no active owner to approve access or apply sensitivity labels?
- What sensitivity label gap allows a sensitive team to be created as public or with unrestricted external sharing?
- What third-party app permission policy allows an untrusted app to read team messages or files without explicit review?
- What meeting policy gap allows external participants to record, bypass the lobby, or access meeting content without appropriate controls?
- What rollback path exists if a tenant-wide external access change disrupts existing B2B partner collaboration or federated calling?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Teams external access settings, expiration policy enforcement, sensitivity label deployment on teams, or app permission policy state.

## Escalation triggers

Escalate to live-guard gate before any of the following:

- Changing tenant-wide external access or federation settings (allow/block domains, per-user settings)
- Publishing or modifying sensitivity label policies that affect Teams, Microsoft 365 groups, or meeting labels
- Changing meeting policy settings with broad security implications (lobby bypass, recording, end-to-end encryption)
- Modifying org-wide app settings or app permission policies affecting third-party app access
- Enabling, modifying, or disabling information barrier policies
- Changing phone system emergency calling configurations or voice routing policies
- Enabling or disabling Microsoft Purview communication compliance or insider risk management integration with Teams
