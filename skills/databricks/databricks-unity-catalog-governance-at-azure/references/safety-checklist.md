# Safety checklist

Use this reference before privileged, destructive, compliance-impacting, or production-impacting recommendations.

## Non-negotiables

- Never ask users to paste access tokens, service principal secrets, client secrets, connection strings, storage account keys, or customer identifiers into chat.
- Use read-only workspace MCP or IaC repository evidence for live state when available; otherwise use sanitized user evidence or official documentation and label the evidence level.
- Do not invent metastore IDs, catalog names, workspace URLs, principal IDs, grant assignments, or live configuration state.
- Require explicit user approval before privileged, destructive, compliance-impacting, or production-impacting recommendations.
- Use current official Databricks and Microsoft Learn documentation for service behavior when the answer depends on platform details.
- Keep remediation least-privilege, reversible, and scoped to the requested workspace or catalog boundary.
- Static review only: never execute GRANT, REVOKE, CREATE, DROP, or ALTER against a live workspace. Production grant/role/policy/cluster changes are live-guard gated (escalate).

## Stress checks

- What grants can expose data beyond the intended consumer group?
- What admin role or account-level privilege can be escalated?
- What interactive-user pattern breaks production automation or compliance posture?
- What missing parent USE grant silently fails or silently expands access?
- What audit evidence is missing from system tables?
- What rollback or validation path is unproven?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Databricks workspace state, grant assignments, or metastore configuration.
