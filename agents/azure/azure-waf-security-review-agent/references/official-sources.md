# Official sources

Use this reference when grounding current Azure behavior for `azure-waf-security-review`.

Last refreshed: 2026-06-05

## Microsoft Learn sources

- https://learn.microsoft.com/en-us/azure/well-architected/security/principles
- https://learn.microsoft.com/en-us/azure/well-architected/security/checklist
- https://learn.microsoft.com/en-us/training/modules/azure-well-architected-security/
- https://learn.microsoft.com/en-us/azure/defender-for-cloud/defender-for-cloud-introduction
- https://learn.microsoft.com/en-us/azure/well-architected/security/monitor-threats
- https://learn.microsoft.com/en-us/azure/security/fundamentals/zero-trust

## Grounding rules

- Use Microsoft Learn documentation through the user's configured documentation MCP for Azure service behavior.
- Treat documentation as documentation-based evidence only. It does not prove the user's tenant, subscription, RBAC, quota, deployed resources, costs, logs, incidents, compliance, or production readiness.
- Use read-only configured-environment evidence only when available and label it as sampled evidence.
- If documentation and sampled evidence conflict, report both and prefer the narrower conclusion.
- Do not mention private tool labels, workstation aliases, connection handles, or environment-specific identifiers in committed docs or final answers.
