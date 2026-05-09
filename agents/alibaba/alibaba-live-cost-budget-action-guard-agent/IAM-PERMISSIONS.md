# IAM Permissions — Alibaba Cloud Live Cost Budget Action Guard

## Minimum Read Permissions (Discovery and Audit)

| Policy | Purpose |
|--------|---------|
| `AliyunBSSReadOnlyAccess` | Read billing bills, budget configurations, Savings Plan coverage, and RI utilization without mutation rights |

## Required Mutation Permissions (Live Operations)

| Policy | Purpose | Scope Guidance |
|--------|---------|---------------|
| `AliyunBSSFullAccess` | Modify budget thresholds, purchase Savings Plans, or purchase Reserved Instances | Must confirm exact action, term, and cost model before assuming this policy; never granted as standing access |

## Narrowing Guidance

- Confirm the exact budget name, current spend, and proposed threshold before any budget mutation.
- `AliyunBSSFullAccess` should be assumed via STS for specific approved operations only — never a standing policy attachment.
- Use RAM conditions to restrict BSS mutations to specific account sub-users or projects where the RAM policy language permits.
- Prefer `AliyunBSSReadOnlyAccess` for all spend analysis, coverage review, and pre-purchase modeling.

## Anti-Patterns — Never Grant

- `AdministratorAccess` — account-wide full control; never appropriate for a cost guard agent.
- `AliyunBSSFullAccess` as a standing attachment to any RAM user or operational role.
- Long-lived RAM user access keys with `AliyunBSSFullAccess` — if compromised, the attacker can purchase committed spend contracts.
- Granting `AliyunBSSFullAccess` to CI/CD pipeline service accounts — Savings Plan purchases are non-refundable contracts.
- Lowering budget thresholds without modeling current spend — risk of immediate service suspension.

## Audit Trail

All BSS financial mutations (budget changes, Savings Plan purchases, RI purchases) are logged in **ActionTrail** under the `bss` service. Ensure ActionTrail is enabled for the account root and delivering to OSS or SLS. Query events with `EventName` containing `ModifyBudget`, `CreateSavingsPlan`, or `CreateReservedInstance`. Monitor budget utilization via BSS budget alert notifications.
