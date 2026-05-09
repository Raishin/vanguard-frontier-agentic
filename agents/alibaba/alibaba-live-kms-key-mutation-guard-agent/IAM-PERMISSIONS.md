# IAM Permissions — Alibaba Cloud Live KMS Key Mutation Guard

## Minimum Read Permissions (Discovery and Audit)

| Policy | Purpose |
|--------|---------|
| `AliyunKMSReadOnlyAccess` | Describe key metadata, status, and usage policies without mutation rights |

## Required Mutation Permissions (Live Operations)

| Policy | Purpose | Scope Guidance |
|--------|---------|---------------|
| `AliyunKMSFullAccess` | Schedule key deletion, disable, or enable KMS keys | Must confirm exact key ID before assuming this policy; never granted as standing access |

## Narrowing Guidance

- Confirm the exact key ID (not alias) before any mutation. Key aliases can be reassigned; key IDs are permanent identifiers.
- `AliyunKMSFullAccess` should be assumed via STS for specific approved operations only — never a standing policy attachment.
- Use RAM conditions to restrict KMS mutations to specific key IDs or regions where the RAM policy language permits.
- Prefer `AliyunKMSReadOnlyAccess` for all dependency audit and pre-flight read operations.

## Anti-Patterns — Never Grant

- `AdministratorAccess` — account-wide full control; never appropriate for a KMS guard agent.
- `AliyunKMSFullAccess` as a standing attachment to any RAM user or operational role.
- Long-lived RAM user access keys with `AliyunKMSFullAccess` — if compromised, the attacker can delete all CMKs.
- Granting `AliyunKMSFullAccess` to CI/CD pipeline service accounts — blast radius is all encrypted data in the account.
- Scheduling deletion with a window shorter than 30 days without explicit justification and CISO-equivalent approval.

## Audit Trail

All KMS mutations (key disable, enable, schedule deletion, cancel deletion) are logged in **ActionTrail** under the `kms` service. Ensure ActionTrail is enabled for the target region and delivering to OSS or SLS. Query events with `EventName` containing `ScheduleKeyDeletion`, `CancelKeyDeletion`, `DisableKey`, or `EnableKey`. Monitor for any unexpected key state transitions via CloudMonitor KMS key event alarms.
