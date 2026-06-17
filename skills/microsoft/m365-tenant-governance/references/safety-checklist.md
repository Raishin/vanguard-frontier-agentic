# Safety checklist

Use this reference before any recommendation that changes admin role assignments, organization-wide settings, GDAP relationships, Secure Score improvement actions, or any other Microsoft 365 tenant-level configuration.

## Non-negotiables

- Never recommend assigning Global Administrator for tasks achievable with a least-privileged role. State this refusal plainly if pressed.
- Never ask users to paste secrets, admin credentials, tenant IDs, client secrets, certificates, private keys, or customer data into chat.
- Use read-only Microsoft 365 admin center evidence or Microsoft Graph read evidence for live state when available; otherwise use repository evidence, sanitized user evidence, or official documentation and label the evidence level.
- Do not invent admin role counts, GDAP relationship states, Secure Score values, or org-wide settings configurations.
- Require explicit user approval before recommending creation, modification, or removal of admin role assignments, GDAP relationships, or org-wide settings changes.
- Keep remediation least-privilege, reversible, staged, and scoped to the requested role or policy boundary.
- Treat any standing Global Administrator account used for day-to-day operations as a critical finding until scoped to emergency-only use.
- Treat any active legacy DAP relationship (blanket standing Global Administrator for partners) as critical until migrated to GDAP with task-scoped, time-bound roles.

## Stress checks

- How many Global Administrator accounts exist and which are justified for emergency-only use?
- Which partner GDAP or DAP relationships grant broader access than required for the stated task?
- Which Message Center advisory notices have been missed or not routed through a change review workflow?
- Which org-wide settings changes lack a documented change control record or rollback procedure?
- Which Secure Score improvement actions affect governance-critical controls and remain unaddressed?
- What cross-workload policy gap creates an inconsistency in data protection or compliance posture?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Microsoft 365 admin role inventory, active GDAP relationship scope, Secure Score posture, or org-wide settings configuration.

## Escalation triggers

Escalate to live-guard gate before any of the following:

- Assigning or removing Microsoft 365 or Microsoft Entra admin roles for any user
- Creating, modifying, or terminating GDAP relationships or delegated admin relationships
- Changing tenant-level organization-wide settings (external sharing, Teams meeting defaults, cross-tenant access)
- Implementing Secure Score improvement actions that modify tenant configuration
- Changing Message Center notification routing or administrative contact configuration
- Modifying multi-workload policy baselines that affect Exchange Online, SharePoint, Teams, or Microsoft Entra ID simultaneously
