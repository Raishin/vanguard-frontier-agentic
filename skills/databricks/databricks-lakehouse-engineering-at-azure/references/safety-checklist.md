# Safety checklist

Use this reference before privileged, destructive, compliance-impacting, or production-impacting recommendations.

## Non-negotiables

- Never ask users to paste access tokens, service principal secrets, client secrets, connection strings, storage account keys, SAS tokens, or customer identifiers into chat.
- Use read-only workspace MCP or IaC repository evidence for live state when available; otherwise use sanitized user evidence or official documentation and label the evidence level.
- Do not invent cluster IDs, workspace URLs, storage account names, Access Connector resource IDs, external location paths, or live configuration state.
- Require explicit user approval before privileged, destructive, compliance-impacting, or production-impacting recommendations.
- Use current official Databricks and Microsoft Learn documentation for service behavior when the answer depends on platform details.
- Keep remediation least-privilege, reversible, and scoped to the requested workspace or storage boundary.
- Static review only: never execute cluster create/edit, storage credential create, external location create, or ADLS Gen2 configuration changes against live infrastructure. Production infrastructure changes are live-guard gated (escalate).

## Stress checks

- What can expose ADLS Gen2 data to unintended identities (misconfigured managed identity, open container ACL)?
- What deprecated credential passthrough pattern creates a compliance or security gap?
- What cluster access mode violation bypasses Unity Catalog enforcement?
- What AKV-backed secret scope misconfiguration leaks secrets or creates read-path failures?
- What VNet or Private Link gap allows data exfiltration or unintended public access?
- What rollback or validation path is unproven?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live cluster state, storage credential configuration, or network topology.
