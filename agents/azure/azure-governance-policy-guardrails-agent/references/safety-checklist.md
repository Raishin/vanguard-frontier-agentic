# Safety checklist

Use before Azure Policy governance guardrails recommendations that affect access, cost, network exposure, data, compliance, production availability, or automation.

## Non-negotiables

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for Azure service behavior.
- Use read-only configured-environment evidence only when available and label it as sampled evidence.
- Never ask for credentials, tokens, tenant IDs, subscription IDs, connection strings, certificates, private keys, or customer data.
- Require explicit approval before recommending or executing mutations, deletes, privilege changes, secret-bearing reads, or production-impacting operations.
- State what is unknown; documentation proves service behavior, not the user's deployed state.
- Keep action/tool permissions least-privilege and scoped to the task.
- Require rollback or disablement path for production-impacting recommendations.
- Verify owner, scope, and evidence label before presenting a go/no-go verdict.

## Component risks

- **Identity and access:** overbroad roles, standing privilege, unsafe exclusions, long-lived secrets, and unverified licensing.
- **Network and data exposure:** public access, private DNS gaps, unclassified exports, excessive logs, and unreviewed retention.
- **Cost and capacity:** false precision, hidden dependencies, commitment lock-in, overprovisioning, and unowned recommendations.
- **Governance and automation:** broad assignment scope, remediation side effects, missing rollback, and stale exception ownership.
- **Operational readiness:** missing alerts, untested failover/restore, absent runbooks, and unsupported environment assumptions.

## Evidence labels

Use `sampled evidence`, `repo evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Azure environment.
