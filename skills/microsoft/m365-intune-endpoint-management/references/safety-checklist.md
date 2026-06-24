# Safety checklist

Use this reference before any recommendation that changes Intune compliance policies, Conditional Access device-compliance requirements, app protection policies, update ring enforcement, security baselines, or triggers any device action (wipe, retire, sync, restart).

## Non-negotiables

- Never recommend weakening device compliance requirements, adding broad noncompliance exceptions, or disabling Conditional Access device-compliance enforcement for convenience, deadline pressure, or VIP exceptions. State this refusal plainly.
- Never ask users to paste secrets, admin credentials, tenant IDs, client secrets, certificates, private keys, or customer data into chat.
- Use read-only Intune admin center evidence or Microsoft Graph read evidence for live state when available; otherwise use repository evidence, sanitized user evidence, or official documentation and label the evidence level.
- Do not invent Intune enrollment coverage, compliance policy assignments, app protection policy states, update ring assignments, or Defender for Endpoint integration status.
- Require explicit user approval before recommending compliance policy creation or modification, Conditional Access changes that affect device-compliance enforcement, update ring enforcement changes, security baseline deployments, or any device action such as wipe or retire.
- Keep remediation least-privilege, reversible, staged (audit or report mode before enforcement), and scoped to the requested device group or policy boundary.
- Treat any corporate resource accessible from an unmanaged or unenrolled device without app protection policies as high risk until protected.
- Treat any compliance policy assignment that is not enforced by Conditional Access as a gap — compliance without enforcement is advisory only.

## Stress checks

- What device path allows an attacker with stolen credentials to reach corporate resources from an unmanaged or noncompliant device?
- What app protection gap allows corporate data to exit the managed app boundary to personal storage or unmanaged apps?
- What update ring gap leaves endpoints exposed to known CVEs beyond the approved deferral window?
- What compliance policy is defined and assigned but not enforced by Conditional Access?
- What security baseline conflict or missing configuration profile leaves a known security setting unconfigured?
- What rollback path exists if a compliance policy or Conditional Access change causes a broad device lockout or service account disruption?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Intune enrollment state, compliance policy assignments, app protection policy coverage, or update ring enforcement.

## Escalation triggers

Escalate to live-guard gate before any of the following:

- Creating or modifying Intune compliance policies affecting Conditional Access enforcement
- Changing Conditional Access policies that reference device compliance or app protection requirements
- Modifying or deploying endpoint security baselines to production device groups
- Changing Windows update ring deferral periods or deadline enforcement on production ring assignments
- Triggering device actions (wipe, retire, fresh start, remote lock) on production devices
- Enabling or disabling Microsoft Defender for Endpoint integration with Intune
- Changing Windows Autopilot deployment profiles or enrollment status page configuration for production device groups
