# IAM Permissions — Alibaba Cloud Live RAM Policy Change Guard

## Minimum Read Permissions (Discovery and Audit)

| Policy | Purpose |
|--------|---------|
| `AliyunRAMReadOnlyAccess` | Read RAM users, groups, roles, policies, and STS token metadata without mutation rights |

## Required Mutation Permissions (Live Operations)

| Policy | Purpose | Scope Guidance |
|--------|---------|---------------|
| `AliyunRAMFullAccess` | Create, modify, or delete RAM policies, roles, and user attachments | Must be confirmed as required for the specific change; never granted as a standing role to operational identities |

## Narrowing Guidance

- `AliyunRAMFullAccess` should never be a standing permission for any RAM user, role, or CI/CD service account. It must be assumed via STS for specific approved changes only.
- For read-only policy audits, `AliyunRAMReadOnlyAccess` is sufficient — do not use `AliyunRAMFullAccess` for reads.
- Use RAM conditions to restrict mutations to specific RAM resource types where possible.
- Prefer STS-assumed roles with short TTL over long-lived RAM user access keys for operational access.
- Resource Directory Control Policy changes additionally require Resource Management admin permissions — confirm separate authorization.

## Anti-Patterns — Never Grant

- `AdministratorAccess` — account-wide full control; never appropriate for a RAM guard agent.
- `AliyunRAMFullAccess` as a standing attachment to any RAM user or operational role — this must be on-demand only.
- Long-lived RAM user access keys with `AliyunRAMFullAccess` attached — if compromised, the attacker can escalate to any privilege level.
- Granting `AliyunRAMFullAccess` to CI/CD pipeline service accounts — blast radius is the entire account's identity configuration.
- Policy changes that grant `AdministratorAccess` to any principal without explicit CISO-level approval.

## Audit Trail

All RAM mutations are logged in **ActionTrail** under the `ram` service. Ensure ActionTrail is enabled and delivering to OSS or SLS for the target account. Query events with `EventName` containing `CreatePolicy`, `DeletePolicy`, `AttachPolicyToRole`, `AttachPolicyToUser`, `CreateRole`, `DeleteRole`, or `SetDefaultPolicyVersion`. Export ActionTrail logs to SLS for long-term retention and anomaly detection.
