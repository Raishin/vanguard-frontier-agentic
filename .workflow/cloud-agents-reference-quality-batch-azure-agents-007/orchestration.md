# Azure agents reference-quality batch 007 orchestration

Date: 2026-06-05
Provider: Azure
Asset type: agents
Batch size: exactly 5

## Targets

1. agents/azure/azure-role-selector-agent
2. agents/azure/azure-security-posture-hardening-agent
3. agents/azure/azure-subscription-resource-organization-agent
4. agents/azure/azure-waf-cost-optimization-review-agent
5. agents/azure/azure-waf-reliability-review-agent

## Evidence discipline

- Used Microsoft Learn documentation through the user configured documentation MCP for documented Azure service behavior.
- Treated documentation as documentation-based evidence only; it does not prove deployed tenant, subscription, RBAC, quotas, resources, costs, incidents, or production readiness.
- No AWS assets were intentionally changed.
- No committed docs should mention private tool labels, workstation aliases, connection handles, or environment-specific identifiers.

## AgentCore-style alignment

- Matched the AgentCore reference-pack pattern: component operations reference, official sources, safety checklist, workflow/output, and documentation evidence discipline.
- Operations references include the AgentCore-style sections: What people get wrong, Officially grounded service shape, Non-negotiable design rules, Minimal safe implementation flow, High-risk assumptions to kill, Safe command/code verification targets, and When to push back.
- Primary AGENT docs remain lean and route details to references.
