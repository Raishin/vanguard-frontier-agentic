# Safety checklist

Use this reference before warehouse resizing, storage integration changes, masking policy deployment, or production-impacting recommendations.

## Non-negotiables

- Never ask users to paste storage account keys, SAS tokens, service principal secrets, subscription IDs, tenant IDs, connection strings, private keys, or customer data into chat.
- Use read-only Snowflake MCP or read-only SQL evidence for live state when available; otherwise use repository evidence, sanitized user evidence, or official documentation and label the evidence level.
- Do not invent account names, warehouse configurations, storage integration states, policy assignments, quotas, or live configuration state.
- Require explicit user approval before warehouse resizing, storage integration creation, masking policy attachment, row access policy attachment, or production-impacting actions.
- Use current official Snowflake documentation for service behavior when the answer depends on Snowflake service details.
- Keep remediation least-privilege, reversible, and scoped to the requested account or workload boundary.
- Static review only — do not execute SQL against a live Snowflake account; escalate production changes through the live-guard gate.

## Stress checks

- What can expose sensitive data through missing or misconfigured masking policies?
- What can expose data through missing or incorrect row access policies?
- What can create unbounded cost through oversized warehouses or missing AUTO_SUSPEND?
- What can break production or block rollback?
- What compliance or audit lineage evidence (ACCESS_HISTORY) is missing?
- What rollback or validation path is unproven?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Snowflake account state.
