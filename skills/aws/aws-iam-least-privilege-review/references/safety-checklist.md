# Safety checklist

Use this reference before privileged, destructive, traffic-changing, cost-changing, compliance-impacting, or production-impacting recommendations.

## Non-negotiables

- Never ask users to paste secrets, access keys, session tokens, private keys, customer identifiers, or sensitive account data into chat.
- Use read-only AWS MCP or read-only AWS CLI evidence for live state when available; otherwise use repository evidence, sanitized user evidence, or official documentation and label the evidence level.
- Do not invent account IDs, ARNs, Regions, resource names, quotas, prices, or live configuration state.
- Require explicit user approval before privileged, destructive, traffic-changing, cost-changing, or production-impacting actions.
- Use current official AWS documentation for service behavior when the answer depends on AWS service details.
- Keep remediation least-privilege, reversible, and scoped to the requested workload or account boundary.

## Stress checks

- What can expose data?
- What can escalate privilege?
- What can break production or block rollback?
- What can create unbounded cost?
- What compliance or audit evidence is missing?
- What rollback or validation path is unproven?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live AWS state.
