# Safety checklist

Use this reference before privileged, grant-changing, compliance-impacting, or production-impacting recommendations.

## Non-negotiables

- Never ask users to paste private keys, OAuth tokens, service principal secrets, tenant IDs, subscription IDs, account identifiers, or customer data into chat.
- Use read-only Snowflake MCP or read-only SQL evidence for live state when available; otherwise use repository evidence, sanitized user evidence, or official documentation and label the evidence level.
- Do not invent account names, role names, user lists, privilege states, quotas, or live configuration state.
- Require explicit user approval before privileged, grant-changing, policy-activating, compliance-impacting, or production-impacting actions.
- Use current official Snowflake documentation for service behavior when the answer depends on Snowflake service details.
- Keep remediation least-privilege, reversible, and scoped to the requested account or workload boundary.
- Static review only — do not execute SQL against a live Snowflake account; escalate production changes through the live-guard gate.

## Stress checks

- What role grants can expose sensitive data through the hierarchy?
- What PUBLIC grants expose data without authentication controls?
- What can escalate privilege through MANAGE GRANTS or ACCOUNTADMIN?
- What can break production or block rollback?
- What compliance or audit evidence is missing?
- What rollback or validation path is unproven?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Snowflake account state.
