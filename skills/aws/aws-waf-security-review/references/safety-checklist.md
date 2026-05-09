# Safety Checklist

Use before recommending any IAM policy change, network modification, KMS key action, or production-impacting security control.

## Non-negotiables

- Never ask users to paste access keys, session tokens, account IDs (unless sanitized), private keys, or customer data.
- Do not invent IAM policies, ARNs, resource names, quotas, account IDs, or live configuration state.
- Require explicit user approval before changes to IAM policies, SCPs, GuardDuty suppression rules, Security Hub controls, KMS key policies, or VPC security groups.
- Use official AWS documentation for service behavior — not training memory for specific API behavior that may have changed.
- Keep all recommendations least-privilege, reversible, and scoped to the stated workload boundary.
- Always distinguish between Detective controls (GuardDuty, Config) and Preventive controls (SCPs, Service Control Policies, org policies) — they are complementary, not interchangeable.

## Stress checks

- What can expose data to unauthorized principals?
- What can allow privilege escalation (PassRole, sts:AssumeRole, iam:CreateAccessKey)?
- What can disable audit logging (CloudTrail, Config, GuardDuty)?
- What can bypass encryption requirements?
- What compliance evidence is missing for the stated audit framework?
